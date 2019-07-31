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

/// Constant definition for the screen size of the game window
const SCREEN_SIZE: (f32, f32) = (
    BOARD_TOTAL_SIZE.0 + 32 as f32,
    BOARD_TOTAL_SIZE.1 + 32 as f32,
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
            x: (pos.x * BOARD_CELL_SIZE.0 - ((BOARD_CELL_SIZE.0 - (2 * BOARD_DISC_RADIUS)) / 2)) as f32,
            y: (pos.y * BOARD_CELL_SIZE.1 - ((BOARD_CELL_SIZE.1 - (2 * BOARD_DISC_RADIUS)) / 2)) as f32
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
            cells: (0.. BOARD_SIZE.0).map(|y| Cell::new((pos.x, pos.y + (BOARD_CELL_SIZE.0 * y)).into())).collect(),
            height: 0
        }
    }

    // Calls every Cell's draw fn
    fn draw<'a>(&self, mb: &'a mut graphics::MeshBuilder) -> &'a mut graphics::MeshBuilder {
        for y in 0 .. BOARD_SIZE.0 as usize {
            self.cells[y].draw(mb);
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
        for x in 0 .. BOARD_SIZE.1 as usize {
            self.columns[x].draw(mb);
        }
        mb
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
                std::cmp::min(run_len, 3)
            } else {
                std::cmp::min(run_len, 4)
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

}


pub struct GameState {
    frames: usize,
    gameLoaded: GameLoaded,
    /// connect4 board
    pub board: Board,
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
        let mut mb = graphics::MeshBuilder::new();
        // Draw Board
        let mesh = self.board.draw(&mut mb).build(ctx)?;
        graphics::draw(ctx, &mesh, (Point2 {x: 0.0, y: 0.0},))?;
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
