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
        app.init_resource::<UpgradeChoices>()
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                move_player
                    .in_set(GameSet::Player)
                    .run_if(in_state(RunState::Playing)),
            )
            .add_systems(
                Update,
                choose_upgrade
                    .in_set(GameSet::Player)
                    .run_if(in_state(RunState::LevelUp)),
            );
    }
}

const UPGRADE_CHOICES: [UpgradeKind; 3] = [
    UpgradeKind::SharpBlade,
    UpgradeKind::QuickBreath,
    UpgradeKind::WarmBlood,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UpgradeKind {
    SharpBlade,
    QuickBreath,
    WarmBlood,
}

impl UpgradeKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::SharpBlade => "Lâmina afiada: +6 dano",
            Self::QuickBreath => "Fôlego rápido: dash recarrega mais cedo",
            Self::WarmBlood => "Sangue quente: +18 vida máxima e cura",
        }
    }
}

#[derive(Resource)]
pub struct UpgradeChoices {
    choices: [UpgradeKind; 3],
}

impl UpgradeChoices {
    pub fn reset(&mut self) {
        self.choices = UPGRADE_CHOICES;
    }

    pub fn choices(&self) -> &[UpgradeKind; 3] {
        &self.choices
    }

    fn selected(&self, index: usize) -> Option<UpgradeKind> {
        self.choices.get(index).copied()
    }
}

impl Default for UpgradeChoices {
    fn default() -> Self {
        Self {
            choices: UPGRADE_CHOICES,
        }
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
    pending_upgrades: u32,
    dash_recharge: f32,
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

    pub fn dash_recharge(&self) -> f32 {
        self.dash_recharge
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
    pub fn is_dead(&self) -> bool {
        self.hp <= 0.0
    }

    pub fn pending_upgrades(&self) -> u32 {
        self.pending_upgrades
    }

    pub fn add_xp(&mut self, amount: u32) -> u32 {
        self.xp += amount;
        let mut levels_gained = 0;
        while self.xp >= self.next_xp {
            self.xp -= self.next_xp;
            self.level += 1;
            self.next_xp += 3;
            self.pending_upgrades += 1;
            levels_gained += 1;
        }
        levels_gained
    }

    pub fn consume_pending_upgrade(&mut self) -> bool {
        if self.pending_upgrades == 0 {
            return false;
        }

        self.pending_upgrades -= 1;
        true
    }

    pub fn apply_upgrade(&mut self, upgrade: UpgradeKind) {
        match upgrade {
            UpgradeKind::SharpBlade => {
                self.damage += 6.0;
            }
            UpgradeKind::QuickBreath => {
                self.dash_recharge = (self.dash_recharge - 0.12).max(0.45);
                self.dash_cooldown = self.dash_cooldown.min(self.dash_recharge);
            }
            UpgradeKind::WarmBlood => {
                self.max_hp += 18.0;
                self.hp = (self.hp + 18.0).min(self.max_hp);
            }
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
            dash_recharge: DASH_COOLDOWN,
            dash_cooldown: 0.0,
            invulnerable: 0.0,
            pending_upgrades: 0,
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
        player.dash_cooldown = player.dash_recharge;
        player.invulnerable = DASH_IFRAME;
    }

    clamp_to_arena(&mut transform.translation, Player::RADIUS);
}

fn choose_upgrade(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<RunState>>,
    mut choices: ResMut<UpgradeChoices>,
    mut player_query: Query<&mut Player>,
) {
    let index = if keyboard.just_pressed(KeyCode::Digit1) {
        Some(0)
    } else if keyboard.just_pressed(KeyCode::Digit2) {
        Some(1)
    } else if keyboard.just_pressed(KeyCode::Digit3) {
        Some(2)
    } else {
        None
    };

    let Some(index) = index else {
        return;
    };
    let Some(upgrade) = choices.selected(index) else {
        return;
    };
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };

    player.apply_upgrade(upgrade);
    player.consume_pending_upgrade();
    choices.reset();
    if player.pending_upgrades() == 0 {
        next_state.set(RunState::Playing);
    }
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
    fn xp_rolls_over_and_waits_for_upgrade_choice() {
        let mut player = Player::default();

        assert_eq!(player.add_xp(6), 1);

        assert_eq!(player.level(), 2);
        assert_eq!(player.xp(), 1);
        assert_eq!(player.next_xp(), 8);
        assert_eq!(player.pending_upgrades(), 1);
        assert_eq!(player.damage(), Player::default().damage());
    }

    #[test]
    fn xp_can_queue_multiple_upgrade_choices() {
        let mut player = Player::default();

        assert_eq!(player.add_xp(14), 2);

        assert_eq!(player.level(), 3);
        assert_eq!(player.xp(), 1);
        assert_eq!(player.next_xp(), 11);
        assert_eq!(player.pending_upgrades(), 2);
        assert!(player.consume_pending_upgrade());
        assert_eq!(player.pending_upgrades(), 1);
        assert!(player.consume_pending_upgrade());
        assert_eq!(player.pending_upgrades(), 0);
        assert!(!player.consume_pending_upgrade());
    }

    #[test]
    fn player_hit_invulnerability_prevents_double_damage() {
        let mut player = Player::default();

        assert!(!player.receive_hit(25.0));
        assert_eq!(player.hp(), 75.0);

        assert!(!player.receive_hit(25.0));
        assert_eq!(player.hp(), 75.0);

        player.tick_timers(HIT_IFRAME);
        assert!(player.receive_hit(100.0));
        assert_eq!(player.hp(), 0.0);
        assert!(player.is_dead());
    }

    #[test]
    fn upgrades_apply_explicit_rewards() {
        let mut player = Player::default();

        player.apply_upgrade(UpgradeKind::SharpBlade);
        assert_eq!(player.damage(), 30.0);

        player.apply_upgrade(UpgradeKind::WarmBlood);
        assert_eq!(player.max_hp(), 118.0);
        assert_eq!(player.hp(), 118.0);

        player.apply_upgrade(UpgradeKind::QuickBreath);
        assert!(player.dash_recharge() < 1.0);
    }

    #[test]
    fn quick_breath_has_a_floor() {
        let mut player = Player::default();

        for _ in 0..20 {
            player.apply_upgrade(UpgradeKind::QuickBreath);
        }

        assert_eq!(player.dash_recharge(), 0.45);
    }
}
