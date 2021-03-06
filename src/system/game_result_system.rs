// use dependencies
use amethyst::ecs::{Entities, Join, LazyUpdate, ReadExpect, ReadStorage, System};

// use entities and function
use crate::entities::{Tank1p, Tank2p, HpBarOuter1p, HpBarOuter2p, HpBar1p, HpBar2p, Cannonball1p, Cannonball2p, WinnerText, quit_game};

// define struct `GameResultSystem`
pub struct GameResultSystem;

impl<'s> System<'s> for GameResultSystem {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Tank1p>,
        ReadStorage<'s, Tank2p>,
        ReadStorage<'s, HpBar1p>,
        ReadStorage<'s, HpBar2p>,
        ReadStorage<'s, HpBarOuter1p>,
        ReadStorage<'s, HpBarOuter2p>,
        ReadStorage<'s, Cannonball1p>,
        ReadStorage<'s, Cannonball2p>,
        ReadExpect<'s, WinnerText>,
        ReadExpect<'s, LazyUpdate>
    );

    fn run(&mut self, (entities, tanks_1p, tanks_2p, hp_bars_1p, hp_bars_2p, hp_bars_outer_1p, hp_bars_outer_2p, cannonballs_1p, cannonballs_2p, winner_texts, lazy_update): Self::SystemData) {
        // check players' hp and remove all entities in the game and quit game
        for hp_bar_outer_1p in (&hp_bars_outer_1p).join() {
            for hp_bar_outer_2p in (&hp_bars_outer_2p).join() {
                if hp_bar_outer_1p.hp == 0 && hp_bar_outer_2p.hp == 0 {
                    remove_all_entities(&entities, &tanks_1p, &tanks_2p, &hp_bars_1p, &hp_bars_2p, &hp_bars_outer_1p, &hp_bars_outer_2p, &cannonballs_1p, &cannonballs_2p);
                    quit_game(&entities, 0, winner_texts.font.clone(), &lazy_update)
                }
                else if hp_bar_outer_1p.hp == 0 {
                    remove_all_entities(&entities, &tanks_1p, &tanks_2p, &hp_bars_1p, &hp_bars_2p, &hp_bars_outer_1p, &hp_bars_outer_2p, &cannonballs_1p, &cannonballs_2p);
                    quit_game(&entities, 2, winner_texts.font.clone(), &lazy_update);
                }
                else if hp_bar_outer_2p.hp == 0 {
                    remove_all_entities(&entities, &tanks_1p, &tanks_2p, &hp_bars_1p, &hp_bars_2p, &hp_bars_outer_1p, &hp_bars_outer_2p, &cannonballs_1p, &cannonballs_2p);
                    quit_game(&entities, 1, winner_texts.font.clone(), &lazy_update);
                }
            }
        }
    }
}

// define function `remove_all_entities` to remove all entities in the game
fn remove_all_entities(
    entities: &Entities<>,
    tanks_1p: &ReadStorage<Tank1p>,
    tanks_2p: &ReadStorage<Tank2p>,
    hp_bars_1p: &ReadStorage<HpBar1p>,
    hp_bars_2p: &ReadStorage<HpBar2p>,
    hp_bars_outer_1p: &ReadStorage<HpBarOuter1p>,
    hp_bars_outer_2p: &ReadStorage<HpBarOuter2p>,
    cannonballs_1p: &ReadStorage<Cannonball1p>,
    cannonballs_2p: &ReadStorage<Cannonball2p>
) {
    for (_, entity) in (tanks_1p, entities).join() {
        let _ = entities.delete(entity);
    }
    for (_, entity) in (tanks_2p, entities).join() {
        let _ = entities.delete(entity);
    }
    for (_, entity) in (hp_bars_1p, entities).join() {
        let _ = entities.delete(entity);
    }
    for (_, entity) in (hp_bars_2p, entities).join() {
        let _ = entities.delete(entity);
    }
    for (_, entity) in (hp_bars_outer_1p, entities).join() {
        let _ = entities.delete(entity);
    }
    for (_, entity) in (hp_bars_outer_2p, entities).join() {
        let _ = entities.delete(entity);
    }
    for (_, entity) in (cannonballs_1p, entities).join() {
        let _ = entities.delete(entity);
    }
    for (_, entity) in (cannonballs_2p, entities).join() {
        let _ = entities.delete(entity);
    }
}