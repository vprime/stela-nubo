// Entrypoint for the main game binary
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use noise::{
    core::worley::{distance_functions::*, worley_3d, ReturnType},
    permutationtable::PermutationTable
};
mod generation;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const SPIN_SPEED:Vec3 = Vec3::new(0.0, 0.0, 1.0);
const MOVE_SPEED:f32 = 0.9;
const SPAWN_LIFETIME:f32 = 10.0;
const SPAWN_SPEED:f32 = 0.0001;
const SPAWN_AREA: f32 = 10.0;
const SPAWN_SEED:u32 = 69;
const SPAWN_FREQUENCY:f64 = 1.0;
const SPAWN_DISPLACEMENT:f64 = 1.0;
const SPAWN_CUTOFF:f64 = 0.4;

#[derive(Resource)]
struct SpawnTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(SpawnTimer(Timer::from_seconds(SPAWN_SPEED, TimerMode::Repeating)))
        .add_systems(Startup, (setup, worley_spawner))
        .add_systems(Update, (
            //spin_cubes,
            move_viewer,
        ))
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct Spin(Vec3);

#[derive(Component, Deref)]
struct Death(f32);

#[derive(Component)]
struct Viewer;


fn move_viewer(
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Viewer>>,
    time: Res<Time>
){
    let delta = time.delta_seconds();
    let mut viewer_transform = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        direction += viewer_transform.left();
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += viewer_transform.right();
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction += viewer_transform.forward();
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction += viewer_transform.back();
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction += viewer_transform.up();
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft){
        direction += viewer_transform.down();
    }

    viewer_transform.translation.x += MOVE_SPEED * direction.x * delta;
    viewer_transform.translation.y += MOVE_SPEED * direction.y * delta;
    viewer_transform.translation.z += MOVE_SPEED * direction.z * delta;

    for movement in mouse_input.iter() {
        viewer_transform.rotate_local_y(movement.delta.x * 0.005 * -1.0);
        viewer_transform.rotate_local_x(movement.delta.y * 0.005);
    }
}


fn worley_spawner(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    // Spawn point size 10
    // scale 1:1
    let hasher = PermutationTable::new(SPAWN_SEED);

    let size = 10;

    for n in 0..(size * size * size) {

        let position = Vec3::new(
            (n % size) as f32,
            ((n % (size * size)) / size) as f32,
            (n / (size * size)) as f32
        );

        let noise_value = worley_3d(
            &hasher,
            &euclidean,
            ReturnType::Value,
            [position.x.into(), position.y.into(), position.z.into()]
        );

        if noise_value > SPAWN_CUTOFF {
            commands.spawn((PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(position.x, position.y, position.z),
                ..default()
            },
                //Spin(SPIN_SPEED),
                //Death(time.elapsed_seconds() + SPAWN_LIFETIME)
            ));
        }
    }
}

fn spin_cubes(
    mut query: Query<(&mut Transform, &Spin)>,
    time: Res<Time>
){
    let delta = time.delta_seconds();
    for(mut transform, spin) in &mut query {
        transform.rotate_x(spin.x * delta);
        transform.rotate_y(spin.y * delta);
        transform.rotate_z(spin.z * delta);
    }
}

fn despawn_cubes(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &Death)>
) {
    let elapsed = time.elapsed_seconds();
    for (entity, death) in &query {
        if death.0 < elapsed {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_cubes(mut commands: Commands,
         time: Res<Time>,
         mut timer: ResMut<SpawnTimer>,
         mut meshes: ResMut<Assets<Mesh>>,
         mut materials: ResMut<Assets<StandardMaterial>>,
        query: Query<&Transform, With<Viewer>>) {

    for transform in &query {
        let base_time: f32 = time.delta_seconds();

        let set_x: f32 = transform.translation.x + (base_time * 10000.0 % 6.0) - 3.0;
        let set_y: f32 = transform.translation.y + (base_time * 1000000.0 % 6.0) - 3.0;
        let set_z: f32 = transform.translation.z + (base_time * 100000000.0 % 6.0) - 3.0;
        commands.spawn((PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube {size: 0.25})),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(set_x, set_y, set_z),
            ..default()
        },
            Spin(SPIN_SPEED),
            Death(time.elapsed_seconds() + SPAWN_LIFETIME)
        ));
    }
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    },
        Viewer
    ));

    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    ));

}
