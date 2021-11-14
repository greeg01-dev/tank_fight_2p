use amethyst::assets::{Handle, Loader};
use amethyst::ecs::{Entities, LazyUpdate, ReadExpect};
use amethyst::prelude::*;
use amethyst::ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform};

pub struct Winner {
    pub font: Handle<FontAsset>
}

pub fn initialise_text(world: &mut World) {
    world.insert(Winner {
        font: {
            let loader = world.read_resource::<Loader>();
            let font: Handle<FontAsset> = loader.load(
                "font/square.ttf",
                TtfFormat,
                (),
                &world.read_resource()
            );
            font
        }
    });
}

pub fn quit_game(entities: &Entities, winner: u8, font: Handle<FontAsset>, lazy_update: &ReadExpect<LazyUpdate>) {
    let ui_text = UiText::new(
        font,
        {
            if winner == 0 {
                String::from("The game is ended in a draw")
            }
            else {
                format!("{}p Wins", winner)
            }
        },
        [0.0, 0.0, 0.0, 1.0],
        100.0,
        LineMode::Single,
        Anchor::Middle
    );
    let ui_transform = UiTransform::new(
        String::from("winner"),
        Anchor::Middle,
        Anchor::Middle,
        0.0, 0.0, 0.5,
        700.0, 100.0
    );

    let text_entity = entities.create();

    lazy_update.insert(text_entity, ui_text);
    lazy_update.insert(text_entity, ui_transform);
}