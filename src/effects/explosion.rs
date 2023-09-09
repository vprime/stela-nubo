use bevy::{
    prelude::*,
    render::{
        mesh::VertexAttributeValues,
        render_resource::*,
    },
};
use std::time::Duration;
use itertools::Itertools;
use crate::effects::{Explosion, ExplosionEvent, Particles, ParticlesMaterial};
use crate::util::{Lifetime};

const EXPLODE_LIFE: f32 = 5.0;


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

        let positions: Vec<_> =
            vertices.iter().map(|(p, _)| *p).collect();
        let normals: Vec<_> =
            vertices.iter().map(|(_, n)| *n).collect();

        let mut mesh =
            Mesh::new(PrimitiveTopology::PointList);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION,positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh
    }
}

pub fn update_time_for_particles_material(
    mut materials: ResMut<Assets<ParticlesMaterial>>,
    time: Res<Time>,
){
    for material in materials.iter_mut() {
        material.1.time = time.elapsed_seconds() as f32;
    }
}

pub fn spawn_explosions(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ParticlesMaterial>>,
    mut explosion_event: EventReader<ExplosionEvent>
)
{
    let now = time.elapsed_seconds();
    for explosion in explosion_event.iter(){
        let mut particles =
            Mesh::from(Particles { num_particles: (explosion.power * 10.0) as u32 });

        if let Some(VertexAttributeValues::Float32x3(
                        positions,
                    )) = particles.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[r, g, b]| {
                    [
                        *r * 2.0 - 1.0,
                        *g * 2.0 - 1.0,
                        *b * 2.0 - 1.0,
                        1.,
                    ]
                })
                .collect();
            particles.insert_attribute(
                Mesh::ATTRIBUTE_COLOR,
                colors,
            );
        }

        commands.spawn((
            MaterialMeshBundle {
                mesh: meshes.add(particles),
                transform: Transform::from_xyz(
                    explosion.position.x, explosion.position.y, explosion.position.z
                ),
                material: materials.add(ParticlesMaterial {
                    time: now,
                    start: now,
                    end: now + EXPLODE_LIFE,
                }),
                ..default()
            },
            Explosion,
            Lifetime {
                timer: Timer::new(Duration::from_secs_f32(EXPLODE_LIFE), TimerMode::Once)
            },
        ));
    }
}
