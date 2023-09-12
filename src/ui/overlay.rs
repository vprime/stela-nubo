use bevy::prelude::*;
use crate::components::Health;
use crate::player::Player;
use crate::ui::{FONT_PATH, MenuButtonAction, NORMAL_BUTTON, PANEL_BACKGROUND};

#[derive(Component)]
pub struct GameOverlayUi;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
struct PointText;

pub fn setup_overlay_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    commands
        // Overlay container
        .spawn((NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                position_type: PositionType::Absolute,
                ..default()
            },
            ..default()
        },
                GameOverlayUi
        ))
        .with_children(|parent| {
            // Health text
            parent.spawn((
                TextBundle::from_section(
                    "Health:",
                    TextStyle {
                        font: asset_server.load(FONT_PATH),
                        font_size: 10.0,
                        color: Color::WHITE,
                    },
                )
                    .with_style(Style {
                        margin: UiRect::all(Val::Px(5.)),
                        ..default()
                    }),
                Label,
                HealthText,
            ));

        });
}

pub fn update_health_overlay_text(
    health_query: Query<&Health, With<Player>>,
    mut text_query: Query<&mut Text, With<HealthText>>
){
    let health = health_query.single();
    let mut text = text_query.single_mut();

    text.sections[0].value = format!("Health: {0:?} of {1:?}", health.current, health.full);
}