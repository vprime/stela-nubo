
use bevy_xpbd_3d::{ prelude::*, PhysicsSchedule, PhysicsStepSet };
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use crate::input::PlayerAction;
use crate::weapon::Cannon;

const MOVE_SPEED:f32 = 5.0;
const PITCH_SENSITIVITY: f32 = 10.0;
const ROLL_SPEED:f32 = 10.0;
const STRAFE_SPEED:f32 = 3.0;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                player_input))
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
    force += transform.forward() * input.direction.z * MOVE_SPEED;
    force += transform.right() * input.direction.x * STRAFE_SPEED;
    force += transform.down() * input.direction.y * STRAFE_SPEED;

    velocity.0 += force * delta;
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
    force += transform.forward() * input.rotation.z *  ROLL_SPEED;
    force += transform.right() * input.rotation.x * PITCH_SENSITIVITY;
    force += transform.up() * input.rotation.y * PITCH_SENSITIVITY;

    velocity.0 += force * delta;
    velocity.0 *= 0.8;
}

fn player_input(
    mut query: Query<(&ActionState<PlayerAction>, &mut PlayerInput, &mut Cannon)>,
){
    let (input_state, mut player_input, mut cannon) = query.single_mut();

    let mut direction = Vec3::ZERO;
    if input_state.pressed(PlayerAction::Left){
        direction.x = input_state.value(PlayerAction::Left);
    }
    if input_state.pressed(PlayerAction::Forward) {
        direction.z = input_state.value(PlayerAction::Forward);
    }
    if input_state.pressed(PlayerAction::Up) {
        direction.y = input_state.value(PlayerAction::Up);
    }
    player_input.direction = direction;

    let mut rotation = Vec3::ZERO;
    if input_state.pressed(PlayerAction::Pitch) {
        rotation.x = input_state.value(PlayerAction::Pitch);
    }
    if input_state.pressed(PlayerAction::Yaw) {
        rotation.y = input_state.value(PlayerAction::Yaw);
    }
    if input_state.pressed(PlayerAction::Roll) {
        rotation.z = input_state.value(PlayerAction::Roll);
    }
    player_input.rotation = rotation;

    cannon.0 = input_state.pressed(PlayerAction::Shoot);
}



