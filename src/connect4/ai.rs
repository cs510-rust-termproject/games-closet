// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use connect4::core::{Board, GridPosition, MyColor, BOARD_SIZE};
use std::cmp::Ordering;

///
/// A struct representing a potential future move on a given board. Utilized by the AI struct to
/// determine future moves and win probabilities
///
/// # Fields
/// * team  = Integer value (1-2) representing team that is making the move
/// * board = Board object representing grid state after a move is made
/// * run   = Array of runs for the given team from the location of the move for this object. runs[0] is # of runs
///              of length 1, runs[1] is # of runs of length 2, etc. Runs are often duplicates (i.e. a contiguous run of
///              3 in the vertical direction is counted as both a run of 3 in the up and down direction)
///
pub struct MoveCheck {
    team: i32,
    board: Board,
    runs: [i32; 4],
}

impl MoveCheck {
    ///
    /// Method to initialize and return a MoveCheck object
    ///
    /// # Arguments
    /// * board    = Board struct representing the state of the board prior to the move being made
    /// * move_col = Index of column the disc is dropped in to make the move
    /// * team     = Integer value represent the team number of the disc being placed for the move
    ///
    fn new(board: Board, move_col: i32, team: i32) -> Self {
        let mut new_board = board.clone();
        let runs = new_board.get_runs_from_point(
            GridPosition::new(
                move_col,
                new_board.get_column_height(move_col as usize) as i32,
            ),
            team,
        );
        new_board.insert(move_col, team, MyColor::White);
        MoveCheck {
            team,
            board: new_board,
            runs,
        }
    }

    ///
    /// Method returning a boolean indicating if the move produces a run of 4 for the given team
    ///
    fn has_end_result(&self) -> bool {
        self.runs[3] > 0
    }

    ///
    /// Method to return the win probability for a given team based on the current move. Does weighted probability
    /// calculation using the number of runs of each length and produces a value between 1.0 and 0.0.
    ///
    /// # Arguments
    /// * team = Integer value (1 or 2) of team for which to calculate win probaility
    ///
    fn get_win_probability(&self, team: i32) -> f32 {
        let mut prob = 0f32;
        for i in 0..self.runs.len() {
            //Formula is 1/2^(i-3) * runs[i], so each run[3] has a prob of 1 (since it corresponds to a run of 3),
            //each run[2] has a prob of 0.5, etc. All of this is divided by 2 since runs duplicate in opposite directionss
            prob += (2.0f32.powi((i as i32) - 3) * (self.runs[i] as f32)) / 2.0;
        }
        //If teams match, return probability (but don't go over prob of 1.0)
        if team == self.team {
            prob.min(1.0)
        //If teams don't match, return 1-probability (but don't go below prob of 0.0)
        } else {
            (1.0 - prob).max(0.0)
        }
    }
}

//Ordering implementation based on documentation example (https://doc.rust-lang.org/std/cmp/trait.Ord.html), tailored to compare MoveCheck's runs
impl Ord for MoveCheck {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..4).rev() {
            if self.runs[i] != other.runs[i] {
                if (i as i32) * (self.runs[i] - other.runs[i]) < 0 {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for MoveCheck {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MoveCheck {
    fn eq(&self, other: &Self) -> bool {
        self.runs == other.runs
    }
}

impl Eq for MoveCheck {}

///
/// A struct representing an AI or bot player for Connect4 which has methods to determine "ideal" moves
///
/// # Fields
/// * team            = Integer value (1-2) representing team that is making the move
/// * difficulty      = Integer value that determines how "smart" the AI is (i.e. how deep the recursive search for a move will go)
/// * last_move_frame = Integer used to track when a determination of an ideal move has last been made. Should be set to -1 until move
///                     is determined for a round, then reset once move is drawn and board state changes
///
pub struct AI {
    pub team: i32,
    difficulty: i32,
    pub last_move_frame: i32,
}

impl AI {
    ///
    /// Method to initialize and return an AI object
    ///
    /// # Arguments
    /// * team     = Integer value represent the team number of the disc being placed for the move
    /// * difficulty      = Integer value that determines how "smart" the AI is (i.e. how deep the recursive search for a move will go)
    ///
    pub fn new(team: i32, difficulty: i32) -> Self {
        AI {
            team,
            difficulty,
            last_move_frame: -1,
        }
    }

    ///
    /// Method to determine the "optimal" move based on aboard state. Returns an integer value represnting the column to place
    /// the next disc
    ///
    /// # Arguments
    /// * board    = Board struct representing the current state of the board
    ///
    pub fn pick_optimal_move(&self, board: Board) -> i32 {
        let mut best_move = -1;
        let mut best_prob = 0f32;
        for i in 0..BOARD_SIZE.1 {
            //For each valid move, create a MoveCheck to evaluate immediate move options
            if !board.is_column_full(i as usize) {
                let next_move = MoveCheck::new(board.clone(), i, self.team);
                //If move will win game, make move
                if next_move.has_end_result() {
                    return i;
                } else {
                    //Otherwise, find win probability after move has been made to see if it is better than other possible moves
                    let curr_prob = self.find_win_probability(next_move.board, 1, self.difficulty);
                    if curr_prob == 1f32 {
                        return i;
                    } else if curr_prob >= best_prob {
                        best_prob = curr_prob;
                        best_move = i;
                    }
                }
            }
        }
        best_move
    }

    ///
    /// Method to recursively find the win probability for a given board state. Returns a value between 1.0 and 0.0
    ///
    /// # Arguments
    /// * board    = Board struct representing the current state of the board (this will be updated each successive recursive call)
    /// * curr_move = How deep into the recursion we are. Also worth noting that if curr_move % 2 == 0, then this is a move made by the
    ///               the AI, otherwise it is a move made by the opponent
    /// * last_move = Integer indicating depth at which to stop recursion and make a best guess of prob based on board state
    ///
    fn find_win_probability(&self, board: Board, curr_move: i32, last_move: i32) -> f32 {
        let mut moves = Vec::new();
        for i in 0..BOARD_SIZE.1 {
            if !board.is_column_full(i as usize) {
                let board = board.clone();
                //This will always make a MoveCheck where the "team" is self.team if curr_move%2 == 0 and the opposite team if curr_move%2 == 1
                //Assumes only two teams, 1 and 2
                let move_check = MoveCheck::new(board, i, (self.team + curr_move + 1) % 2 + 1);
                //If move produces end result, return an absolute probability of 1 (if current move_check is for team) or 0 (for opp)
                if move_check.has_end_result() {
                    return (1 - (self.team - move_check.team).abs() % 2) as f32;
                //If curr_move is not last move, recurse on subsequent moves from the current move_check and add result to a list
                } else if curr_move < last_move {
                    moves.push(self.find_win_probability(
                        move_check.board,
                        curr_move + 1,
                        last_move,
                    ));
                //Base case - this is the last move, so just add the current probability of win for this move_check to the list
                } else {
                    moves.push(move_check.get_win_probability(self.team));
                }
            }
        }
        //Edge case - no moves to make, return 0 probability (can't win)
        if moves.is_empty() {
            0f32
        //If the list of evaluated moves has a guaranteed outcome of 1.0 or 0.0, then return that value because that move will certainly be made
        } else if moves.contains(&((curr_move % 2) as f32)) {
            (curr_move % 2) as f32
        //Otherwise, return average of all possibilities
        //TODO: Consider picking "best" move instead?
        } else {
            moves.iter().sum::<f32>() / (moves.len() as f32)
        }
    }
}

#[cfg(test)]
mod ai_tests {
    use super::*;
    use connect4::core::Board;

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

    mod move_check {
        use super::*;
        use connect4::ai::MoveCheck;

        mod get_win_probability {
            use super::*;

            #[test]
            fn should_return_inverse_probs_for_diff_teams() {
                let data = vec![vec![1, 1, 1, 1, 1, 0]];
                let check = MoveCheck::new(create_test_board(data), 0, 1);
                assert_eq!(check.get_win_probability(1), 1.0);
                assert_eq!(check.get_win_probability(2), 0.0);
            }

            #[test]
            fn should_return_value_between_0_and_1() {
                let data = vec![
                    vec![1, 1, 1, 1, 1, 0],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                ];
                let check = MoveCheck::new(create_test_board(data), 0, 1);
                assert_eq!(check.get_win_probability(1), 1.0);
                assert_eq!(check.get_win_probability(2), 0.0);
            }
        }
    }

    mod ai {
        use super::*;
        use connect4::ai::AI;

        mod pick_optimal_move {
            use super::*;

            #[test]
            fn should_return_first_winning_move() {
                let data = vec![
                    vec![0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0],
                    vec![1, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0],
                    vec![0, 0, 0, 0, 0, 0],
                ];
                let mut board = create_test_board(data);
                let test_ai = AI::new(1, 1);
                //Should prioritize col 1 over col 5 even though both win
                assert_eq!(test_ai.pick_optimal_move(board.clone()), 1i32);
                //Insert enemy token to block col 1, now col 5 should be found
                board.insert(1, 2, MyColor::White);
                assert_eq!(test_ai.pick_optimal_move(board.clone()), 5i32);
            }
        }

        mod find_win_probability {
            use super::*;

            #[test]
            fn should_default_to_0_if_board_full() {
                let data = vec![
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                    vec![1, 1, 1, 1, 1, 1],
                ];
                let board = create_test_board(data);
                let test_ai = AI::new(1, 1);
                assert_eq!(test_ai.find_win_probability(board.clone(), 0, 3), 0.0);
                assert_eq!(test_ai.find_win_probability(board.clone(), 1, 3), 0.0);
            }

            #[test]
            fn should_return_win_or_loss_with_4_run() {
                let data = vec![
                    vec![1, 1, 2, 1, 2, 0],
                    vec![1, 2, 2, 1, 2, 2],
                    vec![2, 2, 2, 0, 0, 0],
                    vec![1, 1, 1, 1, 0, 0],
                    vec![0, 0, 0, 0, 0, 0],
                    vec![1, 1, 2, 1, 2, 1],
                    vec![1, 1, 2, 2, 2, 0],
                ];
                let board = create_test_board(data);
                let test_ai = AI::new(1, 1);
                assert_eq!(test_ai.find_win_probability(board.clone(), 0, 3), 1.0);
                assert_eq!(test_ai.find_win_probability(board.clone(), 1, 3), 0.0);
            }

            #[test]
            fn should_return_average_of_options() {
                let data = vec![
                    vec![2, 2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2, 2],
                    vec![2, 2, 2, 2, 2, 0], //Prob of 1/8 here (one run of length 1)
                    vec![2, 2, 1, 0, 0, 0], //Prob of 1/4 here (one run of length 2)
                    vec![2, 2, 2, 2, 2, 0], //Prob of 1/8 here (one run of length 1)
                    vec![1, 1, 0, 0, 0, 0],
                ]; //Prob of 1/2 (here (one run of length 3))
                   //Average prob is thus 1/4
                let board = create_test_board(data);
                let test_ai = AI::new(1, 1);
                assert_eq!(test_ai.find_win_probability(board.clone(), 0, 0), 0.25);
            }
        }
    }
}
