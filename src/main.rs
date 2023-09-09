mod generation;
mod player;
mod util;
mod weapon;
mod input;
mod destructible;
mod health;
mod states;
mod camera;
mod ui;

// Entrypoint for the main game binary
use bevy::{
    prelude::*,
};
use bevy_xpbd_3d::prelude::*;
use leafwing_input_manager::{
    prelude::*,
    Actionlike
};
use generation::*;
use crate::camera::CameraPlugin;
use crate::destructible::DestructiblesPlugin;
use crate::input::InputPlugin;
use crate::weapon::WeaponPlugin;
use crate::health::HealthPlugin;
use crate::player::PlayerPlugin;
use crate::states::*;
use crate::ui::UiPlugin;


const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            PlayerPlugin,
            MapGenerationPlugin,
            InputPlugin,
            StatesPlugin,
            WeaponPlugin,
            DestructiblesPlugin,
            HealthPlugin,
            UiPlugin,
            CameraPlugin
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .insert_resource(Gravity(Vec3::ZERO))
        .add_systems(OnEnter(GameStates::Paused), bevy_xpbd_3d::pause)
        .add_systems(OnExit(GameStates::Paused), bevy_xpbd_3d::resume)
        .run();
}
