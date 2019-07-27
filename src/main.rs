// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
//
/// NOTE - the structure for this code was based in a example in the ggez repo.
/// Specifically, the design of 02_hello_world.rs (see 
/// https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)

extern crate ggez;
mod connect4;

use ggez::event;
use ggez::graphics;
use ggez::input::mouse;
use ggez::input::mouse::MouseButton;
use ggez::mint::Point2;
use ggez::{Context, GameResult};

/// Enum representing which game is loaded
enum GameLoaded {
    NONE,
    CONNECT4,
}

/*impl GameLoaded {

}*/

struct Button {
    text: graphics::Text,
    outline: graphics::Rect,
    highlighted: bool,
}

impl Button {
    fn new(text: graphics::Text, dim: graphics::Rect) -> Button {
        Button { text: text, outline: dim, highlighted: false}
    }

    fn is_button_under_mouse(&mut self, ctx: &mut Context) -> bool {
        let mouse_loc = mouse::position(ctx);
        if self.outline.contains(mouse_loc)  {
            self.highlighted = true;
        } else {
            self.highlighted = false;
        }
        self.highlighted
    }

}

struct GameState {
    frames: usize,
    buttons: Vec<Button>,
    gameLoaded: GameLoaded,
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //println!("Update called");
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        //Usage of Point2 from ggez example 04_snake.rs, line 354 (https://github.com/ggez/ggez/blob/master/examples/04_snake.rs)
        let dest_point = Point2 { x: 10.0, y: 10.0 };
        graphics::draw(ctx, &self.buttons[0].text, (dest_point,))?;
        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        //Check whether buttons are highlighted, updated states accordingly
        for i in 0..self.buttons.len() {
            self.buttons[i].is_button_under_mouse(_ctx);
            println!("Button '{}' highlighted: {}", self.buttons[i].text.contents(), self.buttons[i].highlighted);
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        //Check whether buttons are highlighted (set by clicking down). If one is highlighted and mouse still on it, button is "clicked"
        for i in 0..self.buttons.len() {
            if self.buttons[i].highlighted && self.buttons[i].is_button_under_mouse(_ctx) {
                println!("Button '{}' clicked!!!!!!", self.buttons[i].text.contents());
            }
        }
    }

}

//Implementation based on structure in example from GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        //let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let text = graphics::Text::new("Connect4");
        let text_width = text.width(ctx) as f32;
        let text_height = text.height(ctx) as f32;
        let mut btns = Vec::new();
        btns.push(Button::new(text, graphics::Rect::new(10.0, 10.0, text_width, text_height)));

        let s = GameState { frames: 0, buttons: btns, gameLoaded: GameLoaded::NONE };
        Ok(s)
    }

    //Method to print organized list of buttons
    fn print_buttons(&mut self, ctx: &mut Context) -> graphics::Rect {
        let screen = graphics::screen_coordinates(ctx);
        let mut button_height = 0.0;
        let mut button_width = 0.0;
        for i in 0..self.buttons.len() {
            button_height += 1.5*self.buttons[i].outline.h;
            if button_width < self.buttons[i].outline.w {
                button_width = self.buttons[i].outline.w;
            }
        }
        let x_start = (screen.w - button_width) / 2 as f32;
        let y_start = (screen.h - button_height) / 2 as f32;
        graphics::Rect::new(x_start, y_start, button_width, button_height)
    }

}

//Main game loop - tweaked from example in GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("gamescloset", "cs510");
    let (ctx, event_loop) = &mut cb.build()?;

    let state = &mut GameState::new(ctx)?;
    event::run(ctx, event_loop, state)*/
    //connect4::core::main()
}