#![windows_subsystem = "windows"]

// use dependencies
use amethyst::{prelude::*, renderer::{
    types::DefaultBackend
}, utils::application_root_dir};
use amethyst::core::TransformBundle;
use amethyst::input::StringBindings;
use amethyst::renderer::{RenderFlat2D, RenderingBundle, RenderToWindow};
use amethyst::ui::RenderUi;
use amethyst::window::DisplayConfig;

//mod files
mod state;
mod system;
mod entities;

use state::GamePlayState;

pub const ARENA_WIDTH: f32 = 1000.0; // Application's width
pub const ARENA_HEIGHT: f32 = 500.0; // Application's height

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    // define paths
    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("assets");

    // define bundles
    let input_bundle = amethyst::input::InputBundle::<StringBindings>::new();
    let ui_bundle = amethyst::ui::UiBundle::<StringBindings>::new();

    // define game's data
    let game_data = GameDataBuilder::new()
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(DisplayConfig {
                        title: "Tank Fight 2p".to_string(),
                        dimensions: Some((1000, 500)),
                        icon: Some(app_root.join("assets").join("icon").join("icon_image.png")),
                        resizable: false,
                        ..Default::default()
                    })
                        .with_clear([1.0, 1.0, 1.0, 1.0])
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
        )?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with_bundle(ui_bundle)?
        .with(system::MoveTankSystem, "move_tank_system", &["input_system"])
        .with(system::FireCannonballSystem, "fire_cannonball_system", &["input_system"])
        .with(system::MoveCannonballSystem, "move_cannonball_system", &[])
        .with(system::MoveHpBarSystem, "move_hp_bar_system", &[])
        .with(system::ReduceHpSystem, "reduce_hp_system", &[])
        .with(system::HpBarSystem, "hp_bar_system", &[])
        .with(system::GameResultSystem, "game_result_system", &[]);

    // define the variable `game` and run the application
    let mut game = Application::new(assets_dir, GamePlayState::default(), game_data)?;
    game.run();

    Ok(())
}
