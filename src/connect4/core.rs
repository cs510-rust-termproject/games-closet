// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
extern crate ggez;

use ggez::{event, graphics, Context, GameResult};
use ggez::mint::Point2;
use ggez::input::mouse;
use ggez::input::mouse::MouseButton;
use connect4::ai::AI;

/// Constant definition for the connect4 board size: 6x7 cells, row x column
pub const BOARD_SIZE: (i32, i32) = (6, 7);

/// Constant definition for the pixel size for each square tiles: 32x32 pixels
const BOARD_CELL_SIZE: (i32, i32) = (64, 64);

/// Constant definition for the radius of each playing disc: 14px
const BOARD_DISC_RADIUS: i32 = 28;

/// Constant definition for the border size of the board
const BOARD_BORDER_SIZE: i32 = 32;

const BOARD_TOTAL_SIZE: (f32, f32) = (
        ((BOARD_SIZE.1 * BOARD_CELL_SIZE.0) + BOARD_BORDER_SIZE) as f32,
        ((BOARD_SIZE.0 * BOARD_CELL_SIZE.0) + BOARD_BORDER_SIZE) as f32,
);

// Testing dynamic Turn Indicator Box size, further decrement by width / 2
const TURN_INDICATOR_POS_OFFSET: (i32, i32) = (10 + (BOARD_TOTAL_SIZE.0 / 2.0) as i32, 10);

const TURN_INDICATOR_BOX_SIZE_OFFSET: (i32, i32) = (16, 32);

const TURN_INDICATOR_FONT_SIZE: i32 = 48;

const COLUMN_SELECTION_INDICATOR_POS_OFFSET: (i32, i32) = (10, 10 + TURN_INDICATOR_POS_OFFSET.1 + TURN_INDICATOR_BOX_SIZE_OFFSET.1 + TURN_INDICATOR_FONT_SIZE);

const BOARD_POS_OFFSET: (i32, i32) = (10, 10 + COLUMN_SELECTION_INDICATOR_POS_OFFSET.1 + BOARD_CELL_SIZE.1);

const RESET_BUTTON_OFFSET: (i32, i32) = (10, 10);

/// Constant definition for the screen size of the game window
const SCREEN_SIZE: (f32, f32) = (
    BOARD_TOTAL_SIZE.0 + (BOARD_POS_OFFSET.0 as f32),
    BOARD_TOTAL_SIZE.1 + (BOARD_POS_OFFSET.1 as f32),
);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum MyColor {
    White,
    Blue,
    Red,
    Green
}

impl MyColor {
    pub fn get_draw_color(&self) -> ggez::graphics::Color {
        let circ_color = match self {
            MyColor::White => graphics::WHITE,
            MyColor::Blue => graphics::Color::from_rgba(0,0,255,255),
            MyColor::Red => graphics::Color::from_rgba(255,0,0,255),
            MyColor::Green => graphics::Color::from_rgba(0,255,0,255)
        };
        circ_color
    }
}

//use MyColor::*;

/// Struct determines position on the board
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    /// Constructor for GridPosition
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition {x, y}
    }
}

/// From trait converting i32 tuples to GridPosition
impl From<(i32, i32)> for GridPosition {
    fn from(pos: (i32, i32)) -> Self {
        GridPosition {x: pos.0, y: pos.1}
    }
}

/*
/// From trait converting GridPosition to Rect; Used for drawing the Cells of the board
impl From<GridPosition> for graphics::Rect {
    fn from(pos: GridPosition) -> Self {
        graphics::Rect::new_i32(
            pos.x * BOARD_CELL_SIZE.0,
            pos.y * BOARD_CELL_SIZE.1,
            BOARD_CELL_SIZE.0,
            BOARD_CELL_SIZE.1,
        )
    }
}

/// From trait converting GridPosition to Point2; Used for drawing playing discs on the board
impl From<GridPosition> for Point2<f32> {
    fn from(pos: GridPosition) -> Self {
        Point2 {
            x: pos.x as f32,
            y: pos.y as f32
        }
    }
}
*/

/// A single cell of the board
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Cell {
    position: GridPosition,
    team: i32,
    color: MyColor,
}

impl Cell {
    pub fn new(pos: GridPosition) -> Self {
        Cell {
            position: pos,
            team: 0,
            color: MyColor::White,
        }
    }

    //Using example from 03_drawing.rs
    fn draw <'a>(&self, mb: &'a mut graphics::MeshBuilder) -> &'a mut graphics::MeshBuilder {
        let circ_color = self.color.get_draw_color();
        //println!("Building mesh\n");
        
        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect {
                    x: self.position.x as f32, 
                    y: self.position.y as f32, 
                    w: BOARD_CELL_SIZE.0 as f32, 
                    h: BOARD_CELL_SIZE.1 as f32,
            },
            graphics::Color::from_rgba(205,133,63,255)
        );
        mb.rectangle(
            graphics::DrawMode::stroke(1.0),
            graphics::Rect {
                    x: self.position.x as f32, 
                    y: self.position.y as f32, 
                    w: BOARD_CELL_SIZE.0 as f32, 
                    h: BOARD_CELL_SIZE.1 as f32,
            },
            graphics::BLACK
        );
        mb.circle(
            graphics::DrawMode::fill(),
            Point2 {
                x: (self.position.x + (BOARD_CELL_SIZE.0 / 2)) as f32,
                y: (self.position.y + (BOARD_CELL_SIZE.1 / 2)) as f32
            },
            BOARD_DISC_RADIUS as f32,
            2.0,
            circ_color
        );
        mb
    }

    fn fill(&mut self, team: i32, color: MyColor) {
        self.team = team;
        self.color = color;
    }
}

//Abstraction of a column of cells for connect 4 board
#[derive(Clone, PartialEq, Eq, Debug)]
struct Column {
    position: GridPosition,
    cells: Vec<Cell>,
    height: usize
}

impl Column {
    pub fn new(pos: GridPosition) -> Self {
        Column {
            position: pos,
            // Adapted from: https://stackoverflow.com/questions/48021408/how-to-init-a-rust-vector-with-a-generator-function
            // Rev() method from https://stackoverflow.com/questions/25170091/how-to-make-a-reverse-ordered-for-loop-in-rust; used because columns drawn from top down
            cells: (0.. BOARD_SIZE.0).rev().map(|y| Cell::new((pos.x, pos.y + (BOARD_CELL_SIZE.0 * y)).into())).collect(),
            height: 0
        }
    }

    // Calls every Cell's draw fn
    fn draw<'a>(&self, mb: &'a mut graphics::MeshBuilder) -> &'a mut graphics::MeshBuilder {
        for cell in &self.cells {
            cell.draw(mb);
            //println!("Cell draw called\n");
        }
        mb
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn is_full(&self) -> bool {
        self.height >= BOARD_SIZE.0 as usize
    }

    //Method to determine if a location (presumed to be the mouse) is inside the column or one cell above (for drop)
    pub fn is_mouse_over(&self, loc: Point2<f32>) -> bool {
        graphics::Rect::new(self.position.x as f32, (self.position.y-(BOARD_CELL_SIZE.1*4/3)) as f32, BOARD_CELL_SIZE.0 as f32, 8.0*BOARD_CELL_SIZE.1 as f32).contains(loc)
    }

    /// Inserts a team's disc of a particular color into a cell
    /// Returns true if disc successfully inserted
    /// Returns false if column is full
    pub fn insert(&mut self,team: i32, color: MyColor) -> bool {
        if self.is_full() {
            false
        } else {
            self.cells[self.height].fill(team, color);
            self.height += 1;
            true
        }
    }

    pub fn reset(&mut self) {
        self.height = 0;
        for cell in &mut self.cells {
            cell.fill(0, MyColor::White);
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    position: GridPosition,
    columns: Vec<Column>,
}

impl Board {
    pub fn new(pos: GridPosition) -> Self {
        Board {
            position: pos,
            columns: (0.. BOARD_SIZE.1).map(|x| Column::new((pos.x + (BOARD_BORDER_SIZE / 2) + (BOARD_CELL_SIZE.1 * x), pos.y + (BOARD_BORDER_SIZE/2)).into())).collect(),
        }
    }

    // Builds Board's rect mesh and columns
    fn draw <'a>(&self, mb: &'a mut graphics::MeshBuilder) -> &'a mut graphics::MeshBuilder {
        mb.rectangle(
            graphics::DrawMode::fill(), 
            graphics::Rect {
                x: self.position.x as f32, 
                y: self.position.y as f32, 
                w: BOARD_TOTAL_SIZE.0 as f32, 
                h: BOARD_TOTAL_SIZE.1 as f32,
            },
            graphics::WHITE
        );
        mb.rectangle(
            graphics::DrawMode::stroke(1.0),
            graphics::Rect {
                    x: self.position.x as f32, 
                    y: self.position.y as f32, 
                    w: BOARD_TOTAL_SIZE.0 as f32, 
                    h: BOARD_TOTAL_SIZE.1 as f32,
            },
            graphics::Color::from_rgba(0, 255, 0, 255),
        );

        // TODO: Need to try to restructure to pass a meshBuilder, build the columns and cells, build it and then draw it
        for column in &self.columns {
            column.draw(mb);
        }
        mb
    }

    //Helper method to get the index of the column that is under the mouse (loc), or -1 if no column is highlighted
    pub fn get_highlighted_column(&self, loc: Point2<f32>) -> i32 {
        for i in 0..self.columns.len() {
            if self.columns[i].is_mouse_over(loc) {
                return i as i32;
            }
        }
        -1
    }

    pub fn on_board(&self, pos: GridPosition) -> bool {
        pos.x >= 0 && pos.x < BOARD_SIZE.1 && pos.y >= 0 && pos.y < BOARD_SIZE.0
    }

    pub fn get_column_height(&self, col: usize) -> usize {
        self.columns.get(col).unwrap().get_height()
    }

    pub fn is_column_full(&self, col: usize) -> bool {
        self.columns.get(col).unwrap().is_full()
    }

    pub fn get_cell_team(&self, pos: GridPosition) -> i32 {
        self.columns.get(pos.x as usize).unwrap()
            .cells.get(pos.y as usize).unwrap()
            .team
    }

    //Method to get a "max" run including a starting point in a target direction for a given team.
    //Accounts for runs towards and away from direction, but allows one space between tiles of the target team in
    //target direction but no spaces in reverse direction. 
    //The min value is 1; the max value returned is 4 even if a run is longer. If a space is used, the max returned value is 3 
    //(as the space presumably prevents an actual run of 4). Cases with a run of 4 prior to space will return 4, except for edge 
    //case where run goes from start and then completely in reverse direction. This can be caught by calling this method with reverse 
    //direction
    fn get_run_in_direction(&self, start: GridPosition, dir: GridPosition, team: i32) -> i32 {
        let mut dir_active = true;
        let mut rev_active = true;
        let mut dir_spaces_used = 0;
        let mut rev_space_used = false;
        let mut run_len = 1i32; //Start with dropped token
        let mut potential_len = 1; //Assume potential length starts at 1 for dropped token
        let mut i = 1; //Start one beyond dropped token
        while run_len <= 4 && (dir_active || rev_active) {
            dir_active = dir_active && self.on_board(GridPosition::new(start.x+i*dir.x, start.y+i*dir.y));
            rev_active = rev_active && self.on_board(GridPosition::new(start.x-i*dir.x, start.y-i*dir.y));
            //Do reverse case first for edge case of AASA_A is treated as a run of 4 and not 3 with a space
            if rev_active {
                let val = self.get_cell_team(GridPosition::new(start.x-i*dir.x, start.y-i*dir.y));
                //If token not for team in cell, end of search in rev direction
                if val != 0 && val != team {
                    rev_active = false;
                //If no spaces used, either add to run_len and/or potential_len depending on if cell is empty or matches team
                } else if !rev_space_used {
                    if val == team {
                        run_len += 1;
                    } else {
                        rev_space_used = true;
                    }
                    potential_len += 1;
                //If space in rev direction found, just add to potential run len to track that
                } else {
                    potential_len += 1;
                }
            }
            if dir_active {
                let val = self.get_cell_team(GridPosition::new(start.x+i*dir.x, start.y+i*dir.y));
                //If token not for team in cell, end of search in target direction
                if val != 0 && val != team {
                    dir_active = false;
                //If 0 or 1 spaces in target direction used, either add to run_len and/or potential_run depending on if cell is empty or matches team
                } else if dir_spaces_used <=  1 {
                    //If you have a contiguous run of 4 with no spaces, immediately return because a winning run has been found!
                    if run_len >= 4 && dir_spaces_used == 0 {
                        return 4i32;
                    } else if val == team {
                        run_len += 1;   
                    } else {
                        dir_spaces_used +=1;
                    }
                    potential_len += 1;
                //If more than one space in target direction used, only add to potential length for non-enemy cells
                } else {
                    potential_len += 1;
                }
            }
            i += 1;
        }
        //If the potential of the run is not 4 or more, return 0 because it is not a viable run
        if potential_len < 4 {
            return 0i32;
        //Otherwise, return the minimum of the run_len and 4 (if no spaces) or 3 (if one space used)
        } else {
            if dir_spaces_used > 0 {
                run_len.min(3)
            } else {
                run_len.min(4)
            }
        }
    }

    //Method to return an array of runs from a start location for a given team, where array[i] returns the number of runs
    //of length i-1. Accounts for all eight directions, but may have false duplicates (e.g. a run BAAAB will return have two
    //runs of length 3 for team A even though technically its the same run)
    pub fn get_runs_from_point(&self, start: GridPosition, team: i32) -> [i32;4] {
        let mut output = [0i32;4];
        let directions = vec![(1, 0), (1, 1), (0, 1), (-1, 1)];
        for dir in directions {
            let a = self.get_run_in_direction(start, GridPosition::new(dir.0, dir.1), team)-1;
            let b = self.get_run_in_direction(start, GridPosition::new(-1*dir.0, -1*dir.1), team)-1;
            if a >= 0 {
                output[a as usize] += 1;
            }
            if b >= 0 {
                output[b as usize] += 1;
            }
        }
        output
    }

    /// Inserts a team's disc of a particular color into a cell
    /// Returns true if disc successfully inserted
    /// Returns false if column is full
    pub fn insert(&mut self, position: i32, team: i32, color: MyColor) -> bool {
        self.columns[position as usize].insert(team, color)
    }
    
    pub fn reset(&mut self) {
        for column in &mut self.columns {
            column.reset();
        }
    }
}

pub struct TurnIndicator {
    gameover: bool,
    team: i32,
}

impl TurnIndicator {
    pub fn new() -> Self {
        TurnIndicator {
            gameover: false,
            team: 0,
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut text: graphics::Text;
        if self.gameover {
            if self.team == 0 {
                text = graphics::Text::new(("Game Draw!", graphics::Font::default(), TURN_INDICATOR_FONT_SIZE as f32));
            } else {
                text = graphics::Text::new((format!("Player {} wins!", self.team), graphics::Font::default(), TURN_INDICATOR_FONT_SIZE as f32));
            }
        } else {
            if self.team == 0 {
                text = graphics::Text::new(("Paused", graphics::Font::default(), TURN_INDICATOR_FONT_SIZE as f32));
            } else {
                text = graphics::Text::new((format!("Player {}'s turn", self.team), graphics::Font::default(), TURN_INDICATOR_FONT_SIZE as f32));
            }
        }

        let dim = &text.dimensions(ctx);
        let pos = Point2{
            x: TURN_INDICATOR_POS_OFFSET.0 as f32 - (dim.0 as f32 / 2.0) as f32, 
            y: TURN_INDICATOR_POS_OFFSET.1 as f32
        };

        let textbox = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(),             
            graphics::Rect {
                x: pos.x, 
                y: pos.y, 
                w: dim.0 as f32 + TURN_INDICATOR_BOX_SIZE_OFFSET.0 as f32,
                h: dim.1 as f32 + TURN_INDICATOR_BOX_SIZE_OFFSET.1 as f32,
            },
            graphics::Color::from_rgba(205,133,63,255),
        )?;
        graphics::draw(ctx, &textbox, (Point2 {x: 0.0, y: 0.0},))?;
        graphics::draw(ctx, &text, (Point2 {x: pos.x + TURN_INDICATOR_BOX_SIZE_OFFSET.0 as f32 / 2.0, y: pos.y + TURN_INDICATOR_BOX_SIZE_OFFSET.1 as f32 / 2.0},))?;
        Ok(())
    }

    pub fn change_team(&mut self, team: i32) {
        self.team = team;
    }

    pub fn game_ends(&mut self) {
        self.gameover = true;
    }

    pub fn reset(&mut self) {
        self.team = 0;
        self.gameover = false;
    }
}

// Button struct from original main
pub struct Button {
    text: graphics::Text,
    outline: graphics::Rect,
    highlighted: bool,
}

impl Button {
    pub fn new(text: graphics::Text, dim: graphics::Rect) -> Button {
        Button { text: text, outline: dim, highlighted: false}
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let textbox = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(),             
            self.outline,
            graphics::Color::from_rgba(133,0,0,255),
        )?;
        graphics::draw(ctx, &textbox, (Point2 {x: 0.0, y: 0.0},))?;
        graphics::draw(ctx, &self.text, (Point2 {x: RESET_BUTTON_OFFSET.0 as f32, y: RESET_BUTTON_OFFSET.1 as f32},))?;
        Ok(())
    }

    pub fn is_button_under_mouse(&mut self, ctx: &mut Context) -> bool {
        let mouse_loc = mouse::position(ctx);
        if self.outline.contains(mouse_loc)  {
            self.highlighted = true;
        } else {
            self.highlighted = false;
        }
        self.highlighted
    }

}

pub struct GameState {
    frames: usize,
    ai_players: Vec<AI>,
    pub board: Board,
    team_colors: Vec<MyColor>,
    pub turnIndicator: TurnIndicator,
    pub highlighted_column: i32,
    mouse_disabled: bool,
    gameover: bool,
    pub reset_button: Button,

}

//Implementation based on structure in example from GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
impl GameState {
    fn new(ctx: &mut Context, players: i32) -> GameResult<GameState> {
        let board_pos = BOARD_POS_OFFSET;
        let text = graphics::Text::new("Reset");
        let text_width = text.width(ctx) as f32;
        let text_height = text.height(ctx) as f32;
        let mut bots = Vec::<AI>::new();
        for i in 0..players {
            bots.push(AI::new(2-i, 3));
        }
        let s = GameState { 
            frames: 0, 
            ai_players: bots,
            board: Board::new(board_pos.into()),
            team_colors: vec![MyColor::White, MyColor::Red, MyColor::Blue],
            turnIndicator: TurnIndicator::new(),
            highlighted_column: -1,
            mouse_disabled: false,
            gameover: false,
            reset_button: Button::new(text, graphics::Rect::new(RESET_BUTTON_OFFSET.0 as f32, RESET_BUTTON_OFFSET.1 as f32, text_width, text_height)),
        };
        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if !self.gameover {
            //Draw state check
            let mut full_column = 0;
            for column_index in 0..self.board.columns.len() {
                if !self.board.is_column_full(column_index) {
                    break;
                }
                full_column += 1;
            }
            if full_column == 7 {
                //All columns full - proceed to Gameover - Draw state
                println!("All columns full; Game Draw!");
                self.gameover = true;
                self.mouse_disabled = true;
                self.turnIndicator.change_team(0);
                self.turnIndicator.game_ends();
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //Draw screen background
        graphics::clear(ctx, graphics::BLACK);
        let mut mb = graphics::MeshBuilder::new();
        //Draw disc over current column
        if self.highlighted_column >= 0 {
            mb.circle(
                graphics::DrawMode::fill(),
                Point2 {
                    x: (self.board.columns[self.highlighted_column as usize].position.x + (BOARD_CELL_SIZE.0 / 2)) as f32,
                    y: (self.board.position.y - (BOARD_CELL_SIZE.1 /2)) as f32
                },
                BOARD_DISC_RADIUS as f32,
                2.0,
                self.team_colors[self.turnIndicator.team as usize].get_draw_color()
            );
        }
        //Draw Board
        let mesh = self.board.draw(&mut mb).build(ctx)?;
        graphics::draw(ctx, &mesh, (Point2 {x: 0.0, y: 0.0},))?;

        //Draw turn indicator
        self.turnIndicator.draw(ctx)?;

        //Draw reset button
        self.reset_button.draw(ctx)?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        if !self.mouse_disabled {
            let was_highlighted = self.highlighted_column;
            self.highlighted_column = self.board.get_highlighted_column(mouse::position(_ctx));
            //Log ONLY switches between columns (otherwise lot of logs to console)
            if was_highlighted != self.highlighted_column {
                println!("Mouse moved to col {}", self.highlighted_column);
            }
        }
    }

    fn mouse_button_down_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if !self.mouse_disabled {
            self.highlighted_column = self.board.get_highlighted_column(mouse::position(_ctx));
        }
    }

    //Todo: If mouse_motion_event is enabled, this will always drop the token (i.e. not click away to undo move). Undetermined if this is desired or not
    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        if !self.mouse_disabled {
            let was_highlighted = self.highlighted_column;
            self.highlighted_column = self.board.get_highlighted_column(mouse::position(_ctx));
            if was_highlighted == self.highlighted_column && self.highlighted_column >= 0 {
                self.mouse_disabled = true;
                if self.board.insert(self.highlighted_column, self.turnIndicator.team, self.team_colors[self.turnIndicator.team as usize]) {
                    println!("Team {} drops token in col {}", self.turnIndicator.team, self.highlighted_column);
                    
                    //game state check
                    let runs = self.board.get_runs_from_point(GridPosition::new(self.highlighted_column, self.board.get_column_height(self.highlighted_column as usize) as i32 - 1), self.turnIndicator.team);
                    println!("runs: {:?}", runs);
                    if runs[3] > 0 {    //Four Connected - Proceed to Gameover - Win/Loss state
                        println!("4 Connected for player {}; Game ends", self.turnIndicator.team);
                        self.gameover = true;
                        self.turnIndicator.game_ends();
                    } else {
                        self.turnIndicator.team = self.turnIndicator.team%2+1; //Change to other team's turn
                    }
                }
                if !self.gameover {
                    self.mouse_disabled = false;
                }
            } 
        }

        if self.reset_button.is_button_under_mouse(_ctx) {
            println!("Reset button pressed; Board reset");
            self.board.reset();
            self.turnIndicator.reset();
            self.turnIndicator.change_team(1);
            self.gameover = false;
            self.mouse_disabled = false;
        }
    }
}

pub fn main(num_players: i32) -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("Connect4", "Lane Barton & Andre Mukhsia")
        .window_setup(ggez::conf::WindowSetup::default().title("Game Closet - Connect 4"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut GameState::new(ctx, num_players)?;
    state.turnIndicator.change_team(1); //Start with player 1
    event::run(ctx, events_loop, state)
}




#[cfg(test)]
mod core_tests {
    use super::*;
    mod Board {
        use super::*;
        use connect4::core::Board;

        //Method to create a board state from a set of vectors, where 0 is empty and 1 or 2 team tokens
        //Note that input is board[column][row], so if you want to add a team 1 token in column 4, row 0, then
        //the board input should have board[4][0] = 1
        fn create_test_board(board: Vec<Vec<i32>>) -> Board {
            let mut output = Board::new(GridPosition{ x: 0, y:0 });
            for i in 0..BOARD_SIZE.1 {
                if (i as usize) < board.len() {
                    let col = board.get(i as usize).unwrap();
                    for j in 0..BOARD_SIZE.0 {
                        if (j as usize) < col.len() {
                            let val = *col.get(j as usize).unwrap();
                            if val > 0 {
                                output.insert(i, val, MyColor::White);
                            }
                        }
                    }
                }
            }
            output
        }

        mod get_run_in_direction { 
            use super::*;

            #[test]
            fn should_find_contiguous_run() {
                let data = vec![vec![0,],
                                vec![0,],
                                vec![0,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![0,]];
                let board = create_test_board(data);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 3);
            }

            #[test]
            fn should_find_not_be_more_than_4() {
                let data = vec![vec![0,],
                                vec![0,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![1,]];
                let board = create_test_board(data);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 4);
            }

            #[test]
            fn should_find_run_with_space() {
                let data = vec![vec![0,],
                                vec![0,],
                                vec![0,],
                                vec![1,],
                                vec![0,],
                                vec![1,],
                                vec![1,]];
                let board = create_test_board(data);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 3);
            }

            #[test]
            fn should_find_not_be_more_than_3_with_space() {
                let data = vec![vec![0,],
                                vec![0,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![0,],
                                vec![1,]];
                let board = create_test_board(data);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 3);
            }

            #[test]
            fn should_not_count_two_spaces() {
                let data = vec![vec![0,],
                                vec![0,],
                                vec![0,],
                                vec![1,],
                                vec![0,],
                                vec![0,],
                                vec![1,]];
                let board = create_test_board(data);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 1);
            }

            #[test]
            fn should_not_count_past_space_in_rev_direction() {
                let data = vec![vec![1,],
                                vec![0,],
                                vec![1,],
                                vec![1,],
                                vec![0,],
                                vec![0,],
                                vec![0,]];
                let board = create_test_board(data);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 2);
            }

            #[test]
            fn should_return_run_of_4_prior_to_space() {
                //Should return 4
                let run1 = vec![vec![0,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![0,],
                                vec![1,]];
                let board = create_test_board(run1);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 4);
                //This should return 4 - handled by rev direction case
                let run2 = vec![vec![0,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![0,],
                                vec![1,]];
                let board = create_test_board(run2);
                assert_eq!(board.get_run_in_direction(GridPosition::new(2, 0), GridPosition::new(1, 0), 1), 4);
                //This should not return 4 - handled by rev direction case
                let run3 = vec![vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![0,],
                                vec![1,],
                                vec![0,]];
                let board = create_test_board(run3);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 3);
            }

            #[test]
            fn returns_0_if_run_of_4_impossible() {
                //Runs 1-3 should return 0, Runs 4-5 should pass due to potential in either direction
                let run1 = vec![vec![0,],
                                vec![2,],
                                vec![1,],
                                vec![1,],
                                vec![1,],
                                vec![2,],
                                vec![1,]];
                let mut board = create_test_board(run1);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 0);
                let run2 = vec![vec![2,],
                                vec![0,],
                                vec![0,],
                                vec![1,],
                                vec![2,],
                                vec![0,],
                                vec![1,]];
                board = create_test_board(run2);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 0);
                let run3 = vec![vec![0,],
                                vec![1,],
                                vec![1,], //Checking here
                                vec![2,],
                                vec![0,],
                                vec![0,],
                                vec![0,]];
                board = create_test_board(run3);
                assert_eq!(board.get_run_in_direction(GridPosition::new(2, 0), GridPosition::new(1, 0), 1), 0);
                let run4 = vec![vec![1,],
                                vec![0,],
                                vec![0,],
                                vec![1,],
                                vec![2,],
                                vec![0,],
                                vec![1,]];
                board = create_test_board(run4);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 1);
                let run5 = vec![vec![2,],
                                vec![0,],
                                vec![0,],
                                vec![1,],
                                vec![0,],
                                vec![2,],
                                vec![1,]];
                board = create_test_board(run5);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1), 1);
            }

            #[test]
            fn should_work_in_all_directions() {
                //Note - the runs may not match in opposite directions!
                let data = vec![vec![1,1,0,1,2,0],
                                vec![1,2,2,1,2,2],
                                vec![2,1,1,2,1,0],
                                vec![2,1,1,1,0,0], //Target is in middle of this column
                                vec![0,0,0,0,0,0],
                                vec![1,1,2,1,2,1],
                                vec![1,1,2,2,2,0]];
                let board = create_test_board(data);
                //Vertical directions - should be 3 in both cases
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(0, 1), 1), 3);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(0, -1), 1), 3);
                //Horizontal directions - should be 0 in both cases since two 2s block potential run of 4
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(1, 0), 1), 0);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(-1, 0), 1), 0);
                //Bottom-left to upper-right diagonal directions - should be 2 going down and 3 going up (since space then token in upper-right dir)
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(1, 1), 1), 3);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(-1, -1), 1), 2);
                //Bottom-right to upper-left diagonal directions - should be 2 going up and 3 going down (since space in down dir, but blocked going up)
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(-1, 1), 1), 2);
                assert_eq!(board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(1, -1), 1), 3);
            }
        }

        mod get_runs_from_point { 
            use super::*;

            #[test]
            fn should_find_runs() {
                let data = vec![vec![1,1,0,1,2,0],
                                vec![1,2,2,1,2,2],
                                vec![2,1,1,2,1,0],
                                vec![2,1,1,1,0,0], //Target is in middle of this column
                                vec![0,0,0,0,0,0],
                                vec![1,1,2,1,2,1],
                                vec![1,1,2,2,2,0]];
                let board = create_test_board(data);
                assert_eq!(board.get_runs_from_point(GridPosition::new(3, 3), 1), [0,2,4,0]);
            }
        }
    }
}
