mod generation;
mod player;
mod util;
mod weapon;
mod input;
mod application;

// Entrypoint for the main game binary
use bevy::prelude::*;
use bevy_xpbd_3d::{ prelude::*, PhysicsSchedule, PhysicsStepSet };
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::prelude::InputMap;
use generation::*;
use crate::application::ApplicationPlugin;
use crate::input::{InputPlugin, PlayerAction};
use crate::player::*;


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PlayerControllerPlugin,
            MapGenerationPlugin,
            InputPlugin,
            ApplicationPlugin
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Gravity(Vec3::ZERO))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let player_spaceship = assets.load("models/player-ship/makoi.glb#Scene0");

    let mut input_map = InputMap::default();
    for action in PlayerAction::variants() {
        input_map.insert(PlayerAction::default_keyboard_mouse_input(action), action);
    }

    // player
    let player_id = commands.spawn((
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
        InputManagerBundle::<PlayerAction>{
            action_state: ActionState::default(),
            input_map: input_map.build()
        },
        PlayerInput::default(),
        SpawnArea {
            radius: 10,
            scale: 5
        },
        PreviousSpawnUpdate(MapAddress{
            x: 1024, y: 1024, z: 1024
            //x: 0, y: 0, z: 0
        })
    )).id();

    // camera
    let camera_id = commands.spawn((Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.5, 1.5)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    },
                                    Viewer
    )).id();

    // Attach camera to player
    commands.entity(player_id).push_children(&[camera_id]);

    // sun
    commands.spawn((
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
    ));

}

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);


