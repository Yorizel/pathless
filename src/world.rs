use bevy::prelude::*;

use crate::shared::{ARENA_HALF_HEIGHT, ARENA_HALF_WIDTH};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.075, 0.085, 0.115),
            Vec2::new(ARENA_HALF_WIDTH * 2.0, ARENA_HALF_HEIGHT * 2.0),
        ),
        Transform::from_xyz(0.0, 0.0, -10.0),
    ));

    for (translation, size) in arena_walls() {
        commands.spawn((
            Sprite::from_color(Color::srgba(0.35, 0.45, 0.62, 0.55), size),
            Transform::from_translation(translation),
        ));
    }
}

fn arena_walls() -> [(Vec3, Vec2); 4] {
    [
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
    ]
}
