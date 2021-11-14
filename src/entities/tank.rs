use amethyst::assets::Handle;
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet};

use crate::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct Tank1p {
    pub shooted: bool,
    pub hp: u8,
    pub transform: Vector3<f32>
}
pub struct Tank2p {
    pub shooted: bool,
    pub hp: u8,
    pub transform: Vector3<f32>
}

impl Component for Tank1p {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Tank2p {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_tanks(world: &mut World, sprite_handle: Handle<SpriteSheet>) {
    let mut tank_1p_transform = Transform::default();
    let mut tank_2p_transform = Transform::default();

    tank_1p_transform.set_translation_xyz(120.0, ARENA_HEIGHT * 0.5, 0.5);
    tank_2p_transform.set_translation_xyz(ARENA_WIDTH - 120.0, ARENA_HEIGHT * 0.5, 0.5);

    let tank_1p = SpriteRender::new(sprite_handle.clone(), 0);
    let tank_2p = SpriteRender::new(sprite_handle.clone(), 1);

    world
        .create_entity()
        .with(Tank1p {
            shooted: false,
            hp: 100,
            transform: {
                let &x = tank_1p_transform.translation();
                x
            }
        })
        .with(tank_1p_transform)
        .with(tank_1p)
        .build();

    world
        .create_entity()
        .with(Tank2p {
            shooted: false,
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