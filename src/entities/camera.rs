use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::Camera;
use crate::{ARENA_HEIGHT, ARENA_WIDTH};

pub fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}