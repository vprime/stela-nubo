use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use crate::effects::explosion::{spawn_explosions, update_time_for_particles_material};
use crate::states::{AppStates, GameStates};
use crate::util::{clean_up, decay_after_lifetime};

mod explosion;

pub struct EffectsPlugin;

impl Plugin for EffectsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ExplosionEvent>()
            .add_systems(OnExit(AppStates::Game), clean_up::<Explosion>)
            .add_plugins((
                MaterialPlugin::<ParticlesMaterial>::default(),
            ))
            .add_systems(Update, (
                update_time_for_particles_material,
                spawn_explosions,
                decay_after_lifetime::<Explosion>,
            )
                .run_if(in_state(GameStates::Playing))
                .run_if(in_state(AppStates::Game)));
    }
}


#[derive(Event)]
pub struct ExplosionEvent {
    pub position: Vec3,
    pub power: f32
}

#[derive(Component)]
pub struct Explodeable;

#[derive(Component)]
pub struct Explosion;

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "00cfdf10-7270-490d-8841-cf08b476303a"]
pub struct ParticlesMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    start: f32,
    #[uniform(2)]
    end: f32,
}

#[derive(Debug, Copy, Clone)]
struct Particles {
    num_particles: u32,
}


impl Material for ParticlesMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/particle.wgsl".into()
    }
    fn vertex_shader() -> ShaderRef {
        "shaders/particle.wgsl".into()
    }
    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }
}