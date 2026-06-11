use bevy::prelude::*;

pub const ARENA_HALF_WIDTH: f32 = 600.0;
pub const ARENA_HALF_HEIGHT: f32 = 330.0;

pub const SPAWN_POINTS: [Vec2; 8] = [
    Vec2::new(-540.0, -280.0),
    Vec2::new(-560.0, 0.0),
    Vec2::new(-540.0, 280.0),
    Vec2::new(0.0, -300.0),
    Vec2::new(0.0, 300.0),
    Vec2::new(540.0, -280.0),
    Vec2::new(560.0, 0.0),
    Vec2::new(540.0, 280.0),
];

pub fn clamp_to_arena(position: &mut Vec3, padding: f32) {
    position.x = position
        .x
        .clamp(-ARENA_HALF_WIDTH + padding, ARENA_HALF_WIDTH - padding);
    position.y = position
        .y
        .clamp(-ARENA_HALF_HEIGHT + padding, ARENA_HALF_HEIGHT - padding);
}
