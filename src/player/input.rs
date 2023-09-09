use bevy::prelude::*;
use leafwing_input_manager::axislike::VirtualAxis;
use leafwing_input_manager::prelude::*;
use leafwing_input_manager::user_input::InputKind;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    // Flight Controls
    Forward,
    Left,
    Up,
    Yaw,
    Pitch,
    Roll,

    // Weapon Controls
    Shoot,

    // System Actions
    Menu
}

impl PlayerAction {
    pub fn default_keyboard_mouse_input(action: PlayerAction) -> UserInput {
        match action {
            Self::Forward => UserInput::VirtualAxis(VirtualAxis::ws()),
            Self::Left => UserInput::VirtualAxis(VirtualAxis::ad()),
            Self::Up => UserInput::VirtualAxis(VirtualAxis {
                negative: InputKind::Keyboard(KeyCode::Space),
                positive: InputKind::Keyboard(KeyCode::ShiftLeft)
            }),
            Self::Roll => UserInput::VirtualAxis(VirtualAxis {
                negative: InputKind::Keyboard(KeyCode::Q),
                positive: InputKind::Keyboard(KeyCode::E)
            }),
            Self::Pitch => UserInput::from(InputKind::SingleAxis(SingleAxis::mouse_motion_y())),
            Self::Yaw => UserInput::from(InputKind::SingleAxis(SingleAxis::mouse_motion_x())),
            Self::Shoot => UserInput::Single(InputKind::Mouse(MouseButton::Left)),
            Self::Menu => UserInput::Single(InputKind::Keyboard(KeyCode::Escape))
        }
    }
}