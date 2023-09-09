use bevy::prelude::*;
use crate::components::health::process_damage_to_health;
use crate::states::{GameStates, AppStates};
mod health;
pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(Update,
                         (process_damage_to_health)
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
