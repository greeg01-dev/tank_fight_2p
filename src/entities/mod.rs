// use dependencies
use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};

// mod entity files
mod camera;
mod cannonball;
mod tank;
mod hp_bar;
mod winner_text;

// use entities and functions
pub use cannonball::{Cannonball1p, Cannonball2p, CannonballResource1p, CannonballResource2p, fire_cannonball};
pub use tank::{Tank1p, Tank2p};
pub use hp_bar::{HpBar1p, HpBar2p, HpBarOuter1p, HpBarOuter2p};
pub use winner_text::{WinnerText, quit_game};

// define function `initialise_entities` to run functions in other files in `/entities`
pub fn initialise_entities(world: &mut World) {
    // define variables which have sprite sheets' info
    let tank_sprite_handle = load_sprite_sheet(world, "texture/tank_sprite.png", "texture/tank_sprite.ron");
    let hp_bar_sprite_handle = load_sprite_sheet(world, "texture/hp_bar.png", "texture/hp_bar.ron");

    camera::initialise_camera(world);
    tank::initialise_tanks(world, tank_sprite_handle.clone());
    cannonball::initialise_cannonball(world, tank_sprite_handle.clone());
    hp_bar::initialise_hp_bar(world, hp_bar_sprite_handle.clone());
    winner_text::initialise_text(world);
}

// define function `load_sprite_sheet` to load sprite sheet
fn load_sprite_sheet(world: &mut World, png_file_path: &str, ron_file_path: &str) -> Handle<SpriteSheet>{
    // load texture
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            png_file_path,
            ImageFormat(Default::default()),
            (),
            &texture_storage
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();

    // return sprite sheet
    loader.load(
        ron_file_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}