use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, System, WriteStorage};
use crate::ARENA_WIDTH;

use crate::entities::{Cannonball1p, Cannonball2p, Tank1p, Tank2p};

pub struct MoveCannonballSystem;

impl<'s> System<'s> for MoveCannonballSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Tank1p>,
        WriteStorage<'s, Tank2p>,
        WriteStorage<'s, Cannonball1p>,
        WriteStorage<'s, Cannonball2p>
    );

    fn run(&mut self, (entities, mut transforms, mut tank_1ps, mut tank_2ps, mut cannonballs_1p, mut cannonballs_2p): Self::SystemData) {
        for (_, transform, entity) in (&mut cannonballs_1p, &mut transforms, &entities).join() {
            for tank in (&mut tank_1ps).join() {
                if tank.shooted {
                    transform.prepend_translation_x(15.0);
                }
                if transform.translation().x >= ARENA_WIDTH {
                    let _ = entities.delete(entity);
                    tank.shooted = false;
                }
            }
        }
        for (_, transform, entity) in (&mut cannonballs_2p, &mut transforms, &entities).join() {
            for tank in (&mut tank_2ps).join() {
                if tank.shooted {
                    transform.prepend_translation_x(-15.0);
                }
                if transform.translation().x <= 0.0 {
                    let _ = entities.delete(entity);
                    tank.shooted = false;
                }
            }
        }
    }
}