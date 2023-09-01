mod generation;

// Entrypoint for the main game binary
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy_xpbd_3d::prelude::*;
use generation::*;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PhysicsPlugins::default()))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Gravity(Vec3::ZERO))
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
const MOUSE_SENSITIVITY: f32 = 0.005;
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
    mut query: Query<(&Transform, &LinearVelocity, &AngularVelocity, &mut ExternalForce, &mut ExternalTorque), (With<Player>, Without<Viewer>)>,
    time: Res<Time>
){
    let delta = time.delta_seconds();
    let (player_transform,
        linear_velocity,
        angular_velocity,
        mut player_force,
        mut player_torque) = query.single_mut();

    let mut direction = Vec3::ZERO;
    let mut roll = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        direction += player_transform.left();
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction += player_transform.right();
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction += player_transform.forward();
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction += player_transform.back();
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction += player_transform.up();
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft){
        direction += player_transform.down();
    }
    if keyboard_input.pressed(KeyCode::Q) {
        roll += 1.0;
    }
    if keyboard_input.pressed(KeyCode::E){
        roll -= 1.0;
    }

    let force = (direction * MOVE_SPEED * delta);
    player_force.apply_force(force).with_persistence(false);

    let mut mouse_delta = Vec2::ZERO;
    for movement in mouse_input.iter() {
        mouse_delta += movement.delta;
    }
    let mut torque = Vec3::ZERO;

    torque.z = roll * ROLL_SPEED * delta;

    if (mouse_delta != Vec2::ZERO) {
        torque.y = mouse_delta.x * delta * MOUSE_SENSITIVITY * -1.0;
        torque.x = mouse_delta.y * delta * MOUSE_SENSITIVITY;
    }

    player_torque.apply_torque(torque).with_persistence(false);
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
        RigidBody::Dynamic,
        Collider::cuboid(1.0, 1.0, 1.0),
        Friction::new(0.4),
        ExternalForce::default(),
        ExternalTorque::default(),
        LinearVelocity::default(),
        AngularVelocity::default(),
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
