// use dependencies
use amethyst::assets::{Handle, Loader};
use amethyst::ecs::{Entities, LazyUpdate, ReadExpect};
use amethyst::prelude::*;
use amethyst::ui::{Anchor, FontAsset, LineMode, TtfFormat, UiText, UiTransform};

// define struct `WinnerText`
pub struct WinnerText {
    pub font: Handle<FontAsset> // to use font in function `quit_game`
}

// define function `initialise_text` to insert `WinnerText` to world
pub fn initialise_text(world: &mut World) {
    world.insert(WinnerText {
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

// define function `quit_game` to quit game with showing the winner
pub fn quit_game(entities: &Entities, winner: u8, font: Handle<FontAsset>, lazy_update: &ReadExpect<LazyUpdate>) {
    // define variable `ui_text which` have configuration about UiText
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
    // define variable `ui_transform` which have transform of text
    let ui_transform = UiTransform::new(
        String::from("winner"),
        Anchor::Middle,
        Anchor::Middle,
        0.0, 0.0, 0.5,
        700.0, 100.0
    );

    // create text_entity
    let text_entity = entities.create();

    // add text_entity to the world
    lazy_update.insert(text_entity, ui_text);
    lazy_update.insert(text_entity, ui_transform);
}