// use dependencies
use amethyst::core::Transform;
use amethyst::ecs::{Join, ReadStorage, System, WriteStorage};

// use entities
use crate::entities::{Tank1p, Tank2p, HpBar1p, HpBar2p, HpBarOuter1p, HpBarOuter2p};

// define struct `MoveHpBarSystem`
pub struct MoveHpBarSystem;

impl<'s> System<'s> for MoveHpBarSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Tank1p>,
        ReadStorage<'s, Tank2p>,
        ReadStorage<'s, HpBar1p>,
        ReadStorage<'s, HpBar2p>,
        ReadStorage<'s, HpBarOuter1p>,
        ReadStorage<'s, HpBarOuter2p>
    );

    fn run(&mut self, (mut transforms, tank_1ps, tank_2ps, hp_bars_1p, hp_bars_2p, hp_bars_outer_1p, hp_bars_outer_2p): Self::SystemData) {
        for tank_1p in (&tank_1ps).join() {
            // move hp bar above the tank
            for (_, transform) in (&hp_bars_1p, &mut transforms).join() {
                transform.set_translation_y(tank_1p.transform.y + 57.5);
            }
            for (_, transform) in (&hp_bars_outer_1p, &mut transforms).join() {
                transform.set_translation_y(tank_1p.transform.y + 57.5);
            }
        }
        for tank_2p in (&tank_2ps).join() {
            // move hp bar above the tank
            for (_, transform) in (&hp_bars_2p, &mut transforms).join() {
                transform.set_translation_y(tank_2p.transform.y + 57.5);
            }
            for (_, transform) in (&hp_bars_outer_2p, &mut transforms).join() {
                transform.set_translation_y(tank_2p.transform.y + 57.5);
            }
        }
    }
}