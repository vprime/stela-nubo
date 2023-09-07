mod generation;
mod player;
mod util;
mod weapon;
mod input;
mod application;
mod destructible;

// Entrypoint for the main game binary
use bevy::{
    prelude::*,
    reflect::{
        TypeUuid, TypePath
    },
    render::{
        mesh::VertexAttributeValues,
        render_resource::*,
    },
};
use bevy::reflect::List;
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::{
    prelude::*,
    Actionlike,
    InputManagerBundle,
    action_state::ActionState
};
use itertools::Itertools;
use generation::*;
use crate::application::ApplicationPlugin;
use crate::destructible::DestructiblesPlugin;
use crate::input::{InputPlugin, PlayerAction};
use crate::player::*;
use crate::weapon::{WeaponBundle, WeaponOptions, WeaponPlugin};


const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PlayerControllerPlugin,
            MapGenerationPlugin,
            InputPlugin,
            ApplicationPlugin,
            WeaponPlugin,
            DestructiblesPlugin,
            MaterialPlugin::<ParticlesMaterial>::default()
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Gravity(Vec3::ZERO))
        .add_systems(Startup, (setup, setup_particles))
        .add_systems(Update, update_time_for_particles_material)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let player_spaceship = assets.load("models/player-ship/makoi.glb#Scene0");

    let mut input_map = InputMap::default();
    for action in PlayerAction::variants() {
        input_map.insert(PlayerAction::default_keyboard_mouse_input(action), action);
    }

    // player
    let player_id = commands.spawn((
        SceneBundle {
            scene: player_spaceship,
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Friction::new(0.4),
        ExternalForce::default(),
        ExternalTorque::default(),
        LinearVelocity::default(),
        AngularVelocity::default(),
        Player,
        InputManagerBundle::<PlayerAction>{
            action_state: ActionState::default(),
            input_map: input_map.build()
        },
        PlayerInput::default(),
        WeaponBundle {
            options: WeaponOptions {
                rate: 0.1,
                speed: 10.0,
                power: 1.0
            },
            ..default()
        },
        SpawnArea {
            radius: 10,
            scale: 5
        },
        PreviousSpawnUpdate(MapAddress{
            x: 1024, y: 1024, z: 1024
            //x: 0, y: 0, z: 0
        })
    )).id();

    // camera
    let camera_id = commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.5, 1.5)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    },
                                    Viewer
    )).id();

    // Attach camera to player
    commands.entity(player_id).push_children(&[camera_id]);

    // sun
    commands.spawn(
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
}

fn setup_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticlesMaterial>>
){
    let mut particles =
        Mesh::from(Particles { num_particles: 100 });

    let position_attribute = particles.attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
    let VertexAttributeValues::Float32x3(positions) = position_attribute else {
        panic!("Unexpected vertex format expected Float32x3.");
    };
    let mut colors: Vec<[f32; 4]> = Vec::new();
    for position in positions {
        colors.push( [
            position[0] * 2.0 - 1.0,
            position[1] * 2.0 - 1.0,
            position[2] * 2.0 - 1.0,
            1.,
        ]);
    }
    particles.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        colors,
    );

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(particles),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(ParticlesMaterial {
            time: 0.0,
            rate: 0.2,
        }),
        ..default()
    });
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "00cfdf10-7270-490d-8841-cf08b476303a"]
pub struct ParticlesMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    rate: f32,
}

#[derive(Debug, Copy, Clone)]
struct Particles {
    num_particles: u32,
}

impl Material for ParticlesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/particle.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/particle.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}

impl From<Particles> for Mesh {
    fn from(particles: Particles) -> Self {
        let extent = 0.5 / 2.0;
        let jump = extent / particles.num_particles as f32;
        let mut rng = fastrand::Rng::new();

        let vertices = (0..=particles.num_particles)
            .cartesian_product(0..=particles.num_particles)
            .cartesian_product(0..=particles.num_particles)
            .map(|((z, y), x)| {
                (
                    // Position
                    [
                        x as f32 * jump - 0.5 * extent + (rng.f32() - 0.5 * 0.1),
                        y as f32 * jump - 0.5 * extent + (rng.f32() - 0.5 * 0.1),
                        z as f32 * jump - 0.5 * extent + (rng.f32() - 0.5 * 0.1)
                    ],
                    // Normal
                    [
                        (x as f32 / particles.num_particles as f32) * 2.0 - 1.0,
                        (y as f32 / particles.num_particles as f32) * 2.0 - 1.0,
                        (z as f32 / particles.num_particles as f32) * 2.0 - 1.0
                    ]
                )
            })
            .collect::<Vec<_>>();

        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();

        for vert in vertices {
            positions.push(vert.0);
            normals.push(vert.1);
        }

        let mut mesh =
            Mesh::new(PrimitiveTopology::PointList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION,positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh
    }
}

fn update_time_for_particles_material(
    mut materials: ResMut<Assets<ParticlesMaterial>>,
    time: Res<Time>,
){
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds() as f32;
    }
}