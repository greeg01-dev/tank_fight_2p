use amethyst::{GameData, SimpleState, StateData};

use crate::entities::initialise_entities;

#[derive(Default)]
pub struct GamePlayState;

impl SimpleState for GamePlayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        initialise_entities(data.world)
    }
}