use std::time::Duration;
use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;
use crate::health::DamageEvent;
use crate::states::{GameStates, AppStates};
use crate::util::{decay_after_lifetime, Lifetime};

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (
                shoot_weapons,
                decay_after_lifetime::<Bullet>,
                bullet_damage
            ).run_if(in_state(GameStates::Playing))
                .run_if(in_state(AppStates::Game)));
    }
}

#[derive(Component)]
pub struct Cannon(pub bool);

#[derive(Component)]
pub struct NextShot(pub f32);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct WeaponOptions {
    pub rate: f32,
    pub speed: f32,
    pub power: f32
}

#[derive(Bundle)]
pub struct WeaponBundle {
    pub weapon: Cannon,
    pub next_shot: NextShot,
    pub options: WeaponOptions
}

#[derive(Component)]
struct SpawnableHandles {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>
}

impl Default for WeaponBundle {
    fn default() -> Self {
        Self {
            weapon: Cannon(false),
            next_shot: NextShot(0.0),
            options: WeaponOptions {
                rate: 1.0,
                speed: 1.0,
                power: 1.0
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    commands.spawn(SpawnableHandles {
        mesh: meshes.add(Mesh::from(shape::Cube {size: 0.1})),
        material: materials.add(Color::rgb(0.95, 0.9, 0.8).into()),
    });
}

fn shoot_weapons(
    time: Res<Time>,
    handle_query: Query<&SpawnableHandles>,
    mut commands: Commands,
    mut query: Query<(&Cannon, &WeaponOptions, &Transform, &LinearVelocity, &mut NextShot)>
){
    let now = time.elapsed_seconds();
    let handles = handle_query.single();
    for (cannon_enabled, options, weapon_transform, weapon_velocity, mut next) in &mut query {
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
            LinearVelocity(spawn_velocity)
        ));

        next.0 = now + options.rate;
    }
}

fn bullet_damage(
    bullets: Query<Entity, With<Bullet>>,
    mut commands: Commands,
    mut collision_event: EventReader<CollisionStarted>,
    mut damage_event: EventWriter<DamageEvent>
){
    for CollisionStarted(entity1, entity2) in collision_event.iter() {
        if bullets.contains(*entity1) {
            damage_event.send(DamageEvent {
                subject: *entity2,
                value: 1.0
            });
            commands.entity(*entity1).despawn_recursive();
        }
        if bullets.contains(*entity2) {
            damage_event.send(DamageEvent {
                subject: *entity1,
                value: 1.0
            });
            commands.entity(*entity2).despawn_recursive();
        }
    }
}