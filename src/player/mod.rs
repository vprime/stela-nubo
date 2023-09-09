
use bevy_xpbd_3d::{ prelude::*, PhysicsSchedule, PhysicsStepSet };
use bevy::prelude::*;
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use leafwing_input_manager::prelude::{ActionState, InputManagerPlugin, InputMap};
use crate::spawnable::{Cannon, NextShot, WeaponBundle, WeaponOptions};
use crate::states::{AppStates, GameStates};
use crate::arena::generation::{SpawnArea, PreviousSpawnUpdate, MapAddress};
use crate::components::{DeathEvent, Health};
use crate::effects::ExplosionEvent;
use crate::player::input::PlayerAction;

pub mod input;
const MOVE_SPEED:f32 = 5.0;
const PITCH_SENSITIVITY: f32 = 10.0;
const ROLL_SPEED:f32 = 10.0;
const STRAFE_SPEED:f32 = 3.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .add_systems(Update, (
                player_input,
                player_death
            ).run_if(in_state(GameStates::Playing))
             .run_if(in_state(AppStates::Game)))

            .add_systems(PhysicsSchedule, (
                player_linear_movement.before(PhysicsStepSet::BroadPhase),
                player_angular_movement.before(PhysicsStepSet::BroadPhase),
            ).run_if(in_state(GameStates::Playing))
                .run_if(in_state(AppStates::Game)))

            .add_systems(OnEnter(AppStates::Game), (
                spawn_player
            ))
            .add_systems(OnExit(AppStates::Game), (
                detach_camera_from_player,
                reset_player.after(detach_camera_from_player)
            ));
    }
}


#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerInput {
    pub direction: Vec3,
    pub rotation: Vec3,
    pub enabled: bool,
}

impl Default for PlayerInput {
    fn default() -> Self {
        Self {
            direction: Vec3::ZERO,
            rotation: Vec3::ZERO,
            enabled: true
        }
    }
}


// fn pause_input(
//     current_state: ResMut<State<GameStates>>,
//     mut player_input: Query<&mut PlayerInput>
// ){
//     match current_state.get() {
//         GameStates::Playing => {
//             for mut input in &mut player_input {
//                 input.enabled = true;
//             }
//         },
//         GameStates::Paused => {
//             for mut input in &mut player_input {
//                 input.enabled = false;
//             }
//         }
//     }
// }


fn player_linear_movement(
    time: Res<Time>,
    mut query: Query<(&PlayerInput, &Transform, &mut LinearVelocity), With<Player>>
){
    let delta = time.delta_seconds();
    let (input,
        transform,
        mut velocity) = query.single_mut();
    if !input.enabled {
        return;
    }
    let mut force = Vec3::ZERO;
    force += transform.forward() * input.direction.z * MOVE_SPEED;
    force += transform.right() * input.direction.x * STRAFE_SPEED;
    force += transform.down() * input.direction.y * STRAFE_SPEED;

    velocity.0 += force * delta;
    velocity.0 *= 0.99;
}

fn player_angular_movement(
    time: Res<Time>,
    mut query: Query<(&PlayerInput, &Transform, &mut AngularVelocity), With<Player>>
){
    let delta = time.delta_seconds();
    let (input,
        transform,
        mut velocity) = query.single_mut();

    if !input.enabled {
        return;
    }
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

    if !player_input.enabled {
        player_input.direction = Vec3::ZERO;
        player_input.rotation = Vec3::ZERO;
        cannon.0 = false;
        return;
    }
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

fn player_death(
    mut death_event: EventReader<DeathEvent>,
    mut explosion_event: EventWriter<ExplosionEvent>,
    mut query: Query<(&Transform, &mut Visibility, &mut PlayerInput), With<Player>>,
    mut next_state: ResMut<NextState<AppStates>>
){
    for death in death_event.iter(){
        if let Ok((transform, mut visibility,  mut input)) = query.get_mut(death.subject) {
            input.enabled = false;
            *visibility = Visibility::Hidden;
            explosion_event.send(ExplosionEvent {
                position: transform.translation,
                power: 10.0
            });
            next_state.set(AppStates::GameOver);
        }
    }
}

fn reset_player(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
){
    for player in player_query.iter() {
        commands.entity(player).despawn_recursive();
    }
}



fn spawn_player(
    mut commands: Commands,
    assets: Res<AssetServer>,
    camera_query: Query<Entity, With<Camera>>,
    time: Res<Time>
) {
    let player_spaceship = assets.load("models/player-ship/makoi.glb#Scene0");
    let camera = camera_query.single();

    let mut input_map = InputMap::default();
    for action in PlayerAction::variants() {
        input_map.insert(PlayerAction::default_keyboard_mouse_input(action), action);
    }

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
        Health {
            full: 100.0,
            current: 100.0,
        },
        InputManagerBundle::<PlayerAction>{
            action_state: ActionState::default(),
            input_map: input_map.build()
        },
        PlayerInput::default(),
        WeaponBundle {
            options: WeaponOptions {
                rate: 0.1,
                speed: 10.0,
                power: 1.0
            },
            next_shot: NextShot(time.elapsed_seconds() + 0.5),
            ..default()
        },
        SpawnArea {
            radius: 10,
            scale: 5
        },
        PreviousSpawnUpdate(MapAddress{
            x: 1024, y: 1024, z: 1024
            //x: 0, y: 0, z: 0
        })
    )).push_children(&[camera]);

    // sun
    commands.spawn(
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                illuminance: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        }
    );
}

fn detach_camera_from_player(
    mut commands: Commands,
    camera_query: Query<Entity, With<Camera>>
){
    let camera = camera_query.single();
    // Remove camera from player
    commands.entity(camera).remove_parent();
}

