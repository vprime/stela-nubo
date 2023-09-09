use bevy::prelude::*;
use queues::Queue;
use crate::arena::generation::{clean_up_map, damage_player, despawn_cubes, destroy_asteroids, spawn_from_queue, spawn_setup, worley_spawner};
use crate::states::{AppStates, GameStates};

pub mod generation;

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_setup)
            .add_systems(OnExit(AppStates::Game), clean_up_map)
            .add_systems(Update, (
                worley_spawner,
                despawn_cubes,
                spawn_from_queue,
                destroy_asteroids,
                damage_player
            )
                .run_if(in_state(GameStates::Playing))
                .run_if(in_state(AppStates::Game)));
    }
}
