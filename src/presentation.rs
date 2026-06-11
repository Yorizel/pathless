use bevy::prelude::*;

use crate::{
    game::{GameSet, RunProgress, RunState},
    player::Player,
};

pub struct PresentationPlugin;

impl Plugin for PresentationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud)
            .add_systems(Update, update_hud.in_set(GameSet::Ui));
    }
}

#[derive(Component)]
struct HudText;

fn spawn_hud(mut commands: Commands) {
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

fn update_hud(
    progress: Res<RunProgress>,
    run_state: Res<State<RunState>>,
    player_query: Query<&Player>,
    mut hud_query: Query<&mut Text, With<HudText>>,
) {
    let Ok(player) = player_query.single() else {
        return;
    };
    let Ok(mut text) = hud_query.single_mut() else {
        return;
    };

    let status = match run_state.get() {
        RunState::GameOver => "GAME OVER — pressione R para reiniciar",
        RunState::Playing if progress.enemies_left_to_spawn() == 0 => "limpe a arena",
        RunState::Playing => "sobreviva",
    };

    text.0 = format!(
        "Pathless\nWave {}  Kills {}\nVida {:.0}/{:.0}  Nível {}  XP {}/{}\nAtaque {:.0}  Dash {:.1}s\n{}",
        progress.wave(),
        progress.kills(),
        player.hp(),
        player.max_hp(),
        player.level(),
        player.xp(),
        player.next_xp(),
        player.damage(),
        player.dash_cooldown(),
        status
    );
}
