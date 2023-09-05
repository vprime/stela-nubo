use bevy::prelude::*;

pub struct DestructiblesPlugin;

impl Plugin for DestructiblesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>();
    }
}

#[derive(Event)]
pub struct DamageEvent {
    pub subject: Entity,
    pub value: f32
}

