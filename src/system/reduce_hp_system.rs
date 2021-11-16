// use dependencies
use amethyst::core::Transform;
use amethyst::ecs::{Entities, Join, ReadStorage, System, WriteStorage};

// use entities
use crate::entities::{Cannonball1p, Cannonball2p, Tank1p, Tank2p};

// define struct `ReduceHpSystem`
pub struct ReduceHpSystem;

impl<'s> System<'s> for ReduceHpSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Tank1p>,
        WriteStorage<'s, Tank2p>,
        WriteStorage<'s, Cannonball1p>,
        WriteStorage<'s, Cannonball2p>
    );

    fn run(&mut self, (entities, transforms, mut tanks_1p, mut tanks_2p, cannonballs_1p, cannonballs_2p): Self::SystemData) {
        for (_, transform, entity) in (&cannonballs_1p, &transforms, &entities).join() {
            for (tank_2p, tank_2p_transform) in (&mut tanks_2p, &transforms).join() {
                for tank_1p in (&mut tanks_1p).join() {
                    // reduce enemy tank's hp and delete the cannonball if it hits enemy tank
                    if transform.translation().x + 3.75 >= tank_2p_transform.translation().x - 37.5 &&
                        transform.translation().x - 3.75 <= tank_2p_transform.translation().x + 37.5 &&
                        transform.translation().y - 3.75 <= tank_2p_transform.translation().y + 37.5 &&
                        transform.translation().y + 3.75 >= tank_2p_transform.translation().y - 37.5 {
                        tank_1p.fired = false;
                        tank_2p.hp -= 10;
                        let _ = entities.delete(entity);
                    }
                }
            }
        }
        for (_, transform, entity) in (&cannonballs_2p, &transforms, &entities).join() {
            for (tank_1p, tank_1p_transform) in (&mut tanks_1p, &transforms).join() {
                for tank_2p in (&mut tanks_2p).join() {
                    // reduce enemy tank's hp and delete the cannonball if it hits enemy tank
                    if transform.translation().x - 3.75 <= tank_1p_transform.translation().x + 37.5 &&
                        transform.translation().x + 3.75 >= tank_1p_transform.translation().x - 37.5 &&
                        transform.translation().y - 3.75 <= tank_1p_transform.translation().y + 37.5 &&
                        transform.translation().y + 3.75 >= tank_1p_transform.translation().y - 37.5 {
                        tank_2p.fired = false;
                        tank_1p.hp -= 10;
                        let _ = entities.delete(entity);
                    }
                }

            }
        }
    }
}