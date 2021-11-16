// use dependencies
use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, System, WriteStorage};
use amethyst::input::{Button, InputHandler, StringBindings, VirtualKeyCode};

// use variable `ARENA_HEIGHT` and entities
use crate::ARENA_HEIGHT;
use crate::entities::{Tank1p, Tank2p};

// define struct `MoveTankSystem`
pub struct MoveTankSystem;

impl<'s> System<'s> for MoveTankSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tank1p>,
        WriteStorage<'s, Tank2p>,
        Read<'s, InputHandler<StringBindings>>
    );

    fn run(&mut self, (mut transforms, mut tank_1ps, mut tank_2ps, input): Self::SystemData) {
        for (tank_1p, transform) in (&mut tank_1ps, &mut transforms).join() {
            let mut scaled_amount: f32 = 0.0;
            // add tank's y coordinate if W button is down
            if input.button_is_down(Button::Key(VirtualKeyCode::W)) {
                scaled_amount = 2.0;
            }
            // subtract tank's y coordinate if S button is down
            if input.button_is_down(Button::Key(VirtualKeyCode::S)) {
                scaled_amount = -2.0;
            }

            // make tank doesn't go out of window
            transform.set_translation_y(
                (transform.translation().y + scaled_amount)
                    .min(ARENA_HEIGHT - 37.5)
                    .max(37.5)
            );
            // change variable `tank_1p.transform` to tank's transform
            {
                let &x = transform.translation();
                tank_1p.transform = x;
            }
        }
        for (tank_2p, transform) in (&mut tank_2ps, &mut transforms).join() {
            let mut scaled_amount: f32 = 0.0;
            // add tank's y coordinate if Up Arrow button is down
            if input.button_is_down(Button::Key(VirtualKeyCode::Up)) {
                scaled_amount = 2.0;
            }
            // subtract tank's y coordinate if Down Arrow button is down
            if input.button_is_down(Button::Key(VirtualKeyCode::Down)) {
                scaled_amount = -2.0;
            }

            // make tank doesn't go out of window
            transform.set_translation_y(
                (transform.translation().y + scaled_amount)
                    .min(ARENA_HEIGHT - 37.5)
                    .max(37.5)
            );
            // change variable `tank_2p.transform` to tank's transform
            {
                let &x = transform.translation();
                tank_2p.transform = x;
            }
        }
    }
}