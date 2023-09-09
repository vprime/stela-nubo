use bevy::prelude::*;

mod game;
pub use self::game::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameStates {
    #[default]
    Playing,
    Paused
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppStates {
    #[default]
    MainMenu,
    Game,
    GameOver,
    Victory
}


pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<GameStates>()
            .add_state::<AppStates>()
            .add_systems(Update, (
                focus_control,
                game_pause_button,
            ).run_if(in_state(AppStates::Game)))
            .add_systems(OnEnter(GameStates::Paused), set_cursor_grab)
            .add_systems(OnEnter(GameStates::Playing), set_cursor_grab)
            .add_systems(OnEnter(AppStates::Game), set_cursor_grab)
            .add_systems(OnExit(AppStates::Game), set_cursor_grab);
    }
}