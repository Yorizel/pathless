use bevy::prelude::*;

use crate::{
    game::{GameSet, RunState},
    player::Player,
    shared::clamp_to_arena,
};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_enemies
                .in_set(GameSet::Enemy)
                .run_if(in_state(RunState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Enemy {
    hp: f32,
    speed: f32,
    damage: f32,
    xp: u32,
    attack_timer: Timer,
}

impl Enemy {
    pub const RADIUS: f32 = 20.0;

    pub fn for_wave(wave: u32) -> Self {
        let wave_bonus = wave.saturating_sub(1) as f32;
        Self {
            hp: 24.0 + wave_bonus * 5.0,
            speed: 78.0 + wave_bonus.min(6.0) * 4.0,
            damage: 9.0 + wave_bonus * 0.8,
            xp: 1,
            attack_timer: Timer::from_seconds(0.65, TimerMode::Repeating),
        }
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.hp -= damage;
    }

    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }

    pub fn xp_reward(&self) -> u32 {
        self.xp
    }

    fn damage(&self) -> f32 {
        self.damage
    }

    fn speed(&self) -> f32 {
        self.speed
    }

    fn can_attack(&mut self, delta: std::time::Duration) -> bool {
        self.attack_timer.tick(delta);
        self.attack_timer.just_finished()
    }
}

pub fn spawn_enemy(commands: &mut Commands, position: Vec2, wave: u32) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.82, 0.2, 0.18),
            Vec2::splat(Enemy::RADIUS * 2.0),
        ),
        Transform::from_xyz(position.x, position.y, 1.0),
        Enemy::for_wave(wave),
    ));
}

fn move_enemies(
    time: Res<Time>,
    mut next_state: ResMut<NextState<RunState>>,
    mut player_query: Query<(&Transform, &mut Player), Without<Enemy>>,
    mut enemies: Query<(&mut Transform, &mut Enemy), Without<Player>>,
) {
    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };
    let player_position = player_transform.translation.truncate();
    let dt = time.delta_secs();

    for (mut enemy_transform, mut enemy) in &mut enemies {
        let enemy_position = enemy_transform.translation.truncate();
        let to_player = player_position - enemy_position;
        let distance = to_player.length();

        if distance > Player::RADIUS + Enemy::RADIUS {
            let step = to_player.normalize_or_zero() * enemy.speed() * dt;
            enemy_transform.translation += step.extend(0.0);
            clamp_to_arena(&mut enemy_transform.translation, Enemy::RADIUS);
            continue;
        }

        if enemy.can_attack(time.delta()) && player.receive_hit(enemy.damage()) {
            next_state.set(RunState::GameOver);
        }
    }
}
