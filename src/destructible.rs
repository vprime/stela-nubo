use bevy::{
    prelude::*,
    render::{
        mesh::VertexAttributeValues,
        render_resource::*,
    },
    reflect::{
        TypeUuid, TypePath
    },
};
use std::time::Duration;
use itertools::Itertools;

pub struct DestructiblesPlugin;

impl Plugin for DestructiblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_plugins((
                MaterialPlugin::<ParticlesMaterial>::default(),
            ))
            .add_systems(Update, (
                update_time_for_particles_material,
                spawn_explosions,
                decay_explosions,
            ));
    }
}
const EXPLODE_LIFE: f32 = 20.0;
#[derive(Event)]
pub struct DamageEvent {
    pub subject: Entity,
    pub value: f32
}

#[derive(Component)]
pub struct Explodeable;

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionLifetime {
    timer: Timer
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "00cfdf10-7270-490d-8841-cf08b476303a"]
pub struct ParticlesMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    start: f32,
    #[uniform(2)]
    end: f32,
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
        let extent = 0.1 / 2.0;
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

fn spawn_explosions(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticlesMaterial>>,
    mut damage_event: EventReader<DamageEvent>,
    explosive_query: Query<&Transform, With<Explodeable>>
)
{
    let now = time.elapsed_seconds();
    for damage in damage_event.iter(){
        if let Ok(transform) = explosive_query.get(damage.subject) {
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
            commands.spawn((
                MaterialMeshBundle {
                    mesh: meshes.add(particles),
                    transform: Transform::from_xyz(
                        transform.translation.x, transform.translation.y, transform.translation.z
                    ),
                    material: materials.add(ParticlesMaterial {
                        time: now,
                        start: now,
                        end: now + EXPLODE_LIFE,
                    }),
                    ..default()
                },
                Explosion,
                ExplosionLifetime {
                    timer: Timer::new(Duration::from_secs_f32(EXPLODE_LIFE), TimerMode::Once)
                },
            ));
        }
    }
}

fn decay_explosions(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut ExplosionLifetime)>
){
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}