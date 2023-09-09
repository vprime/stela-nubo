mod player;
mod util;
mod states;
mod camera;
mod ui;
mod arena;
mod effects;
mod components;
mod spawnable;

// Entrypoint for the main game binary
use bevy::{
    prelude::*,
};
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::{
    prelude::*,
    Actionlike
};
use crate::arena::ArenaPlugin;
use crate::camera::CameraPlugin;
use crate::components::ComponentPlugin;
use crate::effects::EffectsPlugin;
use crate::player::PlayerPlugin;
use crate::spawnable::SpawnablesPlugin;
use crate::states::*;
use crate::ui::UiPlugin;


const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PlayerPlugin,
            ArenaPlugin,
            ComponentPlugin,
            StatesPlugin,
            SpawnablesPlugin,
            EffectsPlugin,
            UiPlugin,
            CameraPlugin
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Gravity(Vec3::ZERO))
        .add_systems(OnEnter(GameStates::Paused), bevy_xpbd_3d::pause)
        .add_systems(OnExit(GameStates::Paused), bevy_xpbd_3d::resume)
        .run();
}
