// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
//
/// NOTE - the structure for this code was based in a example in the ggez repo.
/// Specifically, the design of 02_hello_world.rs (see 
/// https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)

extern crate ggez;

use ggez::event;
use ggez::graphics;
use ggez::mint::Point2;
use ggez::{Context, GameResult};

/// Enum representing which game is loaded
enum GameLoaded {
    CONNECT4,
    NONE
}

/*impl GameLoaded {

}*/

struct GameState {
    frames: usize,
    text: graphics::Text,
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        //Usage of Point2 from ggez eample 04_snake.rs, line 354 (https://github.com/ggez/ggez/blob/master/examples/04_snake.rs)
        let dest_point = Point2 { x: 10.0, y: 10.0 };
        graphics::draw(ctx, &self.text, (dest_point,))?;
        graphics::present(ctx)?;

        Ok(())
    }
}

//Implementation based on structure in example from GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        //let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let text = graphics::Text::new("Test");

        let s = GameState { frames: 0, text };
        Ok(s)
    }
}


//Main game loop - tweaked from example in GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("gamescloset", "cs510");
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut GameState::new(ctx)?;
    event::run(ctx, event_loop, state)
}