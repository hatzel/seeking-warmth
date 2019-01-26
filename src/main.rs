use log::warn;

use amethyst::{
    audio::AudioBundle,
    core::{frame_limiter::FrameRateLimitStrategy, transform::TransformBundle},
    ecs::prelude::{Component, DenseVecStorage},
    input::InputBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    ui::{DrawUi, UiBundle},
    utils::application_root_dir,
};
use std::time::Duration;

mod seeking_warmth;

pub const PLAYER_HEIGHT: f32 = 34.0;
pub const PLAYER_WIDTH: f32 = 19.0;
pub const PLAYER_VELOCITY: f32 = 10.0;

pub struct Player {
    pub velocity: f32,
    pub width: f32,
    pub height: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            velocity: 1.0,
            width: 1.0,
            height: 1.0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    warn!("LOL");

    let root_dir = application_root_dir();

    let config_path = format!("{}/resources/display_config.ron", root_dir);

    let assets_dir = format!("{}/assets", root_dir);

    let config = DisplayConfig::load(&config_path);

    let pipe = Pipeline::build()
        .with_stage(
            Stage::with_backbuffer()
                .clear_target([1.0, 1.0, 1.0, 1.0], 1.0)
                .with_pass(DrawFlat2D::new()),
        );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::build(assets_dir, seeking_warmth::SeekingWarmth)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;
    game.run();

    Ok(())

}