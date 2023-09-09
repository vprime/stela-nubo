use std::time::Duration;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::components::{DamageEvent, Owner};
use crate::spawnable::{Bullet, Cannon, NextShot, SpawnableHandles, WeaponBundle, WeaponOptions};
use crate::util::{Lifetime};


pub fn shoot_weapons(
    time: Res<Time>,
    handle_query: Query<&SpawnableHandles>,
    mut commands: Commands,
    mut query: Query<(Entity, &Cannon, &WeaponOptions, &Transform, &LinearVelocity, &mut NextShot)>
){
    let now = time.elapsed_seconds();
    let handles = handle_query.single();
    for (entity, cannon_enabled, options, weapon_transform, weapon_velocity, mut next) in &mut query {
        if !cannon_enabled.0 || next.0 > now {
            continue;
        }

        let spawn_position = weapon_transform.translation + weapon_transform.forward();
        let spawn_velocity = (weapon_transform.forward() * options.speed) + weapon_velocity.0;
        commands.spawn((PbrBundle {
            mesh: handles.mesh.clone(),
            material: handles.material.clone(),
            ..default()
        }, Bullet,
            Lifetime {
                timer: Timer::new(Duration::from_secs(3), TimerMode::Once)
            },
            Collider::cuboid(0.1, 0.1, 0.1),
            Mass(options.power),
            RigidBody::Dynamic,
            Position(spawn_position),
            LinearVelocity(spawn_velocity),
            Owner(entity)
        ));

        next.0 = now + options.rate;
    }
}

pub fn bullet_damage(
    bullets: Query<(Entity, &Owner), With<Bullet>>,
    mut commands: Commands,
    mut collision_event: EventReader<CollisionStarted>,
    mut damage_event: EventWriter<DamageEvent>
){
    for CollisionStarted(entity1, entity2) in collision_event.iter() {
        if let Ok((entity, owner)) = bullets.get(*entity1) {
            damage_event.send(DamageEvent {
                subject: *entity2,
                source: owner.0,
                value: 1.0
            });
            commands.entity(*entity1).despawn_recursive();
        }
        if let Ok((entity, owner)) = bullets.get(*entity2) {
            damage_event.send(DamageEvent {
                subject: *entity1,
                source: owner.0,
                value: 1.0
            });
            commands.entity(*entity2).despawn_recursive();
        }
    }
}