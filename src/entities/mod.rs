use amethyst::assets::{AssetStorage, Handle, Loader};
use amethyst::prelude::*;
use amethyst::renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture};

mod camera;
mod cannonball;
mod tank;
mod hp_bar;
mod winner;

pub use cannonball::{Cannonball1p, Cannonball2p, CannonballResource1p, CannonballResource2p, fire_cannonball};
pub use tank::{Tank1p, Tank2p};
pub use hp_bar::{HpBar1p, HpBar2p, HpBarOuter1p, HpBarOuter2p};
pub use winner::{Winner, quit_game};

pub fn initialise_entities(world: &mut World) {
    let tank_sprite_handle = load_sprite_sheet(world, "texture/tank_sprite.png", "texture/tank_sprite.ron");
    let hp_bar_sprite_handle = load_sprite_sheet(world, "texture/hp_bar.png", "texture/hp_bar.ron");

    camera::initialise_camera(world);
    tank::initialise_tanks(world, tank_sprite_handle.clone());
    cannonball::initialise_cannonball(world, tank_sprite_handle.clone());
    hp_bar::initialise_hp_bar(world, hp_bar_sprite_handle.clone());
    winner::initialise_text(world);
}

fn load_sprite_sheet(world: &mut World, png_file_path: &str, ron_file_path: &str) -> Handle<SpriteSheet>{
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
    loader.load(
        ron_file_path,
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store
    )
}