mod generation;

// Entrypoint for the main game binary
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use generation::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            worley_spawner,
            move_player,
            despawn_cubes,
            focus_camera_on_player
        ))
        .run();
}


const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const MOVE_SPEED:f32 = 4.0;
const MOUSE_SENSITIVITY: f32 = 0.05;
const ROLL_SPEED:f32 = 1.0;


#[derive(Component)]
struct Viewer;

#[derive(Component)]
struct Player;


fn focus_camera_on_player(
    mut viewer_query: Query<&mut Transform, With<Viewer>>,
    player_query: Query<&Transform, (With<Player>, Without<Viewer>)>,
    time: Res<Time>
)
{
    let mut viewer_transform = viewer_query.single_mut();
    let mut player_transform = player_query.single();
    let dt = time.delta_seconds();

    viewer_transform.translation = player_transform.transform_point(Vec3::new(0.0, 0.5, 1.5));
    viewer_transform.look_at(player_transform.translation, player_transform.up());
}


fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, (With<Player>, Without<Viewer>)>,
    time: Res<Time>
){
    let delta = time.delta_seconds();
    let mut viewer_transform = query.single_mut();
    let mut direction = Vec3::ZERO;
    let mut roll = 0.0;

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
    if keyboard_input.pressed(KeyCode::Q) {
        roll += 1.0;
    }
    if keyboard_input.pressed(KeyCode::E){
        roll -= 1.0;
    }

    viewer_transform.translation.x += MOVE_SPEED / 2.0 * direction.x * delta;
    viewer_transform.translation.y += MOVE_SPEED / 2.0 * direction.y * delta;
    viewer_transform.translation.z += MOVE_SPEED * direction.z * delta;

    let mut mouse_delta = Vec2::ZERO;
    for movement in mouse_input.iter() {
        mouse_delta += movement.delta;
    }

    viewer_transform.rotate_local_z(roll * ROLL_SPEED * delta);
    if (mouse_delta != Vec2::ZERO) {
        viewer_transform.rotate_local_y(mouse_delta.x * delta * MOUSE_SENSITIVITY * -1.0);
        viewer_transform.rotate_local_x(mouse_delta.y * delta * MOUSE_SENSITIVITY);
    }
}



fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>
) {

    let player_spaceship = assets.load("models/player-ship/makoi.glb#Scene0");

    // player
    commands.spawn((
        SceneBundle {
            scene: player_spaceship,
            ..default()
        },
       Player,
       SpawnArea(10.0),
       PreviousSpawnUpdate(Vec3::ZERO)
    ));

    // camera
    commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, -5.0).looking_at(Vec3::new(0.0, 0.0, 1.0), Vec3::Y),
        ..default()
    },
        Viewer
    ));

    // sun
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
