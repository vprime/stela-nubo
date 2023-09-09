use bevy::prelude::*;
use bevy::window::{CursorGrabMode, WindowFocused};
use leafwing_input_manager::prelude::ActionState;
use crate::input::PlayerAction;
use crate::states::{AppStates, GameStates};

pub fn set_cursor_grab(
    current_state: ResMut<State<GameStates>>,
    app_state: ResMut<State<AppStates>>,
    mut windows: Query<&mut Window>,
){
    if app_state.eq(&AppStates::Game) {
        match current_state.get() {
            GameStates::Playing => {
                for mut window in &mut windows {
                    window.cursor.visible = false;
                    window.cursor.grab_mode = CursorGrabMode::Locked;
                }
            },
            GameStates::Paused => {
                for mut window in &mut windows {
                    window.cursor.visible = true;
                    window.cursor.grab_mode = CursorGrabMode::None;
                }
            }
        }
    } else {
        for mut window in &mut windows {
            window.cursor.visible = true;
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
}

pub fn game_pause_button(
    query: Query<&ActionState<PlayerAction>>,
    current_state: ResMut<State<GameStates>>,
    mut next_state: ResMut<NextState<GameStates>>
){
    let player_input = query.single();
    if player_input.just_pressed(PlayerAction::Menu) {
        let new_state = match current_state.get() {
            GameStates::Playing => GameStates::Paused,
            GameStates::Paused => GameStates::Playing
        };
        next_state.set(new_state);
    }
}


pub fn focus_control(
    mut window_focused: EventReader<WindowFocused>,
    mut next_state: ResMut<NextState<GameStates>>
){
    for focus in window_focused.iter() {
        if !focus.focused {
            next_state.set(GameStates::Paused);
        }
    }
}