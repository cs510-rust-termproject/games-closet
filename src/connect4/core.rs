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

const BOARD_BORDER_SIZE: i32 = 4

const BOARD_TOTAL_SIZE: (f32, f32) = (
        (BOARD_SIZE.0 as f32 * BOARD_CELL_SIZE.0 as f32) + BOARD_BORDER_SIZE,
        (BOARD_SIZE.1 as f32 * BOARD_CELL_SIZE.1 as f32) + BOARD_BORDER_SIZE,
)

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
        GridPosition { x, y }
    }
}

/// From trait converting i32 tuples to GridPosition
impl From<(i32, i32)> for GridPosition {
    fn from(pos: (i32, i32)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
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
impl From<GridPosition> for Point2 {
    fn from(pos: GridPosition) -> Self {
        Point2::new(
            (GridPosition.x as f32 * BOARD_CELL_SIZE.0 as f32 - ((BOARD_CELL_SIZE - (2 * BOARD_DISC_RADIUS)) as f32 / 2)),
            (GridPosition.y as f32 * BOARD_CELL_SIZE.1 as f32 - ((BOARD_CELL_SIZE - (2 * BOARD_DISC_RADIUS)) as f32 / 2)),
        )
    }
}

/// A single cell of the board
struct Cell {
    position: GridPosition,
    filled: bool,
    color: MyColor;
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
        let mut circ_color;
        match self.color {
            White => graphics::WHITE,
            Blue => graphics::Color::from_rgba(0,0,255,255),
            Red => graphics::Color::from_rgba(255,0,0,255),
        }
        let mesh = MeshBuilder::new()
        .rectangle(
            ctx,
            graphics::DrawMode::fill(),
            self.position.into(),
            graphics::Color::from_rgba(205,133,63,255),
        )?
        .circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position.into(),
            BOARD_DISC_RADIUS,
            0.0,
            graphics::WHITE
        )
        .build(ctx)?;
        graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0))?;
        OK(())
    }
}
