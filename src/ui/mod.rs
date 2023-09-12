use bevy::app::AppExit;
use bevy::prelude::*;
use crate::states::{AppStates, GameStates};
use crate::ui::game_over::{GameOverUi, setup_game_over_ui};
use crate::ui::main_menu::{MainMenuUi, setup_main_menu};
use crate::ui::overlay::{setup_overlay_ui, update_health_overlay_text};
use crate::ui::pause_menu::{open_pause_menu, PauseMenuUi};
use crate::ui::victory::{setup_victory_ui, VictoryUi};

mod pause_menu;
mod main_menu;
mod game_over;
mod victory;
mod overlay;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                button_interaction_effects,
                menu_action
            ));

        app
            .add_systems(OnEnter(AppStates::MainMenu), setup_main_menu)
            .add_systems(OnExit(AppStates::MainMenu), close_panel::<MainMenuUi>);

        app.add_systems(
            OnEnter(GameStates::Paused),
            open_pause_menu.run_if(in_state(AppStates::Game)))
            .add_systems(
            OnEnter(GameStates::Playing),
            close_panel::<PauseMenuUi>)
            .add_systems(
            OnExit(AppStates::Game),
            close_panel::<PauseMenuUi>);

        app
            .add_systems(OnEnter(AppStates::GameOver), setup_game_over_ui)
            .add_systems(OnExit(AppStates::GameOver), close_panel::<GameOverUi>);

        app
            .add_systems(OnEnter(AppStates::Victory), setup_victory_ui)
            .add_systems(OnExit(AppStates::Victory), close_panel::<VictoryUi>);

        app
            .add_systems(OnEnter(AppStates::Game), setup_overlay_ui)
            .add_systems(Update, (update_health_overlay_text).run_if(in_state(AppStates::Game)));

    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const PANEL_BACKGROUND: Color = Color::rgba(0.75, 0.75, 1.0, 0.25);
const FONT_PATH: &str = "fonts/dogica/OTF/dogica.otf";

#[derive(Component)]
enum MenuButtonAction {
    Play,
    MainMenu,
    Resume,
    Quit
}

fn close_panel<T:Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>
){
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn button_interaction_effects(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>)
    >
){
    for (interaction, mut color, mut border_color, children) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = Color::RED;
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>)
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<GameStates>>,
    mut app_state: ResMut<NextState<AppStates>>
){
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    app_state.set(AppStates::Game);
                    game_state.set(GameStates::Playing);
                },
                MenuButtonAction::MainMenu => {
                    app_state.set(AppStates::MainMenu);
                },
                MenuButtonAction::Resume => {
                    game_state.set(GameStates::Playing);
                }
            }
        }
    }
}

