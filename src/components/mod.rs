use bevy::prelude::*;
use fastrand::f32;
use crate::components::health::process_damage_to_health;
use crate::components::points::{damage_points, kill_points};
use crate::states::{GameStates, AppStates};
mod health;
mod points;

pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(Update,
                         (
                             process_damage_to_health,
                             kill_points,
                             damage_points
                         )
                             .run_if(in_state(GameStates::Playing))
                             .run_if(in_state(AppStates::Game)));
    }
}

#[derive(Component)]
pub struct Owner(pub Entity);

#[derive(Component)]
pub struct Health {
    pub full: f32,
    pub current: f32,
}

#[derive(Component)]
pub struct Score {
    pub current: f32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            current: 0.0
        }
    }
}

#[derive(Event)]
pub struct DamageEvent {
    pub subject: Entity,
    pub source: Entity,
    pub value: f32
}

#[derive(Event)]
pub struct DeathEvent {
    pub subject: Entity,
    pub source: Entity
}