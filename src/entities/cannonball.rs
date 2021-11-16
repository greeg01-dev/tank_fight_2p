// use dependencies
use amethyst::assets::Handle;
use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage, Entities, LazyUpdate, ReadExpect};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet};

// define struct `Cannonball1p` with derivable traits `Copy` and `Clone`
#[derive(Copy, Clone)]
pub struct Cannonball1p;
// define struct `Cannonball2p` with derivable traits `Copy` and `Clone`
#[derive(Copy, Clone)]
pub struct Cannonball2p;

// define struct `CannonballResource1p`
pub struct CannonballResource1p {
    pub component: Cannonball1p, // to use `Cannonball1p` struct with this struct
    pub sprite_render: SpriteRender // to use rendered tank sprite
}
// define struct `CannonballResource1p`
pub struct CannonballResource2p {
    pub component: Cannonball2p, // to use `Cannonball2p` struct with this struct
    pub sprite_render: SpriteRender // to use rendered tank sprite
}

impl Component for Cannonball1p {
    type Storage = DenseVecStorage<Self>;
}
impl Component for Cannonball2p {
    type Storage = DenseVecStorage<Self>;
}

// define function `initialise_cannonball` to insert `CannonballResource1p` and `CannonballResource2p` to world
pub fn initialise_cannonball(world: &mut World, sprite_handle: Handle<SpriteSheet>) {
    let cannonball_resource_1p = CannonballResource1p {
        component: Cannonball1p,
        sprite_render: SpriteRender::new(sprite_handle.clone(), 2)
    };
    let cannonball_resource_2p = CannonballResource2p {
        component: Cannonball2p,
        sprite_render: SpriteRender::new(sprite_handle.clone(), 2)
    };
    world.insert(cannonball_resource_1p);
    world.insert(cannonball_resource_2p);
}

// define function `fire_cannonball` to fire the cannonball
pub fn fire_cannonball(
    entities: &Entities,
    cannonball_resource_1p: &ReadExpect<CannonballResource1p>,
    cannonball_resource_2p: &ReadExpect<CannonballResource2p>,
    lazy_update: &ReadExpect<LazyUpdate>,
    player: u8, player_transform: &Transform
) {
    // create entity which name is `cannonball_entity`
    let cannonball_entity = entities.create();

    match player {
        1 => {
            // set cannonball's transform
            let mut ball_trans = Transform::default();
            ball_trans.set_translation_xyz(player_transform.translation().x + 37.5, player_transform.translation().y, 0.0);

            // add cannonball entity to the world
            lazy_update.insert(cannonball_entity, cannonball_resource_1p.component.clone());
            lazy_update.insert(cannonball_entity, cannonball_resource_1p.sprite_render.clone());
            lazy_update.insert(cannonball_entity, ball_trans);
        },
        2 => {
            // set cannonball's transform
            let mut ball_trans = Transform::default();
            ball_trans.set_translation_xyz(player_transform.translation().x - 37.5, player_transform.translation().y, 0.0);

            // add cannonball entity to the world
            lazy_update.insert(cannonball_entity, cannonball_resource_2p.component.clone());
            lazy_update.insert(cannonball_entity, cannonball_resource_1p.sprite_render.clone());
            lazy_update.insert(cannonball_entity, ball_trans);
        },
        _ => panic!("Variable `player` must be 1 or 2")
    };
}