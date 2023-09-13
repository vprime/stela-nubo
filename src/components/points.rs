use bevy::prelude::*;
use crate::components::{DamageEvent, DeathEvent, Score};

pub fn kill_points(
    mut point_trackers: Query<&mut Score>,
    mut event_reader: EventReader<DeathEvent>
){
    for event in event_reader.iter() {
        if let Ok(mut tracker) = point_trackers.get_mut(event.source){
            tracker.current += 10.0;
        }
    }
}

pub fn damage_points(
    mut point_trackers: Query<&mut Score>,
    mut event_reader: EventReader<DamageEvent>
){
    for event in event_reader.iter() {
        if let Ok(mut tracker) = point_trackers.get_mut(event.source){
            tracker.current += 1.0;
        }
    }
}