mod generation;
mod player;

// Entrypoint for the main game binary
use bevy::prelude::*;
use bevy_xpbd_3d::{ prelude::*, PhysicsSchedule, PhysicsStepSet };
use generation::*;
use crate::player::*;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PlayerControllerPlugin,
            MapGenerationPlugin
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Gravity(Vec3::ZERO))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let player_spaceship = assets.load("models/player-ship/makoi.glb#Scene0");

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
        PlayerInput::default(),
        SpawnArea(10.0),
        PreviousSpawnUpdate(Vec3::ZERO)
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
    commands.spawn((
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
    ));

}

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);


