// use dependencies
use amethyst::assets::Handle;
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet};

// use variables `ARENA_HEIGHT` and `ARENA_WIDTH`
use crate::{ARENA_HEIGHT, ARENA_WIDTH};

// define struct `Tank1p`
pub struct Tank1p {
    pub fired: bool, // to check if cannonball is fired
    pub hp: u8, // tank's hp
    pub transform: Vector3<f32> // to check tank's transform
}
//define struct `Tank2p`
pub struct Tank2p {
    pub fired: bool, // to check if cannonball is fired
    pub hp: u8, // tank's hp
    pub transform: Vector3<f32> // to check tank's transform
}

impl Component for Tank1p {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Tank2p {
    type Storage = DenseVecStorage<Self>;
}

// define function `initialise_tanks` to make tank entities
pub fn initialise_tanks(world: &mut World, sprite_handle: Handle<SpriteSheet>) {
    // make variables for tanks' transform
    let mut tank_1p_transform = Transform::default();
    let mut tank_2p_transform = Transform::default();

    // set tanks' transform
    tank_1p_transform.set_translation_xyz(120.0, ARENA_HEIGHT * 0.5, 0.5);
    tank_2p_transform.set_translation_xyz(ARENA_WIDTH - 120.0, ARENA_HEIGHT * 0.5, 0.5);

    // set `SpriteRender` for tanks
    let tank_1p = SpriteRender::new(sprite_handle.clone(), 0);
    let tank_2p = SpriteRender::new(sprite_handle.clone(), 1);

    // create `Tank1p` entity
    world
        .create_entity()
        .with(Tank1p {
            fired: false,
            hp: 100,
            transform: {
                let &x = tank_1p_transform.translation();
                x
            }
        })
        .with(tank_1p_transform)
        .with(tank_1p)
        .build();

    // create `Tank2p` entity
    world
        .create_entity()
        .with(Tank2p {
            fired: false,
            hp: 100,
            transform: {
                let &x = tank_2p_transform.translation();
                x
            }
        })
        .with(tank_2p_transform)
        .with(tank_2p)
        .build();
}