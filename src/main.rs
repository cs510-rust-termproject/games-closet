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
use connect4::core::MyColor;

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
    pub text: graphics::Text,
    outline: graphics::Rect,
    background_color: MyColor,
    active: bool,
    pub selected: bool,
    pub highlighted: bool,
    highlighted_color: MyColor
}

impl Button {
    fn new(text: graphics::Text, dim: graphics::Rect) -> Button {
        Button { text: text, 
                 outline: dim, 
                 background_color: MyColor::Red,
                 active: true, 
                 selected: false, 
                 highlighted: false,
                 highlighted_color: MyColor::Green
                }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        if self.active {
            let mut draw_color = self.background_color.get_draw_color();
            if(self.selected || self.highlighted) {
                draw_color = self.highlighted_color.get_draw_color();
            }
            let textbox = graphics::Mesh::new_rectangle(
                ctx, 
                graphics::DrawMode::fill(),             
                self.outline,
                draw_color,
            )?;
            let TEXT_OFFSET = ((self.outline.w - self.text.width(ctx) as f32)/2.0, (self.outline.h - self.text.height(ctx) as f32)/2.0);
            graphics::draw(ctx, &textbox, (Point2 {x: 0.0, y: 0.0},))?;
            graphics::draw(ctx, &self.text, (Point2 {x: self.outline.x + TEXT_OFFSET.0, y: self.outline.y + TEXT_OFFSET.1},))?;
            //println!("{},{}  {},{}", self.outline.x, self.outline.y, self.outline.x - TEXT_OFFSET.0, self.outline.y - TEXT_OFFSET.1);
        }
        Ok(())
    }

    fn set_active(&mut self, a: bool) {
        self.active = a;
    }

    fn set_colors(&mut self, bg_color: MyColor, hl_color: MyColor) {
        self.background_color = bg_color;
        self.highlighted_color = hl_color;
    }

    fn is_button_under_mouse(&mut self, ctx: &mut Context) -> bool {
        let mouse_loc = mouse::position(ctx);
        if self.active && self.outline.contains(mouse_loc)  {
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
    buttons_available: usize,
    gameLoaded: GameLoaded,
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        //Only allow buttons to be active if previous options selected
        for i in 0..self.buttons.len() {
            for j in 0..self.buttons[i].len() {
                //println!("{}: ({},{}) {}", self.buttons[i][j].text.contents(), i, j, i <= self.buttons_available);
                self.buttons[i][j].active = i <= self.buttons_available;
                self.buttons[i][j].selected = (i <= self.buttons_available) && self.buttons[i][j].selected;
            }
        }
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

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        for i in 0..self.buttons.len() {
            for j in 0..self.buttons[i].len() {
                self.buttons[i][j].is_button_under_mouse(_ctx);
            }
        }
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
        for i in 1..self.buttons.len() {
            for j in 0..self.buttons[i].len() {
                if self.buttons[i][j].highlighted && self.buttons[i][j].is_button_under_mouse(_ctx) {
                    let highlighted = self.is_button_in_column_selected(i);
                    if highlighted < 0 {
                        self.buttons[i][j].selected = true;
                        self.buttons_available = i+1;
                    } else if highlighted != j as i32 {
                        self.buttons[i][j].selected = true;
                        self.buttons[i][highlighted as usize].selected = false;
                        self.buttons_available = i+1;
                    } else {
                        self.buttons[i][j].selected = false;
                        self.buttons_available = i;
                    }
                    println!("Button '{}' clicked!", self.buttons[i][j].text.contents());
                    return;
                }
            }
        }
    }

}

//Implementation based on structure in example from GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        //Font should be set to a param
        let mut s = GameState { frames: 0, buttons: Vec::<Vec::<Button>>::new(), buttons_available:1, gameLoaded: GameLoaded::NONE };
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

    //Method to determine if a button in a menu column is selected. Returns index of a highlighted button or -1 if none is highlighted
    fn is_button_in_column_selected(&self, col: usize) -> i32 {
        if col < 0 || col > self.buttons.len() {
            println!("Error: Cannot check button column {}", col);
        } else {
            for j in 0..self.buttons[col].len() {
                if self.buttons[col][j].selected {
                    return j as i32;
                }
            }
        }
        -1
    }

    //Function to initialize possible buttons
    fn create_buttons(&mut self, ctx: &mut Context) {
        //Apparently can't loop through enums, so have to manually add each game
        let games = vec![GameLoaded::CONNECT4];
        //Init button vec for titles, games and num players
        while self.buttons.len() < 4 {
            self.buttons.push(Vec::<Button>::new());
        }
        //TITLES AND START GAME BUTTON (buttons[0] and buttons[3])
        let titles = vec![graphics::Text::new(("Select Game", graphics::Font::default(), 48f32)),
                           graphics::Text::new(("Players", graphics::Font::default(), 48f32)),
                           graphics::Text::new(("Start Game", graphics::Font::default(), 48f32))];
        let mut loc = BUTTON_SPACING.0;
        for title in &titles {
            let buttonText =  graphics::Text::new((title.contents(), graphics::Font::default(), 48f32));
            let buttonOutline = graphics::Rect::new(loc, BUTTON_SPACING.1, 2.0*BUTTON_PADDING.0 + buttonText.width(ctx) as f32, 2.0*BUTTON_PADDING.1 + buttonText.height(ctx) as f32);
            let mut button = Button::new(buttonText, buttonOutline);
            if button.text.contents() != "Start Game" {
                button.set_colors(MyColor::Red, MyColor::Red);
                self.buttons[0].push(button);
            } else {
                button.set_colors(MyColor::Blue, MyColor::Green);
                button.outline.y = (SCREEN_SIZE.1 - button.outline.h)/2.0;
                self.buttons[3].push(button);
            }
            
            loc = loc + buttonOutline.w + BUTTON_SPACING.0;
        }
        //GAME SELECTION BUTTONS (buttons[1])
        let mut maxDim = (0, 0);
        //Identify max length for text for all games
        for game in &games {
            let buttonText = graphics::Text::new((game.to_string(), graphics::Font::default(), 48f32));
            maxDim.0 = maxDim.0.max(buttonText.width(ctx));
            maxDim.1 = maxDim.1.max(buttonText.height(ctx));
        }
        //Create buttons for games based on max dimensions so they are equal size
        for i in 0..games.len() {
            let mut titleOutline = self.buttons[0][0].outline;  
            if i == 0 {
                titleOutline = self.buttons[0][0].outline;            
            } else {
                titleOutline = self.buttons[1][i-1].outline;
            }
            let buttonText = graphics::Text::new((games[0].to_string(), graphics::Font::default(), 48f32));
            let X_OFFSET = (titleOutline.w - (2.0*BUTTON_PADDING.0 + maxDim.0 as f32))/2.0;
            let mut button = Button::new(buttonText,
                                             graphics::Rect::new(titleOutline.x + X_OFFSET, 
                                                                 titleOutline.y + titleOutline.h + BUTTON_SPACING.1,
                                                                 2.0*BUTTON_PADDING.0 + maxDim.0 as f32, 
                                                                 2.0*BUTTON_PADDING.1 +maxDim.1 as f32)
                                            );
            button.set_colors(MyColor::Blue, MyColor::Green);
            self.buttons[1].push(button);
        }
        //PLAYER NUMBERS (buttons[1])
        for i in 0..3 {
            let mut titleOutline = self.buttons[0][0].outline;  
            if i == 0 {
                titleOutline = self.buttons[0][1].outline;            
            } else {
                titleOutline = self.buttons[2][i-1].outline;
            }         
            let buttonText = graphics::Text::new((i.to_string(), graphics::Font::default(), 48f32));
            let textDim = (buttonText.width(ctx), buttonText.height(ctx));
            let X_OFFSET = (titleOutline.w - (2.0*BUTTON_PADDING.0 + textDim.0 as f32))/2.0;
            let mut button = Button::new(buttonText,
                                         graphics::Rect::new(titleOutline.x + X_OFFSET, 
                                                             titleOutline.y + titleOutline.h + BUTTON_SPACING.1,
                                                             2.0*BUTTON_PADDING.0 + textDim.0 as f32, 
                                                             2.0*BUTTON_PADDING.1 + textDim.1 as f32)
                                         );
            button.set_colors(MyColor::Blue, MyColor::Green);
            self.buttons[2].push(button);
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