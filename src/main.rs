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
use ggez::input::mouse::MouseButton;
use ggez::{Context, GameResult};
use connect4::core::MyColor;
use connect4::button::{BUTTON_PADDING, BUTTON_SPACING, Button};

///Constant dimensions for screen
const SCREEN_SIZE: (f32, f32) = (910.0, 500.0); //Note - this is hard coded based on the known title sizes and should be adjusted if titles change

/// Enum representing which game is loaded
enum GameLoaded {
    NONE,
    CONNECT4,
}

//To_string implementation, adapted from https://doc.rust-lang.org/rust-by-example/conversion/string.html
impl fmt::Display for GameLoaded {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            GameLoaded::NONE => "None",
            GameLoaded::CONNECT4 => "Connect 4",
        };
        write!(f, "{}", text)
    }
}

impl From<String> for GameLoaded {
    fn from(text: String) -> Self {
        match text.as_str() {
            "Connect 4" => GameLoaded::CONNECT4,
            _ => GameLoaded::NONE
        }
    }
}

///
/// A struct that contains the states for the main menu
///
/// # Fields
/// * frames                = Integer counter for the number of times the update method is called; helps gauge time
/// * buttons               = Vector of Vector of Buttons on the main menu screen, where buttons are classified as follows:
///                               -buttons[0] represent titles for the menu and should not be highlighted/change state
///                               -buttons[1] represent game options for the first column, "Select Game"
///                               -buttons[2] represent options for the second column, "Players" (or number of human players)
///                               -buttons[3] represents the third "column", the "Start Game" button
/// * buttons_available     = Positive integer value representing how many of menu columns are to be displayed. For example, if this value is 2,
///                           the the first two columns should both be displayed which the final "Start Game" column  should not be visible. This
///                           value should never be less than 1 so titles and at least one set of options are displayed
/// * game_loaded           = GameLoaded struct indicating what is loaded     
/// * connect4_state        = GameState for a Connect4 game in `src/connect4/core.rs`. Used when Connect4 is being played      
/// * main_screen_is_active = Boolean indicating if main menu is loaded or not       
///
struct GameState {
    frames: usize,
    buttons: Vec<Vec<Button>>,
    buttons_available: usize,
    game_loaded: GameLoaded,
    connect4_state: connect4::core::GameState,
    main_screen_is_active: bool,
}

impl event::EventHandler for GameState {
    ///Main update for menu - handles actions either for main menu or game being played
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.frames += 1; //"Timer"
        if self.main_screen_is_active {
            //Only allow buttons to be active if previous options selected
            for i in 0..self.buttons.len() {
                for j in 0..self.buttons[i].len() {
                    //println!("{}: ({},{}) {}", self.buttons[i][j].text.contents(), i, j, i <= self.buttons_available);
                    self.buttons[i][j].active = i <= self.buttons_available;
                    self.buttons[i][j].selected = (i <= self.buttons_available) && self.buttons[i][j].selected;
                }
            }
            //Check if "Start Game" selected, change context accordingly
            if self.buttons[self.buttons.len()-1][0].selected {
                let game_index = self.is_button_in_column_selected(1);
                if game_index >= 0 {
                    self.game_loaded = GameLoaded::from(self.buttons[1][game_index as usize].text.contents());
                } else {
                    println!("No game loaded to start!");
                    return Ok(());
                }
                let players_index = self.is_button_in_column_selected(2);
                if players_index < 0 {
                    println!("No player number selected to start games!");
                    return Ok(());
                } 
                //Create new connect4 state
                self.connect4_state = connect4::core::GameState::new(_ctx, 2-players_index);
                //Change windows size for connect4
                graphics::set_mode(_ctx, ggez::conf::WindowMode::default().dimensions(connect4::core::SCREEN_SIZE.0, connect4::core::SCREEN_SIZE.1))?;
                graphics::set_screen_coordinates(_ctx, graphics::Rect::new(0.0, 0.0, connect4::core::SCREEN_SIZE.0+10.0, connect4::core::SCREEN_SIZE.1+10.0))?;
                self.main_screen_is_active = false;
                self.connect4_state.turn_indicator.change_team(1);
            }
        } else {
            self.connect4_state.update(_ctx)?;
        }

        Ok(())
    }

    ///Method that draws all buttons on grid
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.main_screen_is_active {
            graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
            self.draw_buttons(ctx);
            graphics::present(ctx)?;
        } else {
            self.connect4_state.draw(ctx)?;
        }
        Ok(())
    }

    ///Method to update state of all buttons if mouse moves, either for main menu or active game
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        if self.main_screen_is_active {
            for i in 0..self.buttons.len() {
                for j in 0..self.buttons[i].len() {
                    self.buttons[i][j].is_button_under_mouse(_ctx);
                }
            }
        } else {
            self.connect4_state.mouse_motion_event(_ctx, _x, _y, _dx, _dy);
        }
    }

    ///Method to update state of all buttons if mouse pressed down, either for main menu or active game
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if self.main_screen_is_active {
            //Check whether buttons are highlighted, updated states accordingly
            for i in 0..self.buttons.len() {
                for j in 0..self.buttons[i].len() {
                    self.buttons[i][j].is_button_under_mouse(_ctx);
                }
            }
        } else {
            self.connect4_state.mouse_button_down_event(_ctx, _button, _x, _y);
        }
    }

    ///Method to update state of all buttons if moves up from a pressed state, either for main menu or active game 
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if self.main_screen_is_active {
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
        } else if self.connect4_state.mouse_button_up_event(_ctx, _button, _x, _y) {
            self.main_screen_is_active = true;

            //Need to reset button selection, otherwise it only "resets" connect4
            for i in 1..self.buttons.len() {
                for j in 0..self.buttons[i].len() {
                    self.buttons[i][j].selected = false;
                    self.buttons_available = 1;
                }
            }
            //Change windows size for main menu
            let result = graphics::set_mode(_ctx, ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1));
            match result {
                Ok(_) => (),
                Err(e) => (println!("Error drawing button: {:?}", e)),
            };

            let result = graphics::set_screen_coordinates(_ctx, graphics::Rect::new(0.0, 0.0, SCREEN_SIZE.0+10.0, SCREEN_SIZE.1+10.0));
            match result {
                Ok(_) => (),
                Err(e) => (println!("Error drawing button: {:?}", e)),
            };
        }
    }

}

//Implementation based on structure in example from GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let mut s = GameState { frames: 0, buttons: Vec::<Vec::<Button>>::new(), buttons_available:1, game_loaded: GameLoaded::NONE, connect4_state: connect4::core::GameState::new(ctx, 0), main_screen_is_active: true, };
        s.create_buttons(ctx);
        Ok(s)
    }

    ///Method to print organized list of buttons
    fn draw_buttons(&mut self, ctx: &mut Context) {
        for i in 0..self.buttons.len() {
            for j in 0..self.buttons[i].len() {
                let result = self.buttons[i][j].draw(ctx);
                match result {
                    Ok(_) => (),
                    Err(e) => (println!("Error drawing button: {:?}", e)),
                };
            }
        }
    }

    ///Method to determine if a button in a menu column is selected. Returns index of a highlighted button or -1 if none is highlighted
    fn is_button_in_column_selected(&self, col: usize) -> i32 {
        if col > self.buttons.len() {
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

    ///Function to initialize buttons vector for the main menu. Buttons are 
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
            let button_text =  graphics::Text::new((title.contents(), graphics::Font::default(), 48f32));
            let button_outline = graphics::Rect::new(loc, BUTTON_SPACING.1, 2.0*BUTTON_PADDING.0 + button_text.width(ctx) as f32, 2.0*BUTTON_PADDING.1 + button_text.height(ctx) as f32);
            let mut button = Button::new(button_text, button_outline);
            if button.text.contents() != "Start Game" {
                button.set_colors(MyColor::Red, MyColor::Red);
                self.buttons[0].push(button);
            } else {
                button.set_colors(MyColor::Blue, MyColor::Green);
                button.outline.y = (SCREEN_SIZE.1 - button.outline.h)/2.0;
                self.buttons[3].push(button);
            }
            
            loc = loc + button_outline.w + BUTTON_SPACING.0;
        }
        //GAME SELECTION BUTTONS (buttons[1])
        let mut max_dim = (0, 0);
        //Identify max length for text for all games
        for game in &games {
            let button_text = graphics::Text::new((game.to_string(), graphics::Font::default(), 48f32));
            max_dim.0 = max_dim.0.max(button_text.width(ctx));
            max_dim.1 = max_dim.1.max(button_text.height(ctx));
        }
        //Create buttons for games based on max dimensions so they are equal size
        for i in 0..games.len() {
            let mut title_outline = if i == 0 { self.buttons[0][0].outline } else { self.buttons[1][i-1].outline };
            let button_text = graphics::Text::new((games[0].to_string(), graphics::Font::default(), 48f32));
            let x_offset = (title_outline.w - (2.0*BUTTON_PADDING.0 + max_dim.0 as f32))/2.0;
            let mut button = Button::new(button_text,
                                             graphics::Rect::new(title_outline.x + x_offset, 
                                                                 title_outline.y + title_outline.h + BUTTON_SPACING.1,
                                                                 2.0*BUTTON_PADDING.0 + max_dim.0 as f32, 
                                                                 2.0*BUTTON_PADDING.1 +max_dim.1 as f32)
                                            );
            button.set_colors(MyColor::Blue, MyColor::Green);
            self.buttons[1].push(button);
        }
        //PLAYER NUMBERS (buttons[2])
        for i in 0..3 {
            let mut title_outline = if i == 0 { self.buttons[0][1].outline } else { self.buttons[2][i-1].outline };      
            let button_text = graphics::Text::new((i.to_string(), graphics::Font::default(), 48f32));
            let text_dim = (button_text.width(ctx), button_text.height(ctx));
            let x_offset = (title_outline.w - (2.0*BUTTON_PADDING.0 + text_dim.0 as f32))/2.0;
            let mut button = Button::new(button_text,
                                         graphics::Rect::new(title_outline.x + x_offset, 
                                                             title_outline.y + title_outline.h + BUTTON_SPACING.1,
                                                             2.0*BUTTON_PADDING.0 + text_dim.0 as f32, 
                                                             2.0*BUTTON_PADDING.1 + text_dim.1 as f32)
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
}