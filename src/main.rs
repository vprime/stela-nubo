// Entrypoint for the main game binary
use bevy::prelude::*;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const SPIN_SPEED:Vec3 = Vec3::new(0.0, 0.0, 1.0);
const MOVE_SPEED:Vec3 = Vec3::new(-0.9, -0.1, -0.1);
const SPAWN_LIFETIME:f32 = 10.0;
const SPAWN_SPEED:f32 = 0.0001;

#[derive(Resource)]
struct SpawnTimer(Timer);


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(SpawnTimer(Timer::from_seconds(SPAWN_SPEED, TimerMode::Repeating)))
        .add_systems(Startup, setup)
        .add_systems(Update, (spin_cubes, move_cubes, spawn_cubes, despawn_cubes))
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec3);

#[derive(Component, Deref, DerefMut)]
struct Spin(Vec3);

#[derive(Component, Deref)]
struct Death(f32);

fn move_cubes(
    mut query: Query<(&mut Transform, &Velocity, &Death)>,
    time: Res<Time>
){
    let delta = time.delta_seconds();
    for(mut transform, velocity, death) in &mut query {
        let deathMultiplyer = (death.0 - time.elapsed_seconds()) * 0.7;
        transform.translation.x += velocity.x * delta * deathMultiplyer;
        transform.translation.x += velocity.y * delta * deathMultiplyer;
        transform.translation.x += velocity.z * delta * deathMultiplyer;
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
         mut materials: ResMut<Assets<StandardMaterial>>) {


    // if !timer.0.tick(time.delta()).just_finished() {
    //     return;
    // }
    let base_time: f32 = time.delta_seconds();
    let set_x: f32 = (base_time * 10000.0 % 2.0) + 5.0;
    let set_y: f32 = (base_time * 100000.0 % 6.0) - 3.0;
    let set_z: f32 = (base_time * 1000000.0 % 2.0);
    commands.spawn((PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {size: 0.25})),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(set_x, set_y, set_z),
        ..default()
    },
                    Velocity(MOVE_SPEED),
                    Spin(SPIN_SPEED * set_z),
        Death(time.elapsed_seconds() + SPAWN_LIFETIME)
    ));
}


fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {

    // // Asset loading builtins
    // let drill = asset_server.load("models/Jacklegdrill.glb#Scene0");
    //
    // // Drill
    // commands.spawn((
    //     SceneBundle {
    //     scene: drill,
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     ..default()
    // },
    //     Velocity(INITIAL_DRILL_DIRECTION.normalize() * SPIN_SPEED)
    // ));

    // Plane
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    //     material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
    //     ..default()
    // });
    //
    // // Cube
    // commands.spawn((PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube {size: 1.0})),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     ..default()
    // },
    //                 Velocity(SPIN_SPEED)
    // ));

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

}
