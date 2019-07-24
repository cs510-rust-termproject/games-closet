// Copyright © 2019 Andre Mukhsia, Lane Barton
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

mod core::*;

const BOARD_SIZE: (i32, i32) = (6, 7);

struct GameState {
    mut spaces: [[0i32;7];6];
}

impl GameState {
    fn get_board(&self) -> [[i32;7];6] {
        self.spaces
    }

    fn on_board(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < BOARD_SIZE.0 && y >= 0 && y < BOARD_SIZE.1
    }

    //Method to get a "max" run including a starting point in a target direction for a given team.
    //Accounts for runs towards and away from direction, but allows one space between tiles of the target team in
    //target direction but no spaces in reverse direction. 
    //The min value is 1; the max value returned is 4 even if a run is longer. If a space is used, the max returned value is 3 
    //(as the space presumably prevents an actual run of 4). Cases with a run of 4 prior to space will return 4, except for edge 
    //case where run goes from start and then completely in reverse direction. This can be caught by calling this method with reverse 
    //direction
    fn get_run_in_direction(&self, start: (i32, i32), direction: (i32, i32), team: u32) -> u32 {
        let mut dir_active = true;
        let mut rev_active = true;
        let mut space_used = false;
        let mut run_len = 1u32; //Start with dropped token
        let mut i = 1; //Start one beyond dropped token
        while run_len <= 4 && (dir1_active || dir2_active) {
            dir_active = dir1_active && self.on_board(start.0+i*dir.0, start.1+i*dir.1);
            rev_active = dir2_active && self.on_board(start.0-i*dir.0, start.1-i*dir.1);
            //Do reverse case first for edge case of AASA_A is treated as a run of 4 and not 3 with a space
            if rev_active {
                if self.board[start.0-i*dir.0][start.1-i*dir.1] == team {
                    run_len += 1;
                } else {
                    rev_active = false;
                }
            }
            if dir_active {
                let val = self.board[start.0+i*dir.0][start.1+i*dir.1];
                if val == team {
                    run_len += 1;
                    //Check for contiguous run of 4 before space, return immediately to prevent odd cases with spaces
                    if !space_used && run_len >= 4 {
                        return 4u32;
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
        } else {
            std::cmp::min(run_len, 4)
        }
    }

    //Method to return an array of runs from a start location for a given team, where array[i] returns the number of runs
    //of length i-1. Accounts for all eight directions, but may have false duplicates (e.g. a run BAAAB will return have two
    //runs of length 3 for team A even though technically its the same run)
    fn get_runs_from_point(&self, start: (i32, i32), team: u32) -> [u32;4] {
        let mut output = [0u32;4];
        let directions = vec![(1, 0), (1, 1), (0, 1), (-1, 1)];
        for dir in directions {
            output[self.get_run_in_direction(start, dir, team)-1] += 1;
            output[self.get_run_in_direction(start, (-1*dur.0, -1*dir.1), team)-1] += 1;
        }
        output
    }
}

struct MoveCheck {
    myMove: bool
    runs: [0u32;4]
}

impl MoveCheck {
    fn init(&mut self, board: GameState, moveCol: u32, myMove: bool) -> MoveCheck {
        let mut board = board.clone();
        board.add_disc(moveCol);

    }

    fn compare(&self, other: MoveCheck) -> i32 {
        for i in (0..4).rev() {
            if self.runs[i] != other.runs[i] {
                return i*(self.runs[i]-other.runs[i]);
            }
        }
        0
    }

    fn has_end_result(&self) -> bool {
        self.runs[4] > 0
    }
}

fn pick_optimal_move(state: GameState) -> i32 {
    let mut startBoard = GameState.get_board().clone();

}

fn main() {
    panic!("Connect 4 is not implemented yet!")
}