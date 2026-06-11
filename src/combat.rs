use bevy::prelude::*;

use crate::{
    enemy::Enemy,
    game::{GameSet, RunProgress, RunState},
    player::Player,
    sfx::{GameSfx, play_sfx},
};

const ATTACK_RANGE: f32 = 84.0;
const ATTACK_ARC_DOT: f32 = 0.15;
const ATTACK_COOLDOWN: f32 = 0.32;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            player_attack
                .in_set(GameSet::Combat)
                .run_if(in_state(RunState::Playing)),
        )
        .add_systems(
            Update,
            cleanup_dead
                .in_set(GameSet::Cleanup)
                .run_if(in_state(RunState::Playing)),
        )
        .add_systems(Update, animate_attack_fx.in_set(GameSet::Vfx));
    }
}

#[derive(Component)]
pub struct AttackFx {
    timer: Timer,
}

fn player_attack(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut enemies: Query<(&Transform, &mut Enemy)>,
    sfx: Option<Res<GameSfx>>,
) {
    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };

    let attacking = mouse.pressed(MouseButton::Left) || keyboard.pressed(KeyCode::Space);
    if !attacking || !player.is_attack_ready() {
        return;
    }

    player.begin_attack(ATTACK_COOLDOWN);
    let origin = player_transform.translation.truncate();
    let facing = player.facing();
    if let Some(sfx) = sfx.as_ref() {
        play_sfx(&mut commands, &sfx.attack);
    }
    spawn_slash_fx(&mut commands, origin, facing);

    for (enemy_transform, mut enemy) in &mut enemies {
        if attack_hits_enemy(origin, facing, enemy_transform.translation.truncate()) {
            enemy.take_damage(player.damage());
            if let Some(sfx) = sfx.as_ref() {
                play_sfx(&mut commands, &sfx.hit);
            }
        }
    }
}

fn cleanup_dead(
    mut commands: Commands,
    mut progress: ResMut<RunProgress>,
    mut player_query: Query<&mut Player>,
    enemies: Query<(Entity, &Enemy)>,
    sfx: Option<Res<GameSfx>>,
) {
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };

    for (entity, enemy) in &enemies {
        if !enemy.is_dead() {
            continue;
        }

        commands.entity(entity).despawn();
        progress.record_kill();
        if player.add_xp(enemy.xp_reward()) > 0 {
            if let Some(sfx) = sfx.as_ref() {
                play_sfx(&mut commands, &sfx.level_up);
            }
        }
    }
}

fn animate_attack_fx(
    mut commands: Commands,
    time: Res<Time>,
    mut fx_query: Query<(Entity, &mut AttackFx, &mut Sprite)>,
) {
    for (entity, mut fx, mut sprite) in &mut fx_query {
        fx.timer.tick(time.delta());
        let alpha = 1.0 - fx.timer.fraction();
        sprite.color = sprite.color.with_alpha(alpha.max(0.0));
        if fx.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn spawn_slash_fx(commands: &mut Commands, origin: Vec2, facing: Vec2) {
    let slash_position = origin + facing * 48.0;
    commands.spawn((
        Sprite::from_color(Color::srgba(0.85, 0.95, 1.0, 0.75), Vec2::new(82.0, 18.0)),
        Transform::from_xyz(slash_position.x, slash_position.y, 2.0)
            .with_rotation(Quat::from_rotation_z(facing.y.atan2(facing.x))),
        AttackFx {
            timer: Timer::from_seconds(0.09, TimerMode::Once),
        },
    ));
}

fn attack_hits_enemy(origin: Vec2, facing: Vec2, enemy_position: Vec2) -> bool {
    let to_enemy = enemy_position - origin;
    let distance = to_enemy.length();
    let in_range = distance <= ATTACK_RANGE + Enemy::RADIUS;
    let in_arc = distance <= 0.1 || facing.dot(to_enemy / distance) >= ATTACK_ARC_DOT;
    in_range && in_arc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attack_hits_enemies_in_front_inside_range() {
        let origin = Vec2::ZERO;
        let facing = Vec2::X;

        assert!(attack_hits_enemy(
            origin,
            facing,
            Vec2::new(ATTACK_RANGE + Enemy::RADIUS, 0.0)
        ));
    }

    #[test]
    fn attack_ignores_enemies_behind_or_outside_range() {
        let origin = Vec2::ZERO;
        let facing = Vec2::X;

        assert!(!attack_hits_enemy(origin, facing, Vec2::new(-20.0, 0.0)));
        assert!(!attack_hits_enemy(
            origin,
            facing,
            Vec2::new(ATTACK_RANGE + Enemy::RADIUS + 0.1, 0.0)
        ));
    }
}
