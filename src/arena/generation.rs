use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use noise::{
    core::worley::{distance_functions::*, worley_3d, ReturnType},
    permutationtable::PermutationTable
};
extern crate queues;
use queues::*;
use crate::destructible::{Explodeable, ExplosionEvent};
use crate::health::*;
use crate::player::Player;
use crate::states::{GameStates, AppStates};

const SPAWN_SEED:u32 = 69;
const SPAWN_CUTOFF:f64 = 0.7;


#[derive(Resource)]
pub struct SpawnHashTable(PermutationTable);


#[derive(Component)]
pub struct Asteroid;


#[derive(Component)]
pub struct SpawnableHandles {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>
}


#[derive(Clone)]
struct AsteroidData {
    address: MapAddress,
    size: i32,
    scale: i32
}

#[derive(Bundle)]
pub struct MapSpawnerBundle {
    spawn_area: SpawnArea,
    previous_spawn_update: PreviousSpawnUpdate,
    spawn_queue: SpawnQueue
}

#[derive(Component)]
pub struct SpawnArea {
    pub radius: i32,
    pub scale: i32
}

#[derive(Component)]
pub struct PreviousSpawnUpdate(pub MapAddress);

#[derive(Clone, PartialEq)]
pub struct MapAddress {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

#[derive(Component)]
pub struct SpawnQueue(Queue<AsteroidData>);

pub fn spawn_setup(
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

pub fn spawn_from_queue(
    mut commands: Commands,
    mut spawn_queue_query: Query<&mut SpawnQueue, Changed<SpawnQueue>>,
    handle_query: Query<&SpawnableHandles>
){
    let handles = handle_query.single();
    for mut spawn_queue in &mut spawn_queue_query {
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
                        Position(position),
                        Explodeable,
                        Health {
                            full: 10.0,
                            current: 10.0
                        }
                    ))
                },
                Err(error) => println!("Error dequeing spawnnable: {0}", error)
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

pub fn worley_spawner(
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

pub fn despawn_cubes(
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
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn destroy_asteroids(
    asteroids: Query<(Entity, &Transform), With<Asteroid>>,
    mut commands: Commands,
    mut death_event: EventReader<DeathEvent>,
    mut explosion_event: EventWriter<ExplosionEvent>
){
    for death in death_event.iter(){
        if let Ok((entity, transform)) = asteroids.get(death.subject) {
            explosion_event.send(ExplosionEvent {
                position: transform.translation,
                power: 1.0,
            });
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn damage_player(
    asteroids: Query<Entity, With<Asteroid>>,
    players: Query<Entity, With<Player>>,
    mut collision_event: EventReader<CollisionStarted>,
    mut damage_event: EventWriter<DamageEvent>
){
    for collision in collision_event.iter() {
        if let Ok(player) = players.get(collision.0) {
            if asteroids.contains(collision.1) {
                println!("Player First hit");
                damage_event.send(DamageEvent{subject: player, value: 10.0});
            }
        }
        if let Ok(player) = players.get(collision.1) {
            if asteroids.contains(collision.0) {
                println!("Asteroid first hit");
                damage_event.send(DamageEvent{subject: player, value: 10.0});
            }
        }
    }
}

pub fn clean_up_map(
    asteroids: Query<Entity, With<Asteroid>>,
    mut queues: Query<&mut SpawnQueue>,
    mut commands: Commands,
){
    for asteroid in asteroids.iter() {
        commands.entity(asteroid).despawn_recursive();
    }
    for mut queue in &mut queues.iter_mut() {
        while queue.0.size() > 0 {
            let _ = queue.0.remove();
        }
    }
}