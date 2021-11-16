// use dependencies
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, System, WriteStorage};
use amethyst::input::{Button, InputHandler, StringBindings, VirtualKeyCode};
use amethyst::winit::MouseButton;

// use entities and function
use crate::entities::{CannonballResource1p, CannonballResource2p, fire_cannonball, Tank1p, Tank2p};

// define struct `FireCannonballSystem`
pub struct FireCannonballSystem;

impl<'s> System<'s> for FireCannonballSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tank1p>,
        WriteStorage<'s, Tank2p>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, CannonballResource1p>,
        ReadExpect<'s, CannonballResource2p>,
        ReadExpect<'s, LazyUpdate>
    );

    fn run(&mut self, (entities, transforms, mut tanks_1p, mut tanks_2p, input, cannonball_resources_1p, cannonball_resources_2p,lazy_update): Self::SystemData) {
        // check if Space key is down and fire a cannonball
        if input.button_is_down(Button::Key(VirtualKeyCode::Space)) {
            for (tank_1p, transform) in (&mut tanks_1p, &transforms).join() {
                if !tank_1p.fired {
                    tank_1p.fired = true;
                    fire_cannonball(&entities, &cannonball_resources_1p, &cannonball_resources_2p, &lazy_update, 1, transform);
                }
            }
        }

        // check if mouse button clicked and fire a cannonball
        if input.mouse_button_is_down(MouseButton::Left) {
            for (tank_2p, transform) in (&mut tanks_2p, &transforms).join() {
                if !tank_2p.fired {
                    tank_2p.fired = true;
                    fire_cannonball(&entities, &cannonball_resources_1p, &cannonball_resources_2p, &lazy_update, 2, transform);
                }
            }
        }
    }
}