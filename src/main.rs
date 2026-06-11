use bevy::{prelude::*, window::PrimaryWindow};

const ARENA_HALF_WIDTH: f32 = 600.0;
const ARENA_HALF_HEIGHT: f32 = 330.0;
const PLAYER_SPEED: f32 = 260.0;
const DASH_DISTANCE: f32 = 110.0;
const DASH_COOLDOWN: f32 = 1.0;
const DASH_IFRAME: f32 = 0.25;
const ATTACK_RANGE: f32 = 84.0;
const ATTACK_ARC_DOT: f32 = 0.15;
const ATTACK_COOLDOWN: f32 = 0.32;
const ENEMY_RADIUS: f32 = 20.0;
const PLAYER_RADIUS: f32 = 18.0;
const SPAWN_POINTS: [Vec2; 8] = [
    Vec2::new(-540.0, -280.0),
    Vec2::new(-560.0, 0.0),
    Vec2::new(-540.0, 280.0),
    Vec2::new(0.0, -300.0),
    Vec2::new(0.0, 300.0),
    Vec2::new(540.0, -280.0),
    Vec2::new(560.0, 0.0),
    Vec2::new(540.0, 280.0),
];

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.05, 0.06, 0.09)))
        .insert_resource(Game::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pathless".into(),
                resolution: (1280, 720).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                restart_run,
                spawn_waves,
                move_player,
                player_attack,
                move_enemies,
                cleanup_dead,
                animate_fx,
                update_hud,
            )
                .chain(),
        )
        .run();
}

#[derive(Resource)]
struct Game {
    wave: u32,
    kills: u32,
    spawn_index: usize,
    enemies_left_to_spawn: u32,
    spawn_timer: Timer,
    intermission_timer: Timer,
    game_over: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            wave: 0,
            kills: 0,
            spawn_index: 0,
            enemies_left_to_spawn: 0,
            spawn_timer: Timer::from_seconds(0.55, TimerMode::Repeating),
            intermission_timer: Timer::from_seconds(0.8, TimerMode::Once),
            game_over: false,
        }
    }
}

#[derive(Component)]
struct Player {
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

#[derive(Component)]
struct Enemy {
    hp: f32,
    speed: f32,
    damage: f32,
    xp: u32,
    attack_timer: Timer,
}

impl Enemy {
    fn for_wave(wave: u32) -> Self {
        let wave_bonus = wave.saturating_sub(1) as f32;
        Self {
            hp: 24.0 + wave_bonus * 5.0,
            speed: 78.0 + wave_bonus.min(6.0) * 4.0,
            damage: 9.0 + wave_bonus * 0.8,
            xp: 1,
            attack_timer: Timer::from_seconds(0.65, TimerMode::Repeating),
        }
    }
}

#[derive(Component)]
struct HudText;

#[derive(Component)]
struct AttackFx {
    timer: Timer,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.075, 0.085, 0.115),
            Vec2::new(ARENA_HALF_WIDTH * 2.0, ARENA_HALF_HEIGHT * 2.0),
        ),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    for (translation, size) in [
        (
            Vec3::new(0.0, ARENA_HALF_HEIGHT, -9.0),
            Vec2::new(ARENA_HALF_WIDTH * 2.0, 4.0),
        ),
        (
            Vec3::new(0.0, -ARENA_HALF_HEIGHT, -9.0),
            Vec2::new(ARENA_HALF_WIDTH * 2.0, 4.0),
        ),
        (
            Vec3::new(ARENA_HALF_WIDTH, 0.0, -9.0),
            Vec2::new(4.0, ARENA_HALF_HEIGHT * 2.0),
        ),
        (
            Vec3::new(-ARENA_HALF_WIDTH, 0.0, -9.0),
            Vec2::new(4.0, ARENA_HALF_HEIGHT * 2.0),
        ),
    ] {
        commands.spawn((
            Sprite::from_color(Color::srgba(0.35, 0.45, 0.62, 0.55), size),
            Transform::from_translation(translation),
        ));
    }

    commands.spawn((
        Sprite::from_color(Color::srgb(0.25, 0.68, 0.92), Vec2::splat(34.0)),
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player::default(),
    ));

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16.0),
                top: Val::Px(12.0),
                padding: UiRect::all(Val::Px(12.0)),
                border: UiRect::all(Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.04, 0.05, 0.07, 0.82)),
            BorderColor::all(Color::srgba(0.45, 0.55, 0.75, 0.7)),
        ))
        .with_children(|hud| {
            hud.spawn((
                Text::new(""),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.92, 0.98)),
                HudText,
            ));
        });
}

fn spawn_waves(
    mut commands: Commands,
    time: Res<Time>,
    mut game: ResMut<Game>,
    enemies: Query<Entity, With<Enemy>>,
) {
    if game.game_over {
        return;
    }

    if game.enemies_left_to_spawn == 0 && enemies.is_empty() {
        game.intermission_timer.tick(time.delta());
        if game.intermission_timer.just_finished() {
            game.wave += 1;
            game.enemies_left_to_spawn = 3 + game.wave * 2;
            game.spawn_timer.reset();
        }
        return;
    }

    if game.enemies_left_to_spawn == 0 {
        return;
    }

    game.spawn_timer.tick(time.delta());
    if !game.spawn_timer.just_finished() {
        return;
    }

    let point = SPAWN_POINTS[game.spawn_index % SPAWN_POINTS.len()];
    game.spawn_index += 1;
    game.enemies_left_to_spawn -= 1;

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.82, 0.2, 0.18),
            Vec2::splat(ENEMY_RADIUS * 2.0),
        ),
        Transform::from_xyz(point.x, point.y, 1.0),
        Enemy::for_wave(game.wave),
    ));
}

fn move_player(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Sprite)>,
    game: Res<Game>,
) {
    let Ok((mut transform, mut player, mut sprite)) = player_query.single_mut() else {
        return;
    };

    let dt = time.delta_secs();
    player.attack_cooldown = (player.attack_cooldown - dt).max(0.0);
    player.dash_cooldown = (player.dash_cooldown - dt).max(0.0);
    player.invulnerable = (player.invulnerable - dt).max(0.0);

    sprite.color = if player.invulnerable > 0.0 {
        Color::srgb(0.72, 0.9, 1.0)
    } else {
        Color::srgb(0.25, 0.68, 0.92)
    };

    if game.game_over {
        return;
    }

    if let Some(world_cursor) = cursor_world_position(&window, &camera) {
        let aim = world_cursor - transform.translation.truncate();
        if aim.length_squared() > 1.0 {
            player.facing = aim.normalize();
        }
    }

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

    let movement = movement.normalize_or_zero();
    if movement != Vec2::ZERO {
        transform.translation += (movement * PLAYER_SPEED * dt).extend(0.0);
    }

    if keyboard.just_pressed(KeyCode::ShiftLeft)
        && movement != Vec2::ZERO
        && player.dash_cooldown <= 0.0
    {
        transform.translation += (movement * DASH_DISTANCE).extend(0.0);
        player.dash_cooldown = DASH_COOLDOWN;
        player.invulnerable = DASH_IFRAME;
    }

    clamp_to_arena(&mut transform.translation, PLAYER_RADIUS);
}

fn player_attack(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut enemies: Query<(&Transform, &mut Enemy)>,
    game: Res<Game>,
) {
    if game.game_over {
        return;
    }

    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };

    let attacking = mouse.pressed(MouseButton::Left) || keyboard.pressed(KeyCode::Space);
    if !attacking || player.attack_cooldown > 0.0 {
        return;
    }

    player.attack_cooldown = ATTACK_COOLDOWN;
    let origin = player_transform.translation.truncate();
    let facing = player.facing.normalize_or_zero();
    let slash_position = origin + facing * 48.0;

    commands.spawn((
        Sprite::from_color(Color::srgba(0.85, 0.95, 1.0, 0.75), Vec2::new(82.0, 18.0)),
        Transform::from_xyz(slash_position.x, slash_position.y, 2.0)
            .with_rotation(Quat::from_rotation_z(facing.y.atan2(facing.x))),
        AttackFx {
            timer: Timer::from_seconds(0.09, TimerMode::Once),
        },
    ));

    for (enemy_transform, mut enemy) in &mut enemies {
        let to_enemy = enemy_transform.translation.truncate() - origin;
        let distance = to_enemy.length();
        let in_range = distance <= ATTACK_RANGE + ENEMY_RADIUS;
        let in_arc = distance <= 0.1 || facing.dot(to_enemy / distance) >= ATTACK_ARC_DOT;
        if in_range && in_arc {
            enemy.hp -= player.damage;
        }
    }
}

fn move_enemies(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut player_query: Query<(&Transform, &mut Player), Without<Enemy>>,
    mut enemies: Query<(&mut Transform, &mut Enemy), Without<Player>>,
) {
    if game.game_over {
        return;
    }

    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };
    let player_position = player_transform.translation.truncate();
    let dt = time.delta_secs();

    for (mut enemy_transform, mut enemy) in &mut enemies {
        enemy.attack_timer.tick(time.delta());
        let enemy_position = enemy_transform.translation.truncate();
        let to_player = player_position - enemy_position;
        let distance = to_player.length();

        if distance > PLAYER_RADIUS + ENEMY_RADIUS {
            let step = to_player.normalize_or_zero() * enemy.speed * dt;
            enemy_transform.translation += step.extend(0.0);
            clamp_to_arena(&mut enemy_transform.translation, ENEMY_RADIUS);
            continue;
        }

        if enemy.attack_timer.just_finished() && player.invulnerable <= 0.0 {
            player.hp -= enemy.damage;
            player.invulnerable = 0.18;
        }
    }

    if player.hp <= 0.0 {
        player.hp = 0.0;
        game.game_over = true;
    }
}

fn cleanup_dead(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut player_query: Query<&mut Player>,
    enemies: Query<(Entity, &Enemy)>,
) {
    let Ok(mut player) = player_query.single_mut() else {
        return;
    };

    for (entity, enemy) in &enemies {
        if enemy.hp > 0.0 {
            continue;
        }

        commands.entity(entity).despawn();
        game.kills += 1;
        player.xp += enemy.xp;
    }

    while player.xp >= player.next_xp {
        player.xp -= player.next_xp;
        player.level += 1;
        player.next_xp += 3;
        player.max_hp += 8.0;
        player.hp = (player.hp + 24.0).min(player.max_hp);
        player.damage += 4.0;
    }
}

fn animate_fx(
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

fn update_hud(
    game: Res<Game>,
    player_query: Query<&Player>,
    mut hud_query: Query<&mut Text, With<HudText>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut text) = hud_query.single_mut() else {
        return;
    };

    let status = if game.game_over {
        "GAME OVER — pressione R para reiniciar"
    } else if game.enemies_left_to_spawn == 0 {
        "limpe a arena"
    } else {
        "sobreviva"
    };

    text.0 = format!(
        "Pathless\nWave {}  Kills {}\nVida {:.0}/{:.0}  Nível {}  XP {}/{}\nAtaque {:.0}  Dash {:.1}s\n{}",
        game.wave,
        game.kills,
        player.hp,
        player.max_hp,
        player.level,
        player.xp,
        player.next_xp,
        player.damage,
        player.dash_cooldown,
        status
    );
}

fn restart_run(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Sprite)>,
    enemies: Query<Entity, With<Enemy>>,
    fx: Query<Entity, With<AttackFx>>,
) {
    if !game.game_over || !keyboard.just_pressed(KeyCode::KeyR) {
        return;
    }

    *game = Game::default();
    if let Ok((mut transform, mut player, mut sprite)) = player_query.single_mut() {
        transform.translation = Vec3::new(0.0, 0.0, 1.0);
        *player = Player::default();
        sprite.color = Color::srgb(0.25, 0.68, 0.92);
    }

    for entity in &enemies {
        commands.entity(entity).despawn();
    }
    for entity in &fx {
        commands.entity(entity).despawn();
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

fn clamp_to_arena(position: &mut Vec3, padding: f32) {
    position.x = position
        .x
        .clamp(-ARENA_HALF_WIDTH + padding, ARENA_HALF_WIDTH - padding);
    position.y = position
        .y
        .clamp(-ARENA_HALF_HEIGHT + padding, ARENA_HALF_HEIGHT - padding);
}
