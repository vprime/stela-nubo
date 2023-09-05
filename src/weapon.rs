use bevy::prelude::*;

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shoot_weapons));
    }
}

#[derive(Component)]
pub struct Cannon(bool);

#[derive(Component)]
pub struct NextShot(f32);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct WeaponOptions {
    rate: f32,
    speed: f32,
    power: f32
}

fn shoot_weapons(
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&Cannon, &WeaponOptions, &mut NextShot)>
){
    let now = time.elapsed_seconds();
    for (cannon_enabled, options, mut next) in &mut query {
        if !cannon_enabled.0 || next.0 > now {
            continue;
        }
        println!("Bang!");
        next.0 = now + options.rate;
    }
}