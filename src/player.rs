use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    game::{GameSet, RunState},
    shared::clamp_to_arena,
};

const MOVE_SPEED: f32 = 260.0;
const DASH_DISTANCE: f32 = 110.0;
const DASH_COOLDOWN: f32 = 1.0;
const DASH_IFRAME: f32 = 0.25;
const HIT_IFRAME: f32 = 0.18;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            move_player
                .in_set(GameSet::Player)
                .run_if(in_state(RunState::Playing)),
        );
    }
}

#[derive(Component)]
pub struct Player {
    hp: f32,
    max_hp: f32,
    damage: f32,
    level: u32,
    xp: u32,
    next_xp: u32,
    attack_cooldown: f32,
    dash_cooldown: f32,
    invulnerable: f32,
    facing: Vec2,
}

impl Player {
    pub const RADIUS: f32 = 18.0;

    pub fn hp(&self) -> f32 {
        self.hp
    }

    pub fn max_hp(&self) -> f32 {
        self.max_hp
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn xp(&self) -> u32 {
        self.xp
    }

    pub fn next_xp(&self) -> u32 {
        self.next_xp
    }

    pub fn damage(&self) -> f32 {
        self.damage
    }

    pub fn dash_cooldown(&self) -> f32 {
        self.dash_cooldown
    }

    pub fn facing(&self) -> Vec2 {
        self.facing.normalize_or_zero()
    }

    pub fn is_attack_ready(&self) -> bool {
        self.attack_cooldown <= 0.0
    }

    pub fn begin_attack(&mut self, cooldown: f32) {
        self.attack_cooldown = cooldown;
    }

    pub fn receive_hit(&mut self, damage: f32) -> bool {
        if self.invulnerable > 0.0 {
            return false;
        }

        self.hp = (self.hp - damage).max(0.0);
        self.invulnerable = HIT_IFRAME;
        self.hp <= 0.0
    }

    pub fn add_xp(&mut self, amount: u32) {
        self.xp += amount;
        while self.xp >= self.next_xp {
            self.xp -= self.next_xp;
            self.level += 1;
            self.next_xp += 3;
            self.max_hp += 8.0;
            self.hp = (self.hp + 24.0).min(self.max_hp);
            self.damage += 4.0;
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default();
    }

    fn tick_timers(&mut self, dt: f32) {
        self.attack_cooldown = (self.attack_cooldown - dt).max(0.0);
        self.dash_cooldown = (self.dash_cooldown - dt).max(0.0);
        self.invulnerable = (self.invulnerable - dt).max(0.0);
    }

    fn is_invulnerable(&self) -> bool {
        self.invulnerable > 0.0
    }
}

impl Default for Player {
    fn default() -> Self {
        Self {
            hp: 100.0,
            max_hp: 100.0,
            damage: 24.0,
            level: 1,
            xp: 0,
            next_xp: 5,
            attack_cooldown: 0.0,
            dash_cooldown: 0.0,
            invulnerable: 0.0,
            facing: Vec2::X,
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.25, 0.68, 0.92),
            Vec2::splat(Player::RADIUS * 2.0),
        ),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player::default(),
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Sprite)>,
) {
    let Ok((mut transform, mut player, mut sprite)) = player_query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    player.tick_timers(dt);
    sprite.color = player_color(player.is_invulnerable());

    update_facing_from_cursor(&mut player, &transform, &window, &camera);

    let movement = movement_input(&keyboard);
    if movement != Vec2::ZERO {
        transform.translation += (movement * MOVE_SPEED * dt).extend(0.0);
    }

    if keyboard.just_pressed(KeyCode::ShiftLeft)
        && movement != Vec2::ZERO
        && player.dash_cooldown <= 0.0
    {
        transform.translation += (movement * DASH_DISTANCE).extend(0.0);
        player.dash_cooldown = DASH_COOLDOWN;
        player.invulnerable = DASH_IFRAME;
    }

    clamp_to_arena(&mut transform.translation, Player::RADIUS);
}

fn movement_input(keyboard: &ButtonInput<KeyCode>) -> Vec2 {
    let mut movement = Vec2::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        movement.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        movement.x += 1.0;
    }
    movement.normalize_or_zero()
}

fn update_facing_from_cursor(
    player: &mut Player,
    transform: &Transform,
    window: &Query<&Window, With<PrimaryWindow>>,
    camera: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) {
    let Some(world_cursor) = cursor_world_position(window, camera) else {
        return;
    };

    let aim = world_cursor - transform.translation.truncate();
    if aim.length_squared() > 1.0 {
        player.facing = aim.normalize();
    }
}

fn cursor_world_position(
    window_query: &Query<&Window, With<PrimaryWindow>>,
    camera_query: &Query<(&Camera, &GlobalTransform), With<Camera2d>>,
) -> Option<Vec2> {
    let window = window_query.single().ok()?;
    let cursor = window.cursor_position()?;
    let (camera, camera_transform) = camera_query.single().ok()?;
    camera.viewport_to_world_2d(camera_transform, cursor).ok()
}

fn player_color(invulnerable: bool) -> Color {
    if invulnerable {
        Color::srgb(0.72, 0.9, 1.0)
    } else {
        Color::srgb(0.25, 0.68, 0.92)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xp_rolls_over_and_applies_level_reward() {
        let mut player = Player::default();

        player.add_xp(6);

        assert_eq!(player.level(), 2);
        assert_eq!(player.xp(), 1);
        assert_eq!(player.next_xp(), 8);
        assert!(player.damage() > Player::default().damage());
    }
}
