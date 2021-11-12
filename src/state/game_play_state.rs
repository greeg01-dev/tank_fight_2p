use amethyst::{GameData, SimpleState, SimpleTrans, StateData, StateEvent};
use amethyst::core::Transform;
use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::prelude::*;
use amethyst::renderer::Camera;
use crate::{ARENA_HEIGHT, ARENA_WIDTH};

pub struct GamePlayState;
pub struct Tank;

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<Tank>();
    }
}

impl GamePlayState {
    pub fn new() -> GamePlayState {
        GamePlayState
    }
}

impl Component for Tank {
    type Storage = DenseVecStorage<Self>;
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
        .with(transform)
        .build();
}