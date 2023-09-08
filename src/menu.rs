use bevy::app::AppExit;
use bevy::prelude::*;
use crate::application::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::PAUSE), open_pause_menu)
            .add_systems(OnEnter(AppState::PLAY), close_panel::<PauseMenu>)
            .add_systems(Update, (
                button_interaction_effects,
                menu_action
            ).run_if(in_state(AppState::PAUSE)));
    }
}

// UI Needs
// Start menu: Play, Quit
// Pause menu: Resume, Restart, Quit
// Overlay: Health, Points
// Death Screen: Restart, Quit
// Win screen: Restart, Quit



const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);
const PANEL_BACKGROUND: Color = Color::rgba(0.75, 0.75, 1.0, 0.25);
const FONT_PATH: &str = "fonts/dogica/OTF/dogica.otf";

#[derive(Component)]
struct PauseMenu;

#[derive(Component)]
struct GameOverlay;

#[derive(Component)]
struct DeathScreen;

#[derive(Component)]
struct WinScreen;


#[derive(Component)]
enum MenuButtonAction {
    Play,
    Continue,
    Quit
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
    mut app_state: ResMut<NextState<AppState>>
){
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    println!("Restarting Game!");
                },
                MenuButtonAction::Continue => {
                    app_state.set(AppState::PLAY);
                }
            }
        }
    }
}

fn open_pause_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands
        // Pause Menu Background
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: PANEL_BACKGROUND.into(),
            ..default()
        },
                PauseMenu
        ))
        .with_children(|parent| {
            // Text
            parent.spawn((
                TextBundle::from_section(
                    "Game Paused",
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 30.0,
                        color: Color::WHITE,
                    },
                )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    }),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
            // Resume
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        margin: UiRect::vertical(Val::Px(10.)),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                    MenuButtonAction::Continue
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Resume", TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 10.0,
                        color: Color::rgb(0.9, 0.9, 0.9)
                    }));
                });

            // Restart Button
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(10.)),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                    MenuButtonAction::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Restart", TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 10.0,
                        color: Color::rgb(0.9, 0.9, 0.9)
                    }));
                });

            // Quit Button
            parent
                .spawn((ButtonBundle {
                    style: Style {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(5.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        margin: UiRect::vertical(Val::Px(10.)),
                        ..default()
                    },
                    border_color: BorderColor(Color::BLACK),
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
                    MenuButtonAction::Quit
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Quit", TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 10.0,
                        color: Color::rgb(0.9, 0.9, 0.9)
                    }));
                });
        });
}

fn close_panel<T:Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>
){
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}