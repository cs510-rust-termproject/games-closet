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
const BOARD_BORDER_SIZE: i32 = 4

const BOARD_TOTAL_SIZE: (f32, f32) = (
        (BOARD_SIZE.0 as f32 * BOARD_CELL_SIZE.0 as f32) + BOARD_BORDER_SIZE,
        (BOARD_SIZE.1 as f32 * BOARD_CELL_SIZE.1 as f32) + BOARD_BORDER_SIZE,
)

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
            graphics::Color::from_rgba(205,133,63,255)
        )?
        .circle(
            ctx,
            graphics::DrawMode::fill(),
            self.position.into(),
            BOARD_DISC_RADIUS,
            0.0,
            graphics::WHITE
        )?
        .build(ctx)?;
        graphics::draw(ctx, &mesh, Point2::new(0.0, 0.0))?;
        OK(())
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
            cells: Vec<Cell>::new(),
        }
        for y in 0 .. BOARD_SIZE.0 {
            cells.push(Cell::new(pos.0, pos.1 + (BOARD_CELL_SIZE * y)));
        }
    }

    // Calls every Cell's draw fn
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        for y in 0 .. BOARD_SIZE.0 {
            cells[y].draw(ctx)?;
        }
        OK(())
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
            columns: Vec<Column>::new(),
        }
        for x in 0 .. BOARD_SIZE.1 {
            columns.push(Column::new(pos.0 + BOARD_BORDER_SIZE + (BOARD_CELL_SIZE * x), pos.1 + BOARD_BORDER_SIZE));
        }
    }

    // Draws Board's rect and columns
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let rectangle = graphics::Mesh::new_rectangle(
            ctx, 
            graphics::DrawMode::fill(), 
            graphics::Rect(position.0, position.1, BOARD_TOTAL_SIZE.0, BOARD_TOTAL_SIZE.1), 
            graphics::WHITE
        )?;
        graphics.draw(ctx, &rectangle, Point2::new(0.0, 0.0))?;

        for x in 0 .. BOARD_SIZE.1 {
            columns[x].draw(ctx)?;
        }
        Ok(())
    }
}

        }
    }
}

