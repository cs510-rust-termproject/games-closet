// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use connect4::core::{GridPosition, Board, GameState, BOARD_SIZE, MyColor};
use std::cmp::Ordering;

pub struct MoveCheck {
    team: i32,
    board: Board,
    runs: [i32;4]
}

impl MoveCheck {
    fn new(board: Board, moveCol: i32, team: i32) -> Self {
        let mut newBoard = board.clone();
        let runs = newBoard.get_runs_from_point(GridPosition::new(moveCol, newBoard.get_column_height(moveCol as usize) as i32), team);
        newBoard.insert(moveCol, team, MyColor::White);
        MoveCheck { team: team, board: newBoard, runs: runs }
    }

    fn has_end_result(&self) -> bool {
        self.runs[3] > 0
    }


    fn get_win_probability(&self, team: i32) -> f32 {
        let mut prob = 0f32;
        for i in 0..self.runs.len() {
            //Formula is 1/2^(i-3) * runs[i], so each run[3] has a prob of 1 (since it corresponds to a run of 3),
            //each run[2] has a prob of 0.5, etc. All of this is divided by 2 since runs duplicate in opposite directionss
            prob += (2.0f32.powi((i as i32)-3)*(self.runs[i] as f32))/2.0;
        }
        //If teams match, return probability (but don't go over prob of 1.0)
        if team == self.team {
            prob.min(1.0)
        //If teams don't match, return 1-probability (but don't go below prob of 0.0)
        } else {
            (1.0-prob).max(0.0)
        }
    }
}

//Ordering implementation based on documentation example (https://doc.rust-lang.org/std/cmp/trait.Ord.html), tailored to compare MoveCheck's runs
impl Ord for MoveCheck {
    fn cmp(&self, other: &Self) -> Ordering {
        for i in (0..4).rev() {
            if self.runs[i] != other.runs[i] {
                if (i as i32)*(self.runs[i]-other.runs[i]) < 0 {
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

pub struct AI {
    pub team: i32,
    difficulty: i32,
    pub last_move_frame: i32
}

impl AI {
    pub fn new(team: i32, difficulty: i32) -> Self {
        AI { team: team, difficulty: difficulty, last_move_frame: -1 }
    }

    pub fn pick_optimal_move(&self, board: Board) -> i32 {
        let mut bestMove = -1;
        let mut bestProb = 0.0;
        for i in 0..BOARD_SIZE.1 {
            let mut currBoard = board.clone();
            currBoard.insert(i, self.team, MyColor::White);
            let currProb = self.find_win_probability(currBoard, 0, self.difficulty);
            if currProb == 1.0 {
                return i as i32;
            } else if currProb >= bestProb {
                bestProb = currProb;
                bestMove = i;
            }
        }
        bestMove
    }

    fn find_win_probability(&self, board: Board, currMove: i32, lastMove: i32) -> f32 {
        //Check win for AI turn
        let mut moves = Vec::new();
        //let mut out = "".to_owned();
        for i in 0..BOARD_SIZE.1 {
            if !board.is_column_full(i as usize) {
                //out += "non-full column\n";
                let board = board.clone();
                //This will always make a MoveCheck where the "team" is self.team if currMove%2 == 0 and the opposite team if currMove%2 == 1
                //Assumes only two teams, 1 and 2
                let moveCheck = MoveCheck::new(board, i, (self.team+currMove+1)%2+1);
                //If move produces end result, return an absolute probability of 1 (if current moveCheck is for team) or 0 (for opp)
                if moveCheck.has_end_result() {
                    return (1-(self.team-moveCheck.team).abs()%2) as f32;
                //If currMove is not last move, recurse on subsequent moves from the current moveCheck
                } else if currMove < lastMove {
                    moves.push(self.find_win_probability(moveCheck.board, currMove+1, lastMove));
                    //out += "pushed move\n";
                //Base case - this is the last move, so just return current probability of win for this moveCheck relative to self.team
                } else {
                    //out += "finised move\n";
                    moves.push(moveCheck.get_win_probability(self.team));
                }
            }
        }
        //Edge case - no moves to make, return 0 probability (can't win)
        if moves.len() == 0 {
            0f32
        //Otherwise, return average of all possibilities
        } else {
            moves.iter().sum::<f32>()/(moves.len() as f32)
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

    mod MoveCheck {
        use super::*;
        use connect4::ai::MoveCheck;

        mod get_win_probability { 
            use super::*;

            #[test]
            fn should_return_inverse_probs_for_diff_teams() {
                let data = vec![vec![1,1,1,1,1,0]];
                let check = MoveCheck::new(create_test_board(data), 0, 1);
                assert_eq!(check.get_win_probability(1), 1.0);
                assert_eq!(check.get_win_probability(2), 0.0);
            }

            #[test]
            fn should_return_value_between_0_and_1() {
                let data = vec![vec![1,1,1,1,1,0],
                                vec![1,1,1,1,1,1],
                                vec![1,1,1,1,1,1],
                                vec![1,1,1,1,1,1], 
                                vec![1,1,1,1,1,1],
                                vec![1,1,1,1,1,1],
                                vec![1,1,1,1,1,1]];
                let check = MoveCheck::new(create_test_board(data), 0, 1);
                assert_eq!(check.get_win_probability(1), 1.0);
                assert_eq!(check.get_win_probability(2), 0.0);
            }
        }
    }

    mod AI {
        use super::*;
        use connect4::ai::AI;

        mod find_win_probability { 
            use super::*;

            #[test]
            fn should_default_to_0_if_board_full() {
                let data = vec![vec![1,1,1,1,1,1],
                                vec![1,1,1,1,1,1],
                                vec![1,1,1,1,1,1],
                                vec![1,1,1,1,1,1], 
                                vec![1,1,1,1,1,1],
                                vec![1,1,1,1,1,1],
                                vec![1,1,1,1,1,1]];
                let board = create_test_board(data);
                let testAI = AI::new(1,1);
                assert_eq!(testAI.find_win_probability(board.clone(), 0, 3), 0.0);
                assert_eq!(testAI.find_win_probability(board.clone(), 1, 3), 0.0);
            }

            #[test]
            fn should_return_win_or_loss_with_4_run() {
                let data = vec![vec![1,1,2,1,2,0],
                                vec![1,2,2,1,2,2],
                                vec![2,2,2,0,0,0],
                                vec![1,1,1,1,0,0], 
                                vec![0,0,0,0,0,0],
                                vec![1,1,2,1,2,1],
                                vec![1,1,2,2,2,0]];
                let board = create_test_board(data);
                let testAI = AI::new(1,1);
                assert_eq!(testAI.find_win_probability(board.clone(), 0, 3), 1.0);
                assert_eq!(testAI.find_win_probability(board.clone(), 1, 3), 0.0);
            }

            #[test]
            fn should_return_average_of_options() {
                let data = vec![vec![2,2,2,2,2,2],
                                vec![2,2,2,2,2,2],
                                vec![2,2,2,2,2,2],
                                vec![2,2,2,2,2,0], //Prob of 1/8 here (one run of length 1)
                                vec![2,2,1,0,0,0], //Prob of 1/4 here (one run of length 2)
                                vec![2,2,2,2,2,0], //Prob of 1/8 here (one run of length 1)
                                vec![1,1,0,0,0,0]]; //Prob of 1/2 (here (one run of length 3))
                                //Average prob is thus 1/4
                let board = create_test_board(data);
                let testAI = AI::new(1,1);
                assert_eq!(testAI.find_win_probability(board.clone(), 0, 0), 0.25);
            }
        }
    }
}