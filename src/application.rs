use bevy::prelude::*;
use bevy::window::{CursorGrabMode, WindowFocused};
use leafwing_input_manager::prelude::ActionState;
use crate::input::PlayerAction;

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<AppState>()
            .add_systems(Update, (
                focus_control,
                game_pause_button,
            ))
            .add_systems(OnEnter(AppState::PAUSE), set_cursor_grab)
            .add_systems(OnEnter(AppState::PLAY), set_cursor_grab);
    }
}


#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum AppState {
    PLAY,
    #[default]
    PAUSE
}

fn set_cursor_grab(
    current_state: ResMut<State<AppState>>,
    mut windows: Query<&mut Window>,
){
    match current_state.get() {
        AppState::PLAY => {
            for mut window in &mut windows {
                window.cursor.visible = false;
                window.cursor.grab_mode = CursorGrabMode::Locked;
            }
        },
        AppState::PAUSE => {
            for mut window in &mut windows {
                window.cursor.visible = true;
                window.cursor.grab_mode = CursorGrabMode::None;
            }
        }
    }
}

fn game_pause_button(
    query: Query<&ActionState<PlayerAction>>,
    current_state: ResMut<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>
){
    let player_input = query.single();
    if player_input.just_pressed(PlayerAction::Menu) {
        let new_state = match current_state.get() {
            AppState::PAUSE => AppState::PLAY,
            AppState::PLAY => AppState::PAUSE
        };
        next_state.set(new_state);
    }
}


fn focus_control(
    mut window_focused: EventReader<WindowFocused>,
    mut next_state: ResMut<NextState<AppState>>
){
    for focus in window_focused.iter() {
        if !focus.focused {
            next_state.set(AppState::PAUSE);
        }
    }
}