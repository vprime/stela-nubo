
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use noise::{
    core::worley::{distance_functions::*, worley_3d, ReturnType},
    permutationtable::PermutationTable
};
extern crate queues;
use queues::*;
use crate::destructible::DamageEvent;

const SPAWN_SPEED:f32 = 0.1;
const SPAWN_SEED:u32 = 69;
const SPAWN_FREQUENCY:f64 = 1.0;
const SPAWN_DISPLACEMENT:f64 = 1.0;

const SPAWN_CUTOFF:f64 = 0.7;
const SPAWN_AREA: f32 = 100.0;
const SPAWN_BLOCK_SIZE: f32 = 3.0;
const SPAWN_DENSITY: f32 = 0.5;

pub struct MapGenerationPlugin;

#[derive(Resource)]
struct SpawnHashTable(PermutationTable);

impl Plugin for MapGenerationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (
                worley_spawner,
                despawn_cubes,
                spawn_from_queue,
                destroy_asteroids
        ));
    }
}

#[derive(Component)]
pub struct SpawnArea {
    pub radius: i32,
    pub scale: i32
}

#[derive(Component)]
pub struct Asteroid;

#[derive(Component)]
pub struct PreviousSpawnUpdate(pub MapAddress);

#[derive(Component)]
struct SpawnQueue(Queue<AsteroidData>);

#[derive(Component)]
struct SpawnableHandles {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>
}

#[derive(Bundle)]
pub struct MapSpawnerBundle {
    spawn_area: SpawnArea,
    previous_spawn_update: PreviousSpawnUpdate,
    spawn_queue: SpawnQueue
}

#[derive(Clone)]
struct AsteroidData {
    address: MapAddress,
    size: i32,
    scale: i32
}

#[derive(Clone, PartialEq)]
pub struct MapAddress {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    commands.insert_resource(SpawnHashTable{ 0: PermutationTable::new(SPAWN_SEED) });
    commands.spawn(SpawnQueue {0: queue![]});
    commands.spawn(SpawnableHandles {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        });
}

fn spawn_from_queue(
    mut commands: Commands,
    mut spawn_queue_query: Query<&mut SpawnQueue>,
    handle_query: Query<&SpawnableHandles>
){
    let handles = handle_query.single();
    for mut spawn_queue in &mut spawn_queue_query {
        let size = spawn_queue.0.size();
        while spawn_queue.0.size() > 0 {
            let spawnable_result = spawn_queue.0.remove();

            match spawnable_result {
                Ok(spawnable) => {
                    let position = address_to_translation(spawnable.address, spawnable.scale);
                    _= commands.spawn((PbrBundle {
                        mesh: handles.mesh.clone(),
                        material: handles.material.clone(),
                        ..default()
                    }, Asteroid,
                        Collider::cuboid(1.0, 1.0, 1.0),
                        RigidBody::Static,
                        Position(position)
                    ))
                },
                Err(error) => println!("Error dequeing spawnnable")
            }
        }
    }
}

fn translation_to_address(translation: Vec3, scale: i32) -> MapAddress {
    MapAddress {
        x: (translation.x * (1.0 / scale as f32)).floor() as i32,
        y: (translation.y * (1.0 / scale as f32)).floor() as i32,
        z: (translation.z * (1.0 / scale as f32)).floor() as i32
    }
}

fn address_to_translation(address: MapAddress, scale: i32) -> Vec3 {
    Vec3::new(
        (address.x * scale) as f32,
        (address.y * scale) as f32,
        (address.z * scale) as f32
    )
}

fn worley_spawner(
    spawn_hasher: Res<SpawnHashTable>,
    mut query: Query<(&Transform, &SpawnArea, &mut PreviousSpawnUpdate)>,
    mut spawn_queue_query: Query<&mut SpawnQueue>
){
    let mut spawn_queue = spawn_queue_query.single_mut();
    for(transform, area, mut previous) in &mut query
    {
        let current_address = translation_to_address(transform.translation, area.scale);
        if current_address == previous.0 {
            continue;
        }
        let size = area.radius * 2;

        for n in 0..(size * size * size) {

            let position = MapAddress {
                x: (current_address.x - area.radius + (n % size)),
                y: (current_address.y - area.radius + ((n % (size * size)) / size)),
                z: (current_address.z - area.radius + (n / (size * size)))
            };

            if intersecting(&position, &previous.0, &area.radius){
                continue;
            }

            let noise_value = worley_3d(
                &spawn_hasher.0,
                &euclidean,
                ReturnType::Value,
                [position.x.into(), position.y.into(), position.z.into()]
            );

            if noise_value > SPAWN_CUTOFF {
                let _ = spawn_queue.0.add(AsteroidData
                    {
                        address: MapAddress {
                            x: position.x,
                            y: position.y,
                            z: position.z,
                        },
                        size: (noise_value * 1000.0) as i32,
                        scale: area.scale
                    }
                );
            }
        }
        previous.0 = current_address;
    }
}

pub fn intersecting(a_point: &MapAddress, b_bound: &MapAddress, radius: &i32) -> bool {
    (a_point.x < b_bound.x + radius && a_point.x > b_bound.x - radius) &&
        (a_point.y < b_bound.y + radius && a_point.y > b_bound.y - radius) &&
        (a_point.z < b_bound.z + radius && a_point.z > b_bound.z - radius)
}

fn despawn_cubes(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<Asteroid>>,
    spawner: Query<(&Transform, &SpawnArea)>
) {
    for (entity, transform) in &query {
        for (spawner_transform, spawner_area) in &spawner {
            let entity_address = translation_to_address(transform.translation, spawner_area.scale);
            let spawner_address = translation_to_address(spawner_transform.translation, spawner_area.scale);
            if intersecting(&entity_address, &spawner_address, &spawner_area.radius) {
                continue;
            }
            commands.entity(entity).despawn();
        }
    }
}

fn destroy_asteroids(
    asteroids: Query<Entity, With<Asteroid>>,
    mut commands: Commands,
    mut damage_event: EventReader<DamageEvent>
){
    for damage in damage_event.iter(){
        if asteroids.contains(damage.subject) {
            commands.entity(damage.subject).despawn();
            println!("Destroying Asteroid {0:?}!", damage.subject);
        }
    }
}


