use bevy::prelude::*;
use bevy::window::{CursorGrabMode, WindowFocused};
use leafwing_input_manager::prelude::ActionState;
use crate::input::PlayerAction;

pub struct ApplicationPlugin;

impl Plugin for ApplicationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                cursor_control));
    }
}


fn cursor_control(
    query: Query<&ActionState<PlayerAction>>,
    mut window_focused: EventReader<WindowFocused>,
    mut windows: Query<&mut Window>,
){
    let player_input = query.single();
    for focus in window_focused.iter() {
        if focus.focused {
            for mut window in &mut windows {
                window.cursor.grab_mode = CursorGrabMode::Locked;
                window.cursor.visible = false;
            }
        }
    }

    if player_input.pressed(PlayerAction::Menu) {
        for mut window in &mut windows {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        }
    }
}