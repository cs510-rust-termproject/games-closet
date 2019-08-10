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

use std::fmt;
use ggez::event;
use ggez::graphics;
use ggez::input::mouse;
use ggez::input::mouse::MouseButton;
use ggez::mint::Point2;
use ggez::{Context, GameResult};

const BUTTON_PADDING: (f32, f32) =  (10.0, 10.0);
const BUTTON_SPACING: (f32, f32) = (50.0, 50.0);
const BUTTON_FONT_SIZE: f32 = 36f32;
const SCREEN_SIZE: (f32, f32) = (910.0, 500.0); //Note - this is hard coded based on the known title sizes and should be adjusted if titles change

/// Enum representing which game is loaded
enum GameLoaded {
    NONE,
    CONNECT4,
}

//To_string implementation, found from https://doc.rust-lang.org/rust-by-example/conversion/string.html
impl fmt::Display for GameLoaded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            GameLoaded::NONE => "None",
            GameLoaded::CONNECT4 => "Connect 4",
            _ => panic!("Unknown GameLoaded type")
        };
        write!(f, "{}", text)
    }
}

/*impl GameLoaded {

}*/

struct Button {
    text: graphics::Text,
    outline: graphics::Rect,
    pub active: bool,
    pub selected: bool,
    pub highlighted: bool
}

impl Button {
    fn new(text: graphics::Text, dim: graphics::Rect) -> Button {
        Button { text: text, outline: dim, active: true, selected: false, highlighted: false}
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let textbox = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(),             
            self.outline,
            graphics::Color::from_rgba(133,0,0,255),
        )?;
        let TEXT_OFFSET = ((self.outline.w - self.text.width(ctx) as f32)/2.0, (self.outline.h - self.text.height(ctx) as f32)/2.0);
        graphics::draw(ctx, &textbox, (Point2 {x: 0.0, y: 0.0},))?;
        graphics::draw(ctx, &self.text, (Point2 {x: self.outline.x + TEXT_OFFSET.0, y: self.outline.y + TEXT_OFFSET.1},))?;
        //println!("{},{}  {},{}", self.outline.x, self.outline.y, self.outline.x - TEXT_OFFSET.0, self.outline.y - TEXT_OFFSET.1);
        Ok(())
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
    buttons: Vec<Vec<Button>>,
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
        //let dest_point = Point2 { x: 10.0, y: 10.0 };
        self.draw_buttons(ctx);
        graphics::present(ctx)?;

        Ok(())
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        //Check whether buttons are highlighted, updated states accordingly
        for i in 0..self.buttons.len() {
            for j in 0..self.buttons[i].len() {
                self.buttons[i][j].is_button_under_mouse(_ctx);
                //println!("Button '{}' highlighted: {}", self.buttons[i][j].text.contents(), self.buttons[i][j].highlighted);
            }
        }
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        //Check whether buttons are highlighted (set by clicking down). If one is highlighted and mouse still on it, button is "clicked"
        for i in 0..self.buttons.len() {
            for j in 0..self.buttons[i].len() {
                if self.buttons[i][j].highlighted && self.buttons[i][j].is_button_under_mouse(_ctx) {
                    self.buttons[i][j].selected = !self.buttons[i][j].selected;
                    println!("Button '{}' clicked!", self.buttons[i][j].text.contents());
                }
            }
        }
    }

}

//Implementation based on structure in example from GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        //Font should be set to a param
        let mut s = GameState { frames: 0, buttons: Vec::<Vec::<Button>>::new(), gameLoaded: GameLoaded::NONE };
        s.create_buttons(ctx);
        Ok(s)
    }

    //Method to print organized list of buttons
    fn draw_buttons(&mut self, ctx: &mut Context) {
        for i in 0..self.buttons.len() {
            for j in 0..self.buttons[i].len() {
                self.buttons[i][j].draw(ctx);
            }
        }
    }

    fn create_buttons(&mut self, ctx: &mut Context) {
        //Apparently can't loop through enums, so have to manually add each game
        let games = vec![GameLoaded::CONNECT4];
        //Init button vec for titles, games and num players
        while self.buttons.len() < 3 {
            self.buttons.push(Vec::<Button>::new());
        }
        //TITLES
        let titles = vec![graphics::Text::new(("Select Game", graphics::Font::default(), 48f32)),
                           graphics::Text::new(("Players", graphics::Font::default(), 48f32)),
                           graphics::Text::new(("Start Game", graphics::Font::default(), 48f32))];
        let mut loc = BUTTON_SPACING.0;
        for title in &titles {
            let buttonText =  graphics::Text::new((title.contents(), graphics::Font::default(), 48f32));
            let buttonOutline = graphics::Rect::new(loc, BUTTON_SPACING.1, 2.0*BUTTON_PADDING.0 + buttonText.width(ctx) as f32, 2.0*BUTTON_PADDING.1 + buttonText.height(ctx) as f32);
            println!("{}: {:?}", buttonText.contents(), buttonOutline);;
            self.buttons[0].push(Button::new(buttonText, buttonOutline));
            loc = loc + buttonOutline.w + BUTTON_SPACING.0;
        }
        //GAME SELECTION BUTTONS
        let mut maxDim = (0, 0);
        //Identify max length for text for all games
        for game in &games {
            let buttonText = graphics::Text::new((game.to_string(), graphics::Font::default(), 48f32));
            maxDim.0 = maxDim.0.max(buttonText.width(ctx));
            maxDim.1 = maxDim.1.max(buttonText.height(ctx));
        }
        //Create buttons for games based on max dimensions so they are equal size
        for i in 0..games.len() {
            let titleOutline = self.buttons[0][i].outline;
            self.buttons[1].push(Button::new(graphics::Text::new((games[i].to_string(), graphics::Font::default(), 48f32)),
                                             graphics::Rect::new(titleOutline.x, 
                                                                 titleOutline.y + titleOutline.h + BUTTON_SPACING.1,
                                                                 2.0*BUTTON_PADDING.0 + maxDim.0 as f32, 
                                                                 2.0*BUTTON_PADDING.1 +maxDim.1 as f32)
                                            ));
            println!("{:?}", self.buttons[1][i].outline);
        }
    }

}

//Main game loop - tweaked from example in GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
pub fn main() -> GameResult {
    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("Games Closet", "Lane Barton & Andre Mukhsia")
        .window_setup(ggez::conf::WindowSetup::default().title("Game Closet - Main Menu"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut GameState::new(ctx)?;
    event::run(ctx, event_loop, state)
    //connect4::core::main()
}