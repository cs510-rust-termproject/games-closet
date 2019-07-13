// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

/// Constant definition for the connect4 board size: 7x6 cells
const BOARD_SIZE: (i32, i32) = (7, 6);

/// Constant definition for the pixel size for each square tiles: 32x32 pixels
const BOARD_CELL_SIZE: (i32, i32) = (32, 32);

/// Constant definition for the radius of each playing disc: 14px
const BOARD_DISC_RADIUS: i32 = 14;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum Color {
    Blue,
    Red,
}
use Color::*;

/// Struct determines position on the board
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i32,
    y: i32,
    filled: bool,
    color: Color;
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

/// From trait converting GridPosition to Point2; Used for drawing playing discs on the board
impl From<GridPosition> for Point2 {
    fn from(pos: GridPosition) -> Self {
        Point2::new(
            (GridPosition.x as f32 * BOARD_CELL_SIZE.0 as f32 - ((BOARD_CELL_SIZE - (2 * BOARD_DISC_RADIUS)) as f32 / 2)),
            (GridPosition.y as f32 * BOARD_CELL_SIZE.1 as f32 - ((BOARD_CELL_SIZE - (2 * BOARD_DISC_RADIUS)) as f32 / 2)),
        )
    }
}
