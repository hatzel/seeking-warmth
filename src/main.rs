use log::warn;

use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::utils::application_root_dir;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawFlat2D, Event, Pipeline,
                         RenderBundle, Stage, VirtualKeyCode};
use std::time::Duration;

mod seeking_warmth;

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
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;
    let mut game = Application::build(assets_dir, seeking_warmth::SeekingWarmth)?
        .with_frame_limit(
            FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(2)),
            144,
        )
        .build(game_data)?;
    game.run();

    Ok(())

}