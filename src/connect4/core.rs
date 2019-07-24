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
// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

/// Constant definition for the connect4 board size: 6x7 cells, row x column
const BOARD_SIZE: (i32, i32) = (6, 7);

/// Constant definition for the pixel size for each square tiles: 32x32 pixels
const BOARD_CELL_SIZE: (i32, i32) = (32, 32);

/// Constant definition for the radius of each playing disc: 14px
const BOARD_DISC_RADIUS: i32 = 14;

/// Constant definition for the border size of the board
const BOARD_BORDER_SIZE: i32 = 4;

const BOARD_TOTAL_SIZE: (f32, f32) = (
        ((BOARD_SIZE.0 * BOARD_CELL_SIZE.0) + BOARD_BORDER_SIZE) as f32,
        ((BOARD_SIZE.1 * BOARD_CELL_SIZE.1) + BOARD_BORDER_SIZE) as f32,
);

/// Constant definition for the screen size of the game window
const SCREEN_SIZE: (f32, f32) = (
    BOARD_TOTAL_SIZE.0 + 32 as f32,
    BOARD_TOTAL_SIZE.1 + 32 as f32,
);

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum MyColor {
    White,
    Blue,
    Red,
}
use MyColor::*;

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
            x: (pos.x * BOARD_CELL_SIZE.0 - ((BOARD_CELL_SIZE.0 - (2 * BOARD_DISC_RADIUS)) / 2)) as f32,
            y: (pos.y * BOARD_CELL_SIZE.1 - ((BOARD_CELL_SIZE.1 - (2 * BOARD_DISC_RADIUS)) / 2)) as f32
        }
    }
}

/// A single cell of the board
struct Cell {
    position: GridPosition,
    filled: bool,
    color: MyColor,
}

impl Cell {
    pub fn new(pos: GridPosition) -> Self {
        Cell {
            position: pos,
            filled: false,
            color: White,
        }
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let circ_color = match self.color {
            White => graphics::WHITE,
            Blue => graphics::Color::from_rgba(0,0,255,255),
            Red => graphics::Color::from_rgba(255,0,0,255),
        };
        let mesh = graphics::MeshBuilder::new()
        .rectangle(
            graphics::DrawMode::fill(),
            self.position.into(),
            graphics::Color::from_rgba(205,133,63,255)
        )
        .circle(
            graphics::DrawMode::fill(),
            self.position,
            BOARD_DISC_RADIUS as f32,
            0.0,
            circ_color
        )
        .build(ctx)?;
        graphics::draw(ctx, &mesh, (Point2 {x: 0.0, y: 0.0},))?;
        Ok(())
    }
}

//Abstraction of a column of cells for connect 4 board
struct Column {
    position: GridPosition,
    cells: Vec<Cell>,
    //Maybe need a 'full' state if all cells in column are filled?
}

impl Column {
    pub fn new(pos: GridPosition) -> Self {
        Column {
            position: pos,
            // Adapted from: https://stackoverflow.com/questions/48021408/how-to-init-a-rust-vector-with-a-generator-function
            cells: (0.. BOARD_SIZE.0).map(|y| Cell::new((pos.x, pos.y + (BOARD_CELL_SIZE.0 * y)).into())).collect(),
        }
    }

    // Calls every Cell's draw fn
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for y in 0 .. BOARD_SIZE.0 as usize {
            self.cells[y].draw(ctx)?;
        }
        Ok(())
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
            columns: (0.. BOARD_SIZE.1).map(|x| Column::new((pos.x + BOARD_BORDER_SIZE + (BOARD_CELL_SIZE.1 * x), pos.y + BOARD_BORDER_SIZE).into())).collect(),
        }
    }

    // Draws Board's rect and columns
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect {
                x: self.position.x as f32, 
                y: self.position.y as f32, 
                w: BOARD_TOTAL_SIZE.0 as f32, 
                h: BOARD_TOTAL_SIZE.1 as f32,
            },
            graphics::WHITE
        )?;
        graphics::draw(ctx, &rectangle, (Point2 {x: 0.0, y: 0.0},))?;

        for x in 0 .. BOARD_SIZE.1 as usize {
            self.columns[x].draw(ctx)?;
        }
        Ok(())
    }
}


struct GameState {
    frames: usize,
    gameLoaded: GameLoaded,
    /// connect4 board
    board: Board,
}

//Implementation based on structure in example from GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
impl GameState {
    fn new(ctx: &mut Context) -> GameResult<GameState> {
        let board_pos = (10, 10);

        let s = GameState { 
            frames: 0, 
            gameLoaded: GameLoaded::NONE,
            board: Board::new(board_pos.into()),
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

        // Draw Board
        self.board.draw(ctx)?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ggez::ContextBuilder::new("Connect4", "Lane Barton & Andre Mukhsia")
        .window_setup(ggez::conf::WindowSetup::default().title("Game Closet - Connect 4"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = &mut GameState::new(ctx)?;
    event::run(ctx, events_loop, state)
}