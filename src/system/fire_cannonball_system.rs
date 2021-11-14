use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, LazyUpdate, Read, ReadExpect, System, WriteStorage};
use amethyst::input::{Button, InputHandler, StringBindings, VirtualKeyCode};
use amethyst::winit::MouseButton;

use crate::entities::{CannonballResource1p, CannonballResource2p, fire_cannonball, Tank1p, Tank2p};

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
        if input.button_is_down(Button::Key(VirtualKeyCode::Space)) {
            for (tank_1p, transform) in (&mut tanks_1p, &transforms).join() {
                if !tank_1p.shooted {
                    tank_1p.shooted = true;
                    fire_cannonball(&entities, &cannonball_resources_1p, &cannonball_resources_2p, &lazy_update, 1, transform);
                }
            }
        }

        if input.mouse_button_is_down(MouseButton::Left) {
            for (tank_2p, transform) in (&mut tanks_2p, &transforms).join() {
                if !tank_2p.shooted {
                    tank_2p.shooted = true;
                    fire_cannonball(&entities, &cannonball_resources_1p, &cannonball_resources_2p, &lazy_update, 2, transform);
                }
            }
        }
    }
}