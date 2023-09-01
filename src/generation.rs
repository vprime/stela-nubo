
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

use noise::{
    core::worley::{distance_functions::*, worley_3d, ReturnType},
    permutationtable::PermutationTable
};

const SPAWN_SPEED:f32 = 0.1;
const SPAWN_SEED:u32 = 69;
const SPAWN_FREQUENCY:f64 = 1.0;
const SPAWN_DISPLACEMENT:f64 = 1.0;

const SPAWN_CUTOFF:f64 = 0.9;
const SPAWN_AREA: f32 = 100.0;
const SPAWN_BLOCK_SIZE: f32 = 3.0;
const SPAWN_DENSITY: f32 = 0.5;


#[derive(Component)]
pub struct Asteroid;

#[derive(Component, Deref)]
pub struct SpawnArea(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct PreviousSpawnUpdate(pub Vec3);

pub fn worley_spawner(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Transform, &SpawnArea, &mut PreviousSpawnUpdate)>
){
    let hasher = PermutationTable::new(SPAWN_SEED);

    for(transform, area, mut previous) in &mut query
    {
        let size = area.0.floor() as i16;
        let radius = area.0 * 0.5;

        for n in 0..(size * size * size) {

            let position = Vec3::new(
                (transform.translation.x - radius + (n % size) as f32).floor(),
                (transform.translation.y - radius + ((n % (size * size)) / size) as f32).floor(),
                (transform.translation.z - radius + (n / (size * size)) as f32).floor()
            );

            if intersecting(position, previous.0, radius){
                continue;
            }

            let noise_value = worley_3d(
                &hasher,
                &euclidean,
                ReturnType::Value,
                [position.x.into(), position.y.into(), position.z.into()]
            );

            if noise_value > SPAWN_CUTOFF {
                commands.spawn((PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube {size: noise_value as f32})),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(position.x, position.y, position.z),
                    ..default()
                }, Asteroid,
                   Collider::cuboid(noise_value as f32, noise_value as f32, noise_value as f32),
                ));
            }
        }
        previous.0 = transform.translation;
    }
}


fn intersecting(a_point: Vec3, b_bound: Vec3, radius: f32) -> bool {
    (a_point.x < b_bound.x + radius && a_point.x > b_bound.x - radius) &&
        (a_point.y < b_bound.y + radius && a_point.y > b_bound.y - radius) &&
        (a_point.z < b_bound.z + radius && a_point.z > b_bound.z - radius)
}

pub fn despawn_cubes(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Asteroid>>,
    spawner: Query<(&Transform, &SpawnArea)>
) {
    for (entity, transform) in &query {
        for (spawner_transform, spawner_area) in &spawner {
            if  intersecting(transform.translation, spawner_transform.translation, spawner_area.0) {
                continue;
            }
            commands.entity(entity).despawn();
        }
    }
}
