// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
extern crate ggez;

use ggez::{event, graphics, Context, GameResult};
use ggez::mint::Point2;

/// Enum representing which game is loaded
enum GameLoaded {
    NONE,
    CONNECT4,
}

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

const BOARD_POS_OFFSET: (i32, i32) = (10, 10 + COLUMN_SELECTION_INDICATOR_POS_OFFSET.1);


/// Constant definition for the screen size of the game window
const SCREEN_SIZE: (f32, f32) = (
    BOARD_TOTAL_SIZE.0 + 32 as f32,
    BOARD_TOTAL_SIZE.1 + (TURN_INDICATOR_BOX_SIZE_OFFSET.1 + TURN_INDICATOR_FONT_SIZE + 32) as f32,
);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum MyColor {
    White,
    Blue,
    Red,
}

//use MyColor::*;

/// Struct determines position on the board
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i32,
    y: i32,
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
        let circ_color = match self.color {
            White => graphics::WHITE,
            Blue => graphics::Color::from_rgba(0,0,255,255),
            Red => graphics::Color::from_rgba(255,0,0,255),
        };
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
            cells: (0.. BOARD_SIZE.0).map(|y| Cell::new((pos.x, pos.y + (BOARD_CELL_SIZE.0 * y)).into())).collect(),
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
        self.height >= self.cells.len()
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

struct Board {
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

    pub fn on_board(&self, pos: GridPosition) -> bool {
        pos.x >= 0 && pos.x < BOARD_SIZE.0 && pos.y >= 0 && pos.y < BOARD_SIZE.1
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
        let mut space_used = false;
        let mut run_len = 1i32; //Start with dropped token
        let mut i = 1; //Start one beyond dropped token
        while run_len <= 4 && (dir_active || rev_active) {
            dir_active = dir_active && self.on_board(GridPosition::new(start.x+i*dir.x, start.y+i*dir.y));
            rev_active = rev_active && self.on_board(GridPosition::new(start.x-i*dir.x, start.y-i*dir.y));
            //Do reverse case first for edge case of AASA_A is treated as a run of 4 and not 3 with a space
            if rev_active {
                if self.get_cell_team(GridPosition::new(start.x-i*dir.x, start.y-i*dir.y)) == team {
                    run_len += 1;
                } else {
                    rev_active = false;
                }
            }
            if dir_active {
                let val = self.get_cell_team(GridPosition::new(start.x+i*dir.x, start.y+i*dir.y));
                if val == team {
                    run_len += 1;
                    //Check for contiguous run of 4 before space, return immediately to prevent odd cases with spaces
                    if !space_used && run_len >= 4 {
                        return 4i32;
                    }
                } else if val == 0 && !space_used {
                    space_used = true;
                } else {
                    dir_active = false;
                }
            }
            i += 1;
        }
        if space_used {
            std::cmp::min(run_len, 3)
        //Todo: Handle case where "run" is blocked on both ends
        } else {
            std::cmp::min(run_len, 4)
        }
    }

    //Method to return an array of runs from a start location for a given team, where array[i] returns the number of runs
    //of length i-1. Accounts for all eight directions, but may have false duplicates (e.g. a run BAAAB will return have two
    //runs of length 3 for team A even though technically its the same run)
    fn get_runs_from_point(&self, start: GridPosition, team: i32) -> [i32;4] {
        let mut output = [0i32;4];
        let directions = vec![(1, 0), (1, 1), (0, 1), (-1, 1)];
        for dir in directions {
            output[(self.get_run_in_direction(start, GridPosition::new(dir.0, dir.1), team)-1) as usize] += 1;
            output[(self.get_run_in_direction(start, GridPosition::new(-1*dir.0, -1*dir.1), team)-1) as usize] += 1;
        }
        output
    }

    /// Inserts a team's disc of a particular color into a cell
    /// Returns true if disc successfully inserted
    /// Returns false if column is full
    pub fn insert(&mut self, position: GridPosition, team: i32, color: MyColor) -> bool {
        self.columns[position.x as usize].insert(team, color)
    }

    pub fn reset(&mut self) {
        for column in &mut self.columns {
            column.reset();
        }
    }
}

struct TurnIndicator {
    team: i32,
}

impl TurnIndicator {
    pub fn new() -> Self {
        TurnIndicator {
            team: 0,
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let mut text: graphics::Text;
        if self.team == 0 {
            text = graphics::Text::new(("Paused", graphics::Font::default(), TURN_INDICATOR_FONT_SIZE as f32));
        } else {
            text = graphics::Text::new((format!("Player {}'s turn", self.team), graphics::Font::default(), TURN_INDICATOR_FONT_SIZE as f32));
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

    pub fn reset(&mut self) {
        self.team = 0;
    }
}

struct GameState {
    frames: usize,
    gameLoaded: GameLoaded,
    /// connect4 board
    board: Board,
    turnIndicator: TurnIndicator,
}

//Implementation based on structure in example from GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let board_pos = BOARD_POS_OFFSET;
        let s = GameState { 
            frames: 0, 
            gameLoaded: GameLoaded::NONE,
            board: Board::new(board_pos.into()),
            turnIndicator: TurnIndicator::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //Draw screen background
        graphics::clear(ctx, graphics::BLACK);
        let mut mb = graphics::MeshBuilder::new();
        // Draw Board
        let mesh = self.board.draw(&mut mb).build(ctx)?;
        graphics::draw(ctx, &mesh, (Point2 {x: 0.0, y: 0.0},))?;

        // Draw turn indicator
        self.turnIndicator.draw(ctx)?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

pub fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("Connect4", "Lane Barton & Andre Mukhsia")
        .window_setup(ggez::conf::WindowSetup::default().title("Game Closet - Connect 4"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut GameState::new(ctx)?;
    event::run(ctx, events_loop, state)
}