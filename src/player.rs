
use bevy_xpbd_3d::{ prelude::*, PhysicsSchedule, PhysicsStepSet };
use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::CursorGrabMode;

const MOVE_SPEED:f32 = 4.0;
const MOUSE_SENSITIVITY: f32 = 0.05;
const ROLL_SPEED:f32 = 1.0;
const STRAFE_SPEED:f32 = 0.25;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                player_input,
                cursor_control))
            .add_systems(PhysicsSchedule, (
                player_linear_movement.before(PhysicsStepSet::BroadPhase),
                player_angular_movement.before(PhysicsStepSet::BroadPhase),
            ));
    }
}


#[derive(Component)]
pub struct Viewer;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerInput {
    pub direction: Vec3,
    pub rotation: Vec3
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            rotation: Vec3::ZERO
        }
    }
}


fn player_linear_movement(
    time: Res<Time>,
    mut query: Query<(&PlayerInput, &Transform, &mut LinearVelocity), (With<Player>, Without<Viewer>)>
){
    let delta = time.delta_seconds();
    let (input,
        transform,
        mut velocity) = query.single_mut();
    let mut force = Vec3::ZERO;
    force += transform.forward() * input.direction.z;
    force += transform.left() * input.direction.x;
    force += transform.up() * input.direction.y;

    velocity.0 += force * MOVE_SPEED * delta;
    velocity.0 *= 0.99;
}

fn player_angular_movement(
    time: Res<Time>,
    mut query: Query<(&PlayerInput, &Transform, &mut AngularVelocity), (With<Player>, Without<Viewer>)>
){
    let delta = time.delta_seconds();
    let (input,
        transform,
        mut velocity) = query.single_mut();

    let mut force = Vec3::ZERO;
    force += transform.forward() * input.rotation.z;
    force += transform.left() * input.rotation.x;
    force += transform.up() * input.rotation.y;

    velocity.0 += force * ROLL_SPEED * delta;
    velocity.0 *= 0.8;
}

fn cursor_control(
    mouse_button_input: Res<Input<MouseButton>>,
    mut windows: Query<&mut Window>,
){
    if mouse_button_input.pressed(MouseButton::Left) {
        for mut window in &mut windows {
            if !window.focused {
                continue;
            }

            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
        }
    }
    if mouse_button_input.just_released(MouseButton::Left) {
        for mut window in &mut windows {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}

fn player_input(
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    mut query: Query<&mut PlayerInput>,
){
    let mut player_input = query.single_mut();
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        direction.x = STRAFE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::D) {
        direction.x = -STRAFE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::W) {
        direction.z = 1.0;
    }
    if keyboard_input.pressed(KeyCode::S) {
        direction.z = -1.0;
    }
    if keyboard_input.pressed(KeyCode::Space) {
        direction.y = STRAFE_SPEED;
    }
    if keyboard_input.pressed(KeyCode::ShiftLeft){
        direction.y = -STRAFE_SPEED;
    }

    player_input.direction = direction;

    let mut rotation = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::Q) {
        rotation.z = -1.0;
    }
    if keyboard_input.pressed(KeyCode::E){
        rotation.z = 1.0;
    }

    let mut mouse_delta = Vec2::ZERO;
    if mouse_button_input.pressed(MouseButton::Left) {
        for movement in mouse_input.iter() {
            mouse_delta += movement.delta;
        }
        if (mouse_delta != Vec2::ZERO) {
            rotation.y = mouse_delta.x * MOUSE_SENSITIVITY * -1.0;
            rotation.x = mouse_delta.y * MOUSE_SENSITIVITY;
        }
    }
    player_input.rotation = rotation;


}



