use bevy::prelude::*;
use crate::ui::{FONT_PATH, MenuButtonAction, NORMAL_BUTTON, PANEL_BACKGROUND};

#[derive(Component)]
pub struct GameOverUi;

pub fn setup_game_over_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands
        // Menu Background
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
                GameOverUi
        ))
        .with_children(|parent| {
            // Text
            parent.spawn((
                TextBundle::from_section(
                    "Game Over",
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
                Label,
            ));
            // Play
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
                        MenuButtonAction::MainMenu
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section("Main Menu", TextStyle {
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