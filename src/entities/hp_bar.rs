// use dependencies
use amethyst::assets::Handle;
use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet};

// use variables `ARENA_HEIGHT` and `ARENA_WIDTH`
use crate::{ARENA_HEIGHT, ARENA_WIDTH};

// define structs `HpBar1p`, `HpBar2p`, `HpBarOuter1p`, and `HpBarOuter2p`
pub struct HpBar1p;
pub struct HpBar2p;
pub struct HpBarOuter1p {
    pub hp: u8, // to check if this component is same with tank's hp
    pub max_hp: u8 // to define tank's max hp
}
pub struct HpBarOuter2p {
    pub hp: u8, // to check if this component is same with tank's hp
    pub max_hp: u8 // to define tank's max hp
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

// define function `initialise_hp_bar` to make hp bar entities
pub fn initialise_hp_bar(world: &mut World, sprite_handle: Handle<SpriteSheet>) {
    // makes variables for hp bars' transform
    let mut hp_bar_1p_transform = Transform::default();
    let mut hp_bar_2p_transform = Transform::default();
    let mut hp_bar_outer_1p_transform = Transform::default();
    let mut hp_bar_outer_2p_transform = Transform::default();

    // set hp bars' transform
    hp_bar_1p_transform.set_translation_xyz(120.0, ARENA_HEIGHT * 0.5 + 57.5, 0.4);
    hp_bar_2p_transform.set_translation_xyz(ARENA_WIDTH - 120.0, ARENA_HEIGHT * 0.5 + 57.5, 0.4);
    hp_bar_outer_1p_transform.set_translation_xyz(120.0, ARENA_HEIGHT * 0.5 + 57.5, 0.5);
    hp_bar_outer_2p_transform.set_translation_xyz(ARENA_WIDTH - 120.0, ARENA_HEIGHT * 0.5 + 57.5, 0.5);

    // set `SpriteRender` for hp bars
    let hp_bar_outer = SpriteRender::new(sprite_handle.clone(), 0);
    let hp_bar_inner = SpriteRender::new(sprite_handle.clone(), 1);

    // create `HpBarOuter1p` entity
    world
        .create_entity()
        .with(HpBarOuter1p { hp: 100, max_hp: 100 })
        .with(hp_bar_outer_1p_transform)
        .with(hp_bar_outer.clone())
        .build();

    // create `HpBarOuter2p` entity
    world
        .create_entity()
        .with(HpBarOuter2p { hp: 100, max_hp: 100 })
        .with(hp_bar_outer_2p_transform)
        .with(hp_bar_outer.clone())
        .build();

    // create `HpBar1p` entity
    world
        .create_entity()
        .with(HpBar1p)
        .with(hp_bar_1p_transform)
        .with(hp_bar_inner.clone())
        .build();

    // create `HpBar2p` entity
    world
        .create_entity()
        .with(HpBar2p)
        .with(hp_bar_2p_transform)
        .with(hp_bar_inner.clone())
        .build();
}