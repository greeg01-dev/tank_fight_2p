use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

use crate::entities::{Tank1p, Tank2p, HpBarOuter1p, HpBarOuter2p};

pub struct HpBarSystem;

impl<'s> System<'s> for HpBarSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Tank1p>,
        ReadStorage<'s, Tank2p>,
        WriteStorage<'s, HpBarOuter1p>,
        WriteStorage<'s, HpBarOuter2p>,
    );

    fn run(&mut self, (mut transforms, tanks_1p, tanks_2p, mut hp_bars_outer_1p, mut hp_bars_outer_2p): Self::SystemData) {
        for (hp_bar_outer_1p, transform) in (&mut hp_bars_outer_1p, &mut transforms).join() {
            for tank_1p in (&tanks_1p).join() {
                if tank_1p.hp != hp_bar_outer_1p.hp {
                    hp_bar_outer_1p.hp -= 1;
                    transform.set_scale(Vector3::new(hp_bar_outer_1p.hp as f32 / hp_bar_outer_1p.max_hp as f32, 1.0, 1.0));
                    transform.prepend_translation_x(-0.3);
                }
            }
        }
        for (hp_bar_outer_2p, transform) in (&mut hp_bars_outer_2p, &mut transforms).join() {
            for tank_2p in (&tanks_2p).join() {
                if tank_2p.hp != hp_bar_outer_2p.hp {
                    hp_bar_outer_2p.hp -= 1;
                    transform.set_scale(Vector3::new(hp_bar_outer_2p.hp as f32 / hp_bar_outer_2p.max_hp as f32, 1.0, 1.0));
                    transform.prepend_translation_x(-0.3);
                }
            }
        }
    }
}