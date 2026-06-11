use bevy::prelude::*;
const ATTACK_SOUND: &str = "sounds/attack.ogg";
const HIT_SOUND: &str = "sounds/hit.ogg";
const LEVEL_UP_SOUND: &str = "sounds/level_up.ogg";
#[cfg(test)]
const GAME_SFX_PATHS: [&str; 3] = [ATTACK_SOUND, HIT_SOUND, LEVEL_UP_SOUND];

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_sfx);
    }
}

#[derive(Resource)]
pub struct GameSfx {
    pub attack: Handle<AudioSource>,
    pub hit: Handle<AudioSource>,
    pub level_up: Handle<AudioSource>,
}

pub fn play_sfx(commands: &mut Commands, sound: &Handle<AudioSource>) {
    commands.spawn((AudioPlayer::new(sound.clone()), PlaybackSettings::DESPAWN));
}

fn load_sfx(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(GameSfx {
        attack: asset_server.load(ATTACK_SOUND),
        hit: asset_server.load(HIT_SOUND),
        level_up: asset_server.load(LEVEL_UP_SOUND),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::{audio::Decodable, prelude::AudioSource};
    use std::sync::Arc;

    #[test]
    fn bundled_sfx_are_declared_decodable_ogg_assets() {
        for path in GAME_SFX_PATHS {
            assert!(
                path.ends_with(".ogg"),
                "{path} must use Bevy's default Vorbis decoder"
            );
        }

        for bytes in [
            include_bytes!("../assets/sounds/attack.ogg").as_slice(),
            include_bytes!("../assets/sounds/hit.ogg").as_slice(),
            include_bytes!("../assets/sounds/level_up.ogg").as_slice(),
        ] {
            assert_eq!(&bytes[..4], b"OggS");
            let source = AudioSource {
                bytes: Arc::from(bytes),
            };
            let _decoder = source.decoder();
        }
    }
}
