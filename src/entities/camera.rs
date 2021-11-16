// use dependencies
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::Camera;

// use variables `ARENA_HEIGHT` and `ARENA_WIDTH`
use crate::{ARENA_HEIGHT, ARENA_WIDTH};

// define function `initialise_camera` to make camera entity
pub fn initialise_camera(world: &mut World) {
    // set camera's transform
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    // create camera entity
    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}