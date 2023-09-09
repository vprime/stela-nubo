use bevy::prelude::*;
use crate::spawnable::gun::{bullet_damage, shoot_weapons};
use crate::states::{AppStates, GameStates};
use crate::util::{clean_up, decay_after_lifetime};

mod gun;
pub struct SpawnablesPlugin;

impl Plugin for SpawnablesPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(OnExit(AppStates::Game), clean_up::<Bullet>)
            .add_systems(Update, (
                shoot_weapons,
                decay_after_lifetime::<Bullet>,
                bullet_damage
            ).run_if(in_state(GameStates::Playing))
                .run_if(in_state(AppStates::Game)));
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
pub struct SpawnableHandles {
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