// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.
extern crate ggez;

use connect4::ai::AI;
use connect4::button::Button;
use ggez::input::mouse;
use ggez::input::mouse::MouseButton;
use ggez::mint::Point2;
use ggez::{graphics, Context, GameResult};

/// Constant definition for the connect4 board size: 6x7 cells, row x column.
pub const BOARD_SIZE: (i32, i32) = (6, 7);

/// Constant definition for the pixel size for each square tiles: 32x32 pixels.
const BOARD_CELL_SIZE: (i32, i32) = (64, 64);

/// Constant definition for the radius of each playing disc: 14px.
const BOARD_DISC_RADIUS: i32 = 28;

/// Constant definition for the border size of the board.
const BOARD_BORDER_SIZE: i32 = 32;

/// Constant definition for dimensions of the board
const BOARD_TOTAL_SIZE: (f32, f32) = (
    ((BOARD_SIZE.1 * BOARD_CELL_SIZE.0) + BOARD_BORDER_SIZE) as f32,
    ((BOARD_SIZE.0 * BOARD_CELL_SIZE.0) + BOARD_BORDER_SIZE) as f32,
);

// Testing dynamic Turn Indicator Box size, further decrement by width / 2.
const TURN_INDICATOR_POS_OFFSET: (i32, i32) = (10 + (BOARD_TOTAL_SIZE.0 / 2.0) as i32, 10);

const TURN_INDICATOR_BOX_SIZE_OFFSET: (i32, i32) = (16, 32);

const TURN_INDICATOR_FONT_SIZE: i32 = 48;

const COLUMN_SELECTION_INDICATOR_POS_OFFSET: (i32, i32) = (
    10,
    10 + TURN_INDICATOR_POS_OFFSET.1 + TURN_INDICATOR_BOX_SIZE_OFFSET.1 + TURN_INDICATOR_FONT_SIZE,
);

const BOARD_POS_OFFSET: (i32, i32) = (
    10,
    10 + COLUMN_SELECTION_INDICATOR_POS_OFFSET.1 + BOARD_CELL_SIZE.1,
);

const RESET_BUTTON_OFFSET: (i32, i32) = (10, 10);

/// Constant definition for the screen size of the game window.
pub const SCREEN_SIZE: (f32, f32) = (
    BOARD_TOTAL_SIZE.0 + (BOARD_POS_OFFSET.0 as f32),
    BOARD_TOTAL_SIZE.1 + (BOARD_POS_OFFSET.1 as f32),
);

/// Enums defining some color presets. Call `get_draw_color()` to get the ggez graphics Color object equivalent.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum MyColor {
    White,
    Blue,
    Red,
    Green,
    Brown,
}

impl MyColor {
    ///
    /// Method that produces a graphics::Color struct for drawing purposes
    ///
    pub fn get_draw_color(self) -> ggez::graphics::Color {
        match self {
            MyColor::White => graphics::WHITE,
            MyColor::Blue => graphics::Color::from_rgba(0, 0, 255, 255),
            MyColor::Red => graphics::Color::from_rgba(255, 0, 0, 255),
            MyColor::Green => graphics::Color::from_rgba(0, 255, 0, 255),
            MyColor::Brown => graphics::Color::from_rgba(205, 133, 63, 255),
        }
    }
}

/// Struct representing position on the board
/// Important to note that x is the column value, y is the row value
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GridPosition {
    pub x: i32,
    pub y: i32,
}

impl GridPosition {
    /// Constructor for GridPosition.
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition { x, y }
    }
}

/// From trait converting i32 tuples to GridPosition.
impl From<(i32, i32)> for GridPosition {
    fn from(pos: (i32, i32)) -> Self {
        GridPosition { x: pos.0, y: pos.1 }
    }
}

///
/// A struct a single cell in the board
///
/// # Fields
/// * position = GridPosition struct representing location of the cell on the board
/// * team     = Integer value (0-2) representing the team of the disc in the cell of 0 if the cell is empty
/// * color    = MyColor struct representing color of disc in the cell for drawing purposes. White is empty
///
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Cell {
    position: GridPosition,
    team: i32,
    color: MyColor,
}

impl Cell {
    /// Constructor for Cell, GridPosition is cell's location on the board.
    /// team and color is set as default empty cell values of 0 and MyColor::White.
    pub fn new(pos: GridPosition) -> Self {
        Cell {
            position: pos,
            team: 0,
            color: MyColor::White,
        }
    }

    //Using example from 03_drawing.rs
    /// Create and add the mesh representation of the cell to the MeshBuilder passed in.
    fn draw<'a>(&self, mb: &'a mut graphics::MeshBuilder) -> &'a mut graphics::MeshBuilder {
        let circ_color = self.color.get_draw_color();

        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: self.position.x as f32,
                y: self.position.y as f32,
                w: BOARD_CELL_SIZE.0 as f32,
                h: BOARD_CELL_SIZE.1 as f32,
            },
            graphics::Color::from_rgba(205, 133, 63, 255),
        );
        mb.rectangle(
            graphics::DrawMode::stroke(1.0),
            graphics::Rect {
                x: self.position.x as f32,
                y: self.position.y as f32,
                w: BOARD_CELL_SIZE.0 as f32,
                h: BOARD_CELL_SIZE.1 as f32,
            },
            graphics::BLACK,
        );
        mb.circle(
            graphics::DrawMode::fill(),
            Point2 {
                x: (self.position.x + (BOARD_CELL_SIZE.0 / 2)) as f32,
                y: (self.position.y + (BOARD_CELL_SIZE.1 / 2)) as f32,
            },
            BOARD_DISC_RADIUS as f32,
            2.0,
            circ_color,
        );
        mb
    }

    /// Changes the team and color of the cell.
    fn fill(&mut self, team: i32, color: MyColor) {
        self.team = team;
        self.color = color;
    }
}

///
/// A struct representing a column of cells in the board
///
/// # Fields
/// * position = GridPosition struct representing location of the column in the board
/// * cells    = Vector of cells representing all cells in the column. cells[0] is the where the first disc is dropped           
/// * height   = Integer value re presenting the number/height of filled cells in the column
///
#[derive(Clone, PartialEq, Eq, Debug)]
struct Column {
    position: GridPosition,
    cells: Vec<Cell>,
    height: usize,
}

impl Column {
    ///Constructor for Column
    pub fn new(pos: GridPosition) -> Self {
        Column {
            position: pos,
            // Adapted from: https://stackoverflow.com/questions/48021408/how-to-init-a-rust-vector-with-a-generator-function
            // Rev() method from https://stackoverflow.com/questions/25170091/how-to-make-a-reverse-ordered-for-loop-in-rust; used because columns drawn from top down
            cells: (0..BOARD_SIZE.0)
                .rev()
                .map(|y| Cell::new((pos.x, pos.y + (BOARD_CELL_SIZE.0 * y)).into()))
                .collect(),
            height: 0,
        }
    }

    /// Calls every cell's draw function.
    fn draw<'a>(&self, mb: &'a mut graphics::MeshBuilder) -> &'a mut graphics::MeshBuilder {
        for cell in &self.cells {
            cell.draw(mb);
        }
        mb
    }

    /// Returns height of column.
    pub fn get_height(&self) -> usize {
        self.height
    }

    /// Checks whether the column is full.
    pub fn is_full(&self) -> bool {
        self.height >= BOARD_SIZE.0 as usize
    }

    /// Method to determine if a location (presumed to be the mouse) is inside the column or one cell above (for drop)
    pub fn is_mouse_over(&self, loc: Point2<f32>) -> bool {
        graphics::Rect::new(
            self.position.x as f32,
            (self.position.y - (BOARD_CELL_SIZE.1 * 4 / 3)) as f32,
            BOARD_CELL_SIZE.0 as f32,
            8.0 * BOARD_CELL_SIZE.1 as f32,
        )
        .contains(loc)
    }

    /// Inserts a team's disc of a particular color into a cell
    /// Returns true if disc successfully inserted
    /// Returns false if column is full
    pub fn insert(&mut self, team: i32, color: MyColor) -> bool {
        if self.is_full() {
            false
        } else {
            self.cells[self.height].fill(team, color);
            self.height += 1;
            true
        }
    }

    /// Resets the column.
    /// Changes the cells in the column to their 'empty' state.
    pub fn reset(&mut self) {
        self.height = 0;
        for cell in &mut self.cells {
            cell.fill(0, MyColor::White);
        }
    }
}

///
/// A struct representing the abstraction of the game's Board (connect4).
///
/// # Fields
/// * position = GridPosition struct used to determine the top-left position of the Board in the game window
/// * columns  = Vector of columns representing all columns in the board. cells[0] is the left-most column, cells[5] is the right-most           
///
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Board {
    position: GridPosition,
    columns: Vec<Column>,
}

impl Board {
    ///Constructor for Board
    pub fn new(pos: GridPosition) -> Self {
        Board {
            position: pos,
            columns: (0..BOARD_SIZE.1)
                .map(|x| {
                    Column::new(
                        (
                            pos.x + (BOARD_BORDER_SIZE / 2) + (BOARD_CELL_SIZE.1 * x),
                            pos.y + (BOARD_BORDER_SIZE / 2),
                        )
                            .into(),
                    )
                })
                .collect(),
        }
    }

    /// Builds Board's rect mesh and add it to the `MeshBuilder` passed in and calls column's draw function.
    /// Returns the MeshBuilder (with added board and columns meshes).
    fn draw<'a>(&self, mb: &'a mut graphics::MeshBuilder) -> &'a mut graphics::MeshBuilder {
        mb.rectangle(
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: self.position.x as f32,
                y: self.position.y as f32,
                w: BOARD_TOTAL_SIZE.0 as f32,
                h: BOARD_TOTAL_SIZE.1 as f32,
            },
            graphics::WHITE,
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

    /// Method to get the index of the column that is under the mouse (loc), or -1 if no column is highlighted
    pub fn get_highlighted_column(&self, loc: Point2<f32>) -> i32 {
        for i in 0..self.columns.len() {
            if self.columns[i].is_mouse_over(loc) {
                return i as i32;
            }
        }
        -1
    }

    ///Method to determine if a GridPosition represents a valid location on the Board
    pub fn on_board(&self, pos: GridPosition) -> bool {
        pos.x >= 0 && pos.x < BOARD_SIZE.1 && pos.y >= 0 && pos.y < BOARD_SIZE.0
    }

    ///Method to get the height of a column in the grid
    pub fn get_column_height(&self, col: usize) -> usize {
        self.columns.get(col).unwrap().get_height()
    }

    ///Method to determine if a column in the grid is completely filled
    pub fn is_column_full(&self, col: usize) -> bool {
        self.columns.get(col).unwrap().is_full()
    }

    ///Method to get the team value (1 or 2) from a cell in col[x] and row[y]
    pub fn get_cell_team(&self, pos: GridPosition) -> i32 {
        if self.on_board(pos) {
            self.columns
                .get(pos.x as usize)
                .unwrap()
                .cells
                .get(pos.y as usize)
                .unwrap()
                .team
        } else {
            -1
        }
    }

    ///
    /// Method to get a "max" run including a starting point in a target direction for a given team.
    ///
    /// Accounts for runs towards and away from direction, but allows one space between tiles of the target team in
    /// target direction but no spaces in reverse direction.
    ///
    /// The min value is 1; the max value returned is 4 even if a run is longer. If a space is used, the max returned value is 3
    /// (as the space presumably prevents an actual run of 4). Cases with a run of 4 prior to space will return 4, except for edge
    /// case where run goes from start and then completely in reverse direction. This can be caught by calling this method with reverse
    /// direction
    ///
    /// # Arguments
    /// * start = GridPosition struct representing the starting point to start counting runs from. Assumes that this position in the Board
    ///           is filled and matches the team parameter of this method
    /// * dir   = GridPosition struct used to determine the target direction of the run, where each value is either 0 or 1 to give what is
    ///           rouhgly a unit vector
    /// * team  = Integer value (1 or 2) representing team. Must match value of cell corresponding to start parameter
    ///
    fn get_run_in_direction(&self, start: GridPosition, dir: GridPosition, team: i32) -> i32 {
        let mut dir_active = true;
        let mut rev_active = true;
        let mut dir_spaces_used = 0;
        let mut rev_space_used = false;
        let mut run_len = 1i32; //Start with dropped token
        let mut potential_len = 1; //Assume potential length starts at 1 for dropped token
        let mut i = 1; //Start one beyond dropped token
        while run_len <= 4 && (dir_active || rev_active) {
            dir_active = dir_active
                && self.on_board(GridPosition::new(start.x + i * dir.x, start.y + i * dir.y));
            rev_active = rev_active
                && self.on_board(GridPosition::new(start.x - i * dir.x, start.y - i * dir.y));
            //Do reverse case first for edge case of AASA_A is treated as a run of 4 and not 3 with a space
            if rev_active {
                let val =
                    self.get_cell_team(GridPosition::new(start.x - i * dir.x, start.y - i * dir.y));
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
                let val =
                    self.get_cell_team(GridPosition::new(start.x + i * dir.x, start.y + i * dir.y));
                //If token not for team in cell, end of search in target direction
                if val != 0 && val != team {
                    dir_active = false;
                //If 0 or 1 spaces in target direction used, either add to run_len and/or potential_run depending on if cell is empty or matches team
                } else if dir_spaces_used <= 1 {
                    //If you have a contiguous run of 4 with no spaces, immediately return because a winning run has been found!
                    if run_len >= 4 && dir_spaces_used == 0 {
                        return 4i32;
                    } else if val == team {
                        run_len += 1;
                    } else {
                        dir_spaces_used += 1;
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
            0i32
        //Otherwise, return the minimum of the run_len and 4 (if no spaces) or 3 (if one space used)
        } else if dir_spaces_used > 0 {
            run_len.min(3)
        } else {
            run_len.min(4)
        }
    }

    ///
    /// Method to return an array of runs from a start location for a given team, where array[i] returns the number of runs
    /// of length i-1. Accounts for all eight directions, but may have false duplicates (e.g. a run 21112 will return have two
    /// runs of length 3 for team 1 even though technically its the same run)
    ///
    /// # Arguments
    /// * start = GridPosition struct representing the starting point to start counting runs from. Assumes that this position in the Board
    ///           is filled and matches the team parameter of this method
    /// * team  = Integer value (1 or 2) representing team. Must match value of cell corresponding to start parameter
    ///
    pub fn get_runs_from_point(&self, start: GridPosition, team: i32) -> [i32; 4] {
        let mut output = [0i32; 4];
        let directions = vec![(1, 0), (1, 1), (0, 1), (-1, 1)];
        for dir in directions {
            let a = self.get_run_in_direction(start, GridPosition::new(dir.0, dir.1), team) - 1;
            let b = self.get_run_in_direction(start, GridPosition::new(-dir.0, -dir.1), team) - 1;
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

    /// Calls the reset function of every columns in the Board.
    pub fn reset(&mut self) {
        for column in &mut self.columns {
            column.reset();
        }
    }
}

///
/// A struct for the object that displays whose turn is it currently and the gameover win/ draw message
///
/// # Fields
/// * gaemover = Boolean indicating that game is over
/// * team     = Value from 0-2 indicating the team whose turn it is or 0 if the game is paused or completed           
///
pub struct TurnIndicator {
    gameover: bool,
    team: i32,
}

impl TurnIndicator {
    ///Constructor
    pub fn new() -> Self {
        TurnIndicator {
            gameover: false,
            team: 0,
        }
    }

    /// Draws the turn indicator onto the Context/ game window.
    /// Text displayed depends on the state of the `gameover` and `team` property
    /// team: 0 & gameover: false = Game Draw
    /// team: 1 or 2 & gameover: true = Player 1 or 2 Wins!
    /// team: 1 or 2 & gameover: false = Player 1 or 2's turn
    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let text: graphics::Text;
        if self.gameover {
            if self.team == 0 {
                text = graphics::Text::new((
                    "Game Draw!",
                    graphics::Font::default(),
                    TURN_INDICATOR_FONT_SIZE as f32,
                ));
            } else {
                text = graphics::Text::new((
                    format!("Player {} wins!", self.team),
                    graphics::Font::default(),
                    TURN_INDICATOR_FONT_SIZE as f32,
                ));
            }
        } else if self.team == 0 {
            text = graphics::Text::new((
                "Paused",
                graphics::Font::default(),
                TURN_INDICATOR_FONT_SIZE as f32,
            ));
        } else {
            text = graphics::Text::new((
                format!("Player {}'s turn", self.team),
                graphics::Font::default(),
                TURN_INDICATOR_FONT_SIZE as f32,
            ));
        }

        let dim = &text.dimensions(ctx);
        let pos = Point2 {
            x: TURN_INDICATOR_POS_OFFSET.0 as f32 - (dim.0 as f32 / 2.0) as f32,
            y: TURN_INDICATOR_POS_OFFSET.1 as f32,
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
            graphics::Color::from_rgba(205, 133, 63, 255),
        )?;
        graphics::draw(ctx, &textbox, (Point2 { x: 0.0, y: 0.0 },))?;
        graphics::draw(
            ctx,
            &text,
            (Point2 {
                x: pos.x + TURN_INDICATOR_BOX_SIZE_OFFSET.0 as f32 / 2.0,
                y: pos.y + TURN_INDICATOR_BOX_SIZE_OFFSET.1 as f32 / 2.0,
            },),
        )?;
        Ok(())
    }

    /// Change the value of the team property of the turn indicator object.
    pub fn change_team(&mut self, team: i32) {
        self.team = team;
    }

    /// Change the value of the gameover property of the turn indicator object.
    pub fn game_ends(&mut self) {
        self.gameover = true;
    }

    /// Resets the values of the team and gameover property of the turn indicator object to 0 and false.
    pub fn reset(&mut self) {
        self.team = 0;
        self.gameover = false;
    }
}

///
/// A struct that contains the states for the connect 4 game
///
/// # Fields
/// * frames             = Integer counter for the number of times the update method is called; helps gauge time
/// * ai_players         = Vector of AI structs representing any AI players in the game
/// * board              = Board struct representing current board state           
/// * team_colors        = Vector of MyColor objects representing what color to draw discs for player i or the empty cell (for 0 index)           
/// * turn_indicator     = TurnIndicator object tracking turns         
/// * highlighted_column = Integer from -1 to 6 representing column over which a disc is hovering (-1 means no column is being hovered)           
/// * mouse_disabled     = Boolean indicating if clicking is enabled       
/// * gameover           = Boolean indicating if game is over  
/// * reset_button       = Button drawn to allow board to be reset and game to be restarted           
/// * main_menu_button   = Button drawn to allow return to main menu screen          
///
pub struct GameState {
    frames: usize,
    ai_players: Vec<AI>,
    pub board: Board,
    team_colors: Vec<MyColor>,
    pub turn_indicator: TurnIndicator,
    pub highlighted_column: i32,
    mouse_disabled: bool,
    gameover: bool,
    pub reset_button: Button,
    pub main_menu_button: Button,
}

//Implementation based on structure in example from GGEZ repo (see https://github.com/ggez/ggez/blob/master/examples/02_hello_world.rs)
impl GameState {
    ///Constructor - players is the number of human players to be in the game
    pub fn new(ctx: &mut Context, players: i32) -> GameState {
        let board_pos = BOARD_POS_OFFSET;
        let main_menu_btn_text =
            graphics::Text::new(("Main Menu", graphics::Font::default(), 16f32));
        let main_menu_text_width = main_menu_btn_text.width(ctx) as f32;
        let main_menu_text_height = main_menu_btn_text.height(ctx) as f32;
        let main_menu_btn_outline = graphics::Rect::new(
            RESET_BUTTON_OFFSET.0 as f32,
            RESET_BUTTON_OFFSET.1 as f32 + main_menu_text_height,
            main_menu_text_width,
            main_menu_text_height,
        );
        let mut main_menu_btn = Button::new(main_menu_btn_text, main_menu_btn_outline);

        let reset_text = graphics::Text::new(("Reset", graphics::Font::default(), 16f32));
        let reset_outline = graphics::Rect::new(
            RESET_BUTTON_OFFSET.0 as f32,
            RESET_BUTTON_OFFSET.1 as f32 + main_menu_text_height * 3.0,
            main_menu_text_width,
            main_menu_text_height,
        );
        let mut reset_btn = Button::new(reset_text, reset_outline);

        reset_btn.set_colors(MyColor::Brown, MyColor::Red);
        main_menu_btn.set_colors(MyColor::Brown, MyColor::Green);
        let mut bots = Vec::<AI>::new();
        for i in 0..players {
            bots.push(AI::new(2 - i, 3));
        }
        GameState {
            frames: 0,
            ai_players: bots,
            board: Board::new(board_pos.into()),
            team_colors: vec![MyColor::White, MyColor::Red, MyColor::Blue],
            turn_indicator: TurnIndicator::new(),
            highlighted_column: -1,
            mouse_disabled: false,
            gameover: false,
            reset_button: reset_btn,
            main_menu_button: main_menu_btn,
        }
    }

    /// Update method - contains main game logic.
    pub fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.frames += 1; //Timing mechanism for bot moves
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
                self.turn_indicator.change_team(0);
                self.turn_indicator.game_ends();
            }
            //Check for AI actions
            let mut bot_active = false;
            for ai in &mut self.ai_players {
                if ai.team == self.turn_indicator.team {
                    bot_active = true;
                    self.mouse_disabled = true;
                    //Check if move selection process has started
                    if ai.last_move_frame < 0 {
                        self.highlighted_column = ai.pick_optimal_move(self.board.clone());
                        ai.last_move_frame = self.frames as i32;
                    //If enough frames have passed, make move
                    } else if self.frames > (ai.last_move_frame + 100) as usize {
                        if self.board.insert(
                            self.highlighted_column,
                            self.turn_indicator.team,
                            self.team_colors[self.turn_indicator.team as usize],
                        ) {
                            println!(
                                "AI Player {} drops token in col {}",
                                ai.team, self.highlighted_column
                            );

                            //game state check
                            let runs = self.board.get_runs_from_point(
                                GridPosition::new(
                                    self.highlighted_column,
                                    self.board
                                        .get_column_height(self.highlighted_column as usize)
                                        as i32
                                        - 1,
                                ),
                                ai.team,
                            );
                            if runs[3] > 0 {
                                //Four Connected - Proceed to Gameover - Win/Loss state
                                println!(
                                    "4 Connected for player {}; Game ends",
                                    self.turn_indicator.team
                                );
                                self.gameover = true;
                                self.turn_indicator.game_ends();
                            } else {
                                self.turn_indicator.team = self.turn_indicator.team % 2 + 1; //Change to other team's turn
                            }
                        }
                        //Reset check for a move so next move can be made
                        ai.last_move_frame = -1;
                    }
                }
            }
            self.mouse_disabled = bot_active;
        }
        Ok(())
    }

    ///Draw method to render the board, turn indicator, and other buttons
    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        //Draw screen background
        graphics::clear(ctx, graphics::BLACK);
        let mut mb = graphics::MeshBuilder::new();
        //Draw disc over current column
        if self.highlighted_column >= 0 {
            mb.circle(
                graphics::DrawMode::fill(),
                Point2 {
                    x: (self.board.columns[self.highlighted_column as usize]
                        .position
                        .x
                        + (BOARD_CELL_SIZE.0 / 2)) as f32,
                    y: (self.board.position.y - (BOARD_CELL_SIZE.1 / 2)) as f32,
                },
                BOARD_DISC_RADIUS as f32,
                2.0,
                self.team_colors[self.turn_indicator.team as usize].get_draw_color(),
            );
        }
        //Draw Board
        let mesh = self.board.draw(&mut mb).build(ctx)?;
        graphics::draw(ctx, &mesh, (Point2 { x: 0.0, y: 0.0 },))?;

        //Draw turn indicator
        self.turn_indicator.draw(ctx)?;

        //Draw reset button
        self.reset_button.draw(ctx)?;
        self.main_menu_button.draw(ctx)?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    ///Method active whenever the mouse is moved (if mouse is not intentionally disabled). Changes the highlighted_column
    ///value based on mouse location
    pub fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {
        if !self.mouse_disabled {
            let was_highlighted = self.highlighted_column;
            self.highlighted_column = self.board.get_highlighted_column(mouse::position(_ctx));
            //Log ONLY switches between columns (otherwise lot of logs to console)
            if was_highlighted != self.highlighted_column {
                println!("Mouse moved to col {}", self.highlighted_column);
            }
        }
        self.reset_button.check_button_under_mouse(_ctx);
        self.main_menu_button.check_button_under_mouse(_ctx);
    }

    ///Method active whenever the mouse is pressed down (if mouse is not intentionally disabled). Changes the highlighted_column
    ///value based on mouse location, combined with mouse_button_up_event to form a click
    pub fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        if !self.mouse_disabled {
            self.highlighted_column = self.board.get_highlighted_column(mouse::position(_ctx));
        }
        self.reset_button.check_button_under_mouse(_ctx);
        self.main_menu_button.check_button_under_mouse(_ctx);
    }

    ///Method active whenever thea pressed mouse button is released (if mouse is not intentionally disabled). Changes the highlighted_column
    ///value based on mouse location, combined with mouse_button_up_event to form a click
    pub fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) -> bool {
        if !self.mouse_disabled {
            let was_highlighted = self.highlighted_column;
            self.highlighted_column = self.board.get_highlighted_column(mouse::position(_ctx));
            //TODO: Originally intended to only click if column highlihgted on button down matches highlighted column on mouse up. However,
            //mouse move check automatically updates state, so this will always click. TBD if change will be made to address this
            if was_highlighted == self.highlighted_column && self.highlighted_column >= 0 {
                self.mouse_disabled = true;
                if self.board.insert(
                    self.highlighted_column,
                    self.turn_indicator.team,
                    self.team_colors[self.turn_indicator.team as usize],
                ) {
                    println!(
                        "Team {} drops token in col {}",
                        self.turn_indicator.team, self.highlighted_column
                    );
                    //game state check
                    let runs = self.board.get_runs_from_point(
                        GridPosition::new(
                            self.highlighted_column,
                            self.board
                                .get_column_height(self.highlighted_column as usize)
                                as i32
                                - 1,
                        ),
                        self.turn_indicator.team,
                    );
                    if runs[3] > 0 {
                        //Four Connected - Proceed to Gameover - Win/Loss state
                        println!(
                            "4 Connected for player {}; Game ends",
                            self.turn_indicator.team
                        );
                        self.gameover = true;
                        self.turn_indicator.game_ends();
                    } else {
                        self.turn_indicator.team = self.turn_indicator.team % 2 + 1; //Change to other team's turn
                    }
                }
                if !self.gameover {
                    self.mouse_disabled = false;
                }
            }
        }
        //Check reset button
        if self.reset_button.check_button_under_mouse(_ctx) {
            println!("Reset button pressed; Board reset");
            self.board.reset();
            self.turn_indicator.reset();
            self.turn_indicator.change_team(1);
            self.gameover = false;
            self.mouse_disabled = false;
        }
        //Check main menu button
        if self.main_menu_button.check_button_under_mouse(_ctx) {
            println!("Main Menu Button pressed; Main Menu should pop up");
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod core_tests {
    use super::*;
    //Method to create a board state from a set of vectors, where 0 is empty and 1 or 2 team tokens
    //Note that input is board[column][row], so if you want to add a team 1 token in column 4, row 0, then
    //the board input should have board[4][0] = 1
    fn create_test_board(board: Vec<Vec<i32>>) -> Board {
        let mut output = Board::new(GridPosition { x: 0, y: 0 });
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

    mod board {
        use super::*;

        mod on_board {
            use super::*;

            #[test]
            fn should_handle_edge_cases() {
                let board = create_test_board(vec![vec![]]);
                assert_eq!(board.on_board(GridPosition::new(0, 0)), true);
                assert_eq!(board.on_board(GridPosition::new(-1, 0)), false);
                assert_eq!(board.on_board(GridPosition::new(0, -1)), false);
                assert_eq!(board.on_board(GridPosition::new(6, 5)), true);
                assert_eq!(board.on_board(GridPosition::new(7, 5)), false);
                assert_eq!(board.on_board(GridPosition::new(6, 6)), false);
            }
        }

        mod get_cell_team {
            use super::*;

            #[test]
            fn should_return_error_val_if_not_on_board() {
                let data = vec![
                    vec![1, 1, 0, 1, 2, 0],
                    vec![1, 2, 2, 1, 2, 2],
                    vec![2, 1, 1, 2, 1, 0],
                    vec![2, 1, 1, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0],
                    vec![1, 1, 2, 1, 2, 1],
                    vec![1, 1, 2, 2, 2, 0],
                ];
                let board = create_test_board(data);
                assert_eq!(board.get_cell_team(GridPosition::new(-1, 0)), -1);
                assert_eq!(board.get_cell_team(GridPosition::new(0, 6)), -1);
            }

            #[test]
            fn should_handle_get_team_values() {
                let data = vec![
                    vec![1, 1, 1, 1, 2, 0],
                    vec![1, 2, 2, 1, 2, 2],
                    vec![2, 1, 1, 2, 1, 0],
                    vec![2, 1, 1, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0],
                    vec![1, 1, 2, 1, 2, 1],
                    vec![1, 1, 2, 2, 2, 0],
                ];
                let board = create_test_board(data);
                assert_eq!(board.get_cell_team(GridPosition::new(0, 0)), 1);
                assert_eq!(board.get_cell_team(GridPosition::new(2, 0)), 2);
                assert_eq!(board.get_cell_team(GridPosition::new(0, 5)), 0);
                assert_eq!(board.get_cell_team(GridPosition::new(5, 5)), 1);
                assert_eq!(board.get_cell_team(GridPosition::new(1, 3)), 1);
                assert_eq!(board.get_cell_team(GridPosition::new(4, 0)), 0);
            }
        }

        mod get_run_in_direction {
            use super::*;

            #[test]
            fn should_find_contiguous_run() {
                let data = vec![
                    vec![0],
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![0],
                ];
                let board = create_test_board(data);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    3
                );
            }

            #[test]
            fn should_find_not_be_more_than_4() {
                let data = vec![
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![1],
                ];
                let board = create_test_board(data);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    4
                );
            }

            #[test]
            fn should_find_run_with_space() {
                let data = vec![
                    vec![0],
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![0],
                    vec![1],
                    vec![1],
                ];
                let board = create_test_board(data);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    3
                );
            }

            #[test]
            fn should_find_not_be_more_than_3_with_space() {
                let data = vec![
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![0],
                    vec![1],
                ];
                let board = create_test_board(data);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    3
                );
            }

            #[test]
            fn should_not_count_two_spaces() {
                let data = vec![
                    vec![0],
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![0],
                    vec![0],
                    vec![1],
                ];
                let board = create_test_board(data);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    1
                );
            }

            #[test]
            fn should_not_count_past_space_in_rev_direction() {
                let data = vec![
                    vec![1],
                    vec![0],
                    vec![1],
                    vec![1],
                    vec![0],
                    vec![0],
                    vec![0],
                ];
                let board = create_test_board(data);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    2
                );
            }

            #[test]
            fn should_return_run_of_4_prior_to_space() {
                //Should return 4
                let run1 = vec![
                    vec![0],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![0],
                    vec![1],
                ];
                let board = create_test_board(run1);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    4
                );
                //This should return 4 - handled by rev direction case
                let run2 = vec![
                    vec![0],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![0],
                    vec![1],
                ];
                let board = create_test_board(run2);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(2, 0), GridPosition::new(1, 0), 1),
                    4
                );
                //This should not return 4 - handled by rev direction case
                let run3 = vec![
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![0],
                    vec![1],
                    vec![0],
                ];
                let board = create_test_board(run3);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    3
                );
            }

            #[test]
            fn returns_0_if_run_of_4_impossible() {
                //Runs 1-3 should return 0, Runs 4-5 should pass due to potential in either direction
                let run1 = vec![
                    vec![0],
                    vec![2],
                    vec![1],
                    vec![1],
                    vec![1],
                    vec![2],
                    vec![1],
                ];
                let mut board = create_test_board(run1);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    0
                );
                let run2 = vec![
                    vec![2],
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![2],
                    vec![0],
                    vec![1],
                ];
                board = create_test_board(run2);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    0
                );
                let run3 = vec![
                    vec![0],
                    vec![1],
                    vec![1], //Checking here
                    vec![2],
                    vec![0],
                    vec![0],
                    vec![0],
                ];
                board = create_test_board(run3);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(2, 0), GridPosition::new(1, 0), 1),
                    0
                );
                let run4 = vec![
                    vec![1],
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![2],
                    vec![0],
                    vec![1],
                ];
                board = create_test_board(run4);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    1
                );
                let run5 = vec![
                    vec![2],
                    vec![0],
                    vec![0],
                    vec![1],
                    vec![0],
                    vec![2],
                    vec![1],
                ];
                board = create_test_board(run5);
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 0), GridPosition::new(1, 0), 1),
                    1
                );
            }

            #[test]
            fn should_work_in_all_directions() {
                //Note - the runs may not match in opposite directions!
                let data = vec![
                    vec![1, 1, 0, 1, 2, 0],
                    vec![1, 2, 2, 1, 2, 2],
                    vec![2, 1, 1, 2, 1, 0],
                    vec![2, 1, 1, 1, 0, 0], //Target is in middle of this column
                    vec![0, 0, 0, 0, 0, 0],
                    vec![1, 1, 2, 1, 2, 1],
                    vec![1, 1, 2, 2, 2, 0],
                ];
                let board = create_test_board(data);
                //Vertical directions - should be 3 in both cases
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(0, 1), 1),
                    3
                );
                assert_eq!(
                    board.get_run_in_direction(
                        GridPosition::new(3, 3),
                        GridPosition::new(0, -1),
                        1
                    ),
                    3
                );
                //Horizontal directions - should be 0 in both cases since two 2s block potential run of 4
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(1, 0), 1),
                    0
                );
                assert_eq!(
                    board.get_run_in_direction(
                        GridPosition::new(3, 3),
                        GridPosition::new(-1, 0),
                        1
                    ),
                    0
                );
                //Bottom-left to upper-right diagonal directions - should be 2 going down and 3 going up (since space then token in upper-right dir)
                assert_eq!(
                    board.get_run_in_direction(GridPosition::new(3, 3), GridPosition::new(1, 1), 1),
                    3
                );
                assert_eq!(
                    board.get_run_in_direction(
                        GridPosition::new(3, 3),
                        GridPosition::new(-1, -1),
                        1
                    ),
                    2
                );
                //Bottom-right to upper-left diagonal directions - should be 2 going up and 3 going down (since space in down dir, but blocked going up)
                assert_eq!(
                    board.get_run_in_direction(
                        GridPosition::new(3, 3),
                        GridPosition::new(-1, 1),
                        1
                    ),
                    2
                );
                assert_eq!(
                    board.get_run_in_direction(
                        GridPosition::new(3, 3),
                        GridPosition::new(1, -1),
                        1
                    ),
                    3
                );
            }
        }

        mod get_runs_from_point {
            use super::*;

            #[test]
            fn should_find_runs() {
                let data = vec![
                    vec![1, 1, 0, 1, 2, 0],
                    vec![1, 2, 2, 1, 2, 2],
                    vec![2, 1, 1, 2, 1, 0],
                    vec![2, 1, 1, 1, 0, 0], //Target is in middle of this column
                    vec![0, 0, 0, 0, 0, 0],
                    vec![1, 1, 2, 1, 2, 1],
                    vec![1, 1, 2, 2, 2, 0],
                ];
                let board = create_test_board(data);
                assert_eq!(
                    board.get_runs_from_point(GridPosition::new(3, 3), 1),
                    [0, 2, 4, 0]
                );
            }
        }
    }
}
