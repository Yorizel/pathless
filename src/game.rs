use bevy::prelude::*;

use crate::{
    combat::{AttackFx, CombatPlugin},
    encounter::EncounterPlugin,
    enemy::{Enemy, EnemyPlugin},
    player::{Player, PlayerPlugin},
    presentation::PresentationPlugin,
    shared::SPAWN_POINTS,
    world::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RunState>()
            .init_resource::<RunProgress>()
            .configure_sets(
                Update,
                (
                    GameSet::Restart,
                    GameSet::Spawn,
                    GameSet::Player,
                    GameSet::Combat,
                    GameSet::Cleanup,
                    GameSet::Enemy,
                    GameSet::Vfx,
                    GameSet::Ui,
                )
                    .chain(),
            )
            .add_plugins((
                WorldPlugin,
                PlayerPlugin,
                EncounterPlugin,
                CombatPlugin,
                EnemyPlugin,
                PresentationPlugin,
            ))
            .add_systems(
                Update,
                restart_from_game_over
                    .in_set(GameSet::Restart)
                    .run_if(in_state(RunState::GameOver)),
            );
    }
}

#[derive(States, Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum RunState {
    #[default]
    Playing,
    GameOver,
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum GameSet {
    Restart,
    Spawn,
    Player,
    Combat,
    Cleanup,
    Enemy,
    Vfx,
    Ui,
}

#[derive(Resource)]
pub struct RunProgress {
    wave: u32,
    kills: u32,
    spawn_index: usize,
    enemies_left_to_spawn: u32,
    spawn_timer: Timer,
    intermission_timer: Timer,
}

impl RunProgress {
    pub fn wave(&self) -> u32 {
        self.wave
    }

    pub fn kills(&self) -> u32 {
        self.kills
    }

    pub fn enemies_left_to_spawn(&self) -> u32 {
        self.enemies_left_to_spawn
    }

    pub fn has_pending_spawns(&self) -> bool {
        self.enemies_left_to_spawn > 0
    }

    pub fn tick_spawn_timer(&mut self, delta: std::time::Duration) {
        self.spawn_timer.tick(delta);
    }

    pub fn tick_intermission(&mut self, delta: std::time::Duration) {
        self.intermission_timer.tick(delta);
    }

    pub fn intermission_finished(&self) -> bool {
        self.intermission_timer.just_finished()
    }

    pub fn take_spawn_point(&mut self) -> Option<Vec2> {
        if self.enemies_left_to_spawn == 0 || !self.spawn_timer.just_finished() {
            return None;
        }

        let point = SPAWN_POINTS[self.spawn_index % SPAWN_POINTS.len()];
        self.spawn_index += 1;
        self.enemies_left_to_spawn -= 1;
        Some(point)
    }

    pub fn start_next_wave(&mut self) {
        self.wave += 1;
        self.enemies_left_to_spawn = 3 + self.wave * 2;
        self.spawn_timer.reset();
        self.intermission_timer.reset();
    }

    pub fn record_kill(&mut self) {
        self.kills += 1;
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

impl Default for RunProgress {
    fn default() -> Self {
        Self {
            wave: 0,
            kills: 0,
            spawn_index: 0,
            enemies_left_to_spawn: 0,
            spawn_timer: Timer::from_seconds(0.55, TimerMode::Repeating),
            intermission_timer: Timer::from_seconds(0.8, TimerMode::Once),
        }
    }
}

fn restart_from_game_over(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<RunState>>,
    mut progress: ResMut<RunProgress>,
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Sprite)>,
    enemies: Query<Entity, With<Enemy>>,
    fx: Query<Entity, With<AttackFx>>,
) {
    if !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    progress.reset();
    next_state.set(RunState::Playing);

    if let Ok((mut transform, mut player, mut sprite)) = player_query.single_mut() {
        transform.translation = Vec3::new(0.0, 0.0, 1.0);
        player.reset();
        sprite.color = Color::srgb(0.25, 0.68, 0.92);
    }

    for entity in &enemies {
        commands.entity(entity).despawn();
    }
    for entity in &fx {
        commands.entity(entity).despawn();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawn_points_are_taken_only_when_spawn_timer_finishes() {
        let mut progress = RunProgress::default();
        progress.start_next_wave();

        assert!(progress.take_spawn_point().is_none());

        progress.tick_spawn_timer(std::time::Duration::from_secs_f32(0.55));
        assert!(progress.take_spawn_point().is_some());
        assert_eq!(progress.enemies_left_to_spawn(), 4);
    }
}
