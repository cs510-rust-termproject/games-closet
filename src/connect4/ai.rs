// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use connect4::core::GridPosition;
use connect4::core::Board;
use connect4::core::GameState;
use connect4::core::BOARD_SIZE;
use std::cmp::Ordering;

struct MoveCheck {
    team: i32,
    board: Board,
    runs: [i32;4]
}

impl MoveCheck {
    fn init(board: Board, moveCol: i32, team: i32) -> Self {
        let newBoard = board.clone();
        let runs = newBoard.get_runs_from_point(GridPosition::new(moveCol, newBoard.get_column_height(moveCol as usize) as i32), team);
        newBoard.add_disc(moveCol, team);
        MoveCheck { team: team, board: newBoard, runs: runs }
    }

    /*fn compare(&self, other: MoveCheck) -> i32 {
        for i in (0..4).rev() {
            if self.runs[i] != other.runs[i] {
                return i*(self.runs[i]-other.runs[i]);
            }
        }
        0
    }*/

    fn has_end_result(&self) -> bool {
        self.runs[4] > 0
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

struct AI {
    team: i32,
    difficulty: i32
}

impl AI {
    fn init(team: i32, difficulty: i32) -> Self {
        AI { team, difficulty }
    }

    fn pick_optimal_move(&self, state: GameState) -> i32 {
        let mut bestMove = -1;
        let mut bestProb = 0.0;
        for i in 0..BOARD_SIZE.0 {
            let currBoard = state.board.clone().add_disc(i);
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
        let moves = Vec::new();
        for i in 0..BOARD_SIZE.0 {
            if !board.is_column_full(i as usize) {
                if currMove & 2 == 1 {
                    let oppMove = MoveCheck::init(board, i, self.team%2 + 1);
                    //If move would cause other team to win, return 0 as prob. Otherwise, call recursively on next move
                    if oppMove.has_end_result() {
                        return 0.0;
                    } else if currMove < lastMove {
                        return self.find_win_probability(oppMove.board, currMove+1, lastMove)
                    } else {
                        moves.push(oppMove);
                    }
                } else {
                    let aiMove = MoveCheck::init(board, i, self.team);
                    //If move would cause AI to win, return 1 as prob. Otherwise, call recursively on next move
                    if aiMove.has_end_result() {
                        return 1.0;
                    } else if currMove < lastMove {
                        return self.find_win_probability(aiMove.board, currMove+1, lastMove);
                    } else {
                        moves.push(aiMove);
                    }
                }
            }
        }
        //Edge case - no moves to make, return 0 probability (can't win)
        if moves.len() == 0 {
            0f32
        //Base case - currMove >= lastMove, so function falls through to here to make guestimate
        } else {
            moves.sort();
            //Opp turn 
            if currMove & 2 == 1 {
                let worstCaseRuns = moves.get(moves.len()-1).unwrap().runs;
                //Return 1-weighted prob of enemy getting winning move in future (divided by two because duplicates run in opposite directions
                //likely created). Also doesn't go below probability of 0 for consistency (why .max(0) is added)
                (1.0-(0.5*(worstCaseRuns[3] as f32)+0.25*(worstCaseRuns[2] as f32)+0.125*(worstCaseRuns[1] as f32))/2.0).max(0.0)
            //AI turn
            } else {
                let bestCaseRuns = moves.get(0).unwrap().runs;
                //Return weighted prob of enemy getting winning move in future (divided by two because duplicates run in opposite directions
                //likely created). Also doesn't go above probability of 1 for consistency why .min(1) is added)
                (0.5*(bestCaseRuns[3] as f32)+0.25*(bestCaseRuns[2] as f32)+0.125*(bestCaseRuns[1] as f32)/2.0).min(1.0)
            }
        }
    }
}



/*fn main() {
    panic!("Connect 4 is not implemented yet!")
}*/