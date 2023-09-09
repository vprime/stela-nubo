use bevy::prelude::*;
pub fn intersecting(a_point: Vec3, b_bound: Vec3, radius: f32) -> bool {
    (a_point.x < b_bound.x + radius && a_point.x > b_bound.x - radius) &&
        (a_point.y < b_bound.y + radius && a_point.y > b_bound.y - radius) &&
        (a_point.z < b_bound.z + radius && a_point.z > b_bound.z - radius)
}

#[derive(Component)]
pub struct Lifetime {
    pub timer: Timer
}

pub fn decay_after_lifetime<T:Component>(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Lifetime), With<T>>
){
    for (entity, mut lifetime) in query.iter_mut() {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}


pub fn clean_up<T: Component>(
    mut commands: Commands,
    query: Query<Entity, With<T>>
){
    for item in query.iter() {
        commands.entity(item).despawn_recursive();
    }
}