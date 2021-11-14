use amethyst::assets::Handle;
use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet};

use crate::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct HpBar1p;
pub struct HpBar2p;
pub struct HpBarOuter1p {
    pub hp: u8,
    pub max_hp: u8
}
pub struct HpBarOuter2p {
    pub hp: u8,
    pub max_hp: u8
}

impl Component for HpBar1p {
    type Storage = DenseVecStorage<Self>;
}

impl Component for HpBar2p {
    type Storage = DenseVecStorage<Self>;
}
impl Component for HpBarOuter1p {
    type Storage = DenseVecStorage<Self>;
}

impl Component for HpBarOuter2p {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialise_hp_bar(world: &mut World, sprite_handle: Handle<SpriteSheet>) {
    let mut hp_bar_1p_transform = Transform::default();
    let mut hp_bar_2p_transform = Transform::default();
    let mut hp_bar_outer_1p_transform = Transform::default();
    let mut hp_bar_outer_2p_transform = Transform::default();

    hp_bar_1p_transform.set_translation_xyz(120.0, ARENA_HEIGHT * 0.5 + 57.5, 0.4);
    hp_bar_2p_transform.set_translation_xyz(ARENA_WIDTH - 120.0, ARENA_HEIGHT * 0.5 + 57.5, 0.4);
    hp_bar_outer_1p_transform.set_translation_xyz(120.0, ARENA_HEIGHT * 0.5 + 57.5, 0.5);
    hp_bar_outer_2p_transform.set_translation_xyz(ARENA_WIDTH - 120.0, ARENA_HEIGHT * 0.5 + 57.5, 0.5);

    let hp_bar_outer = SpriteRender::new(sprite_handle.clone(), 0);
    let hp_bar_inner = SpriteRender::new(sprite_handle.clone(), 1);

    world
        .create_entity()
        .with(HpBarOuter1p { hp: 100, max_hp: 100 })
        .with(hp_bar_outer_1p_transform)
        .with(hp_bar_outer.clone())
        .build();

    world
        .create_entity()
        .with(HpBarOuter2p { hp: 100, max_hp: 100 })
        .with(hp_bar_outer_2p_transform)
        .with(hp_bar_outer.clone())
        .build();
    world
        .create_entity()
        .with(HpBar1p)
        .with(hp_bar_1p_transform)
        .with(hp_bar_inner.clone())
        .build();

    world
        .create_entity()
        .with(HpBar2p)
        .with(hp_bar_2p_transform)
        .with(hp_bar_inner.clone())
        .build();
}