use bevy::prelude::*;

use crate::{
    enemy::{Enemy, spawn_enemy},
    game::{GameSet, RunProgress, RunState},
};

pub struct EncounterPlugin;

impl Plugin for EncounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_waves
                .in_set(GameSet::Spawn)
                .run_if(in_state(RunState::Playing)),
        );
    }
}

fn spawn_waves(
    mut commands: Commands,
    time: Res<Time>,
    mut progress: ResMut<RunProgress>,
    enemies: Query<Entity, With<Enemy>>,
) {
    if progress.has_pending_spawns() {
        progress.tick_spawn_timer(time.delta());
        if let Some(point) = progress.take_spawn_point() {
            spawn_enemy(&mut commands, point, progress.wave());
        }
        return;
    }

    if !enemies.is_empty() {
        return;
    }

    progress.tick_intermission(time.delta());
    if progress.intermission_finished() {
        progress.start_next_wave();
    }
}
