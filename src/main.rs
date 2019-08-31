mod bundles;
mod card;
mod components;
mod math;
mod resources;
mod states;
mod systems;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    LogLevelFilter, LoggerConfig,
};

use bundles::SolitaireBundle;
use states::PlayState;

fn main() -> amethyst::Result<()> {
    let mut config = LoggerConfig::default();
    config.level_filter = LogLevelFilter::Debug;
    amethyst::start_logger(config);

    let app_root = application_root_dir()?;

    let assets = app_root.join("assets");
    let display_config = assets.join("display_config.ron");

    let input_bundle = InputBundle::<StringBindings>::new();

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(SolitaireBundle)?
        .with_bundle(input_bundle)?;

    let mut game = Application::new(assets, PlayState, game_data)?;
    game.run();

    Ok(())
}
