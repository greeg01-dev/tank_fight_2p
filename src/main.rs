#[windows_subsystem = "windows"]

use amethyst::{
    prelude::*,
    renderer::{
        types::DefaultBackend
    },
    utils::application_root_dir
};
use amethyst::renderer::{RenderFlat2D, RenderingBundle, RenderToWindow};

mod state;
use state::GamePlayState;

pub const ARENA_WIDTH: f32 = 1000.0;
pub const ARENA_HEIGHT: f32 = 500.0;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let assets_dir = app_root.join("assets");

    let game_data = GameDataBuilder::new()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([1.0, 1.0, 1.0, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
        )?;
    let mut game = Application::new(assets_dir, GamePlayState::new(), game_data)?;
    game.run();
    Ok(())
}