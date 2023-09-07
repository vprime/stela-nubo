use bevy::prelude::*;


pub struct HealthPlugin;
impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .add_event::<DeathEvent>()
            .add_systems(Update, process_damage_to_health);
    }
}

#[derive(Component)]
pub struct Health {
    pub full: f32,
    pub current: f32,
}

#[derive(Event)]
pub struct DamageEvent {
    pub subject: Entity,
    pub value: f32
}

#[derive(Event)]
pub struct DeathEvent {
    pub subject: Entity
}

fn process_damage_to_health(
    mut damage_event: EventReader<DamageEvent>,
    mut death_event: EventWriter<DeathEvent>,
    mut query: Query<&mut Health>
){
    for damage in damage_event.iter() {
        if let Ok(mut subject_health) = query.get_mut(damage.subject){
            subject_health.current = (0.0f32).max(subject_health.current - damage.value);
            println!("Damage: {0} current: {1}", damage.value, subject_health.current);
            if (subject_health.current == 0.0){
                death_event.send(DeathEvent { subject: damage.subject});
            }
        }
    }
}

