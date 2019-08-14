# games-closet
Copyright (c) 2019 Andre Mukhsia, Lane Barton

A Rust crate for CS510 - Rust Programming aiming to implement various basic board/card games.

This crate leverages the [GGEZ crate](https://github.com/ggez/ggez) for creating interactable 2D versions of common games. It allows users to select a game and play against AI opponents and potentially multiplayer games with other live users as well. See the table below for the current status of games intended for playable use in this crate.

| Game       | Implementation Status               | AI Opponents | Multiplayer |
| ---------- | ----------------------------------- | ------------ | ----------- |
| Connect 4  | Done                                | Done         | Done        |
| Battleship | Potential for future implementation | TBD          | TBD         |
| Hearts     | Potential for implementation        | TBD          | TBD         |

For additional implementation on how to run and play each game, see the [How To Play a Game](#how-to-play-a-game) section further down in this README.


## Contributors
Andre Mukhsia <mukhsia@pdx.edu>

Lane Barton <bartoniv@pdx.edu>

## License
This work is released under the "MIT License". Please see the file [LICENSE](LICENSE) in this distribution for license terms.

## How to Play a Game

1. Open the Terminal or Command Prompt and navigate to the games-closet directory
2. Enter `cargo run` to build and run the the program
3. Select a game by clicking a button under `Select Game`
4. Select the number of 'Human' players that will be playing the game; AI will fill player spot(s) if 0 or 1 is selected
5. Click `Start Game` to start the game

### How To Play the Game - Connect 4

1. Players can click non-full columns on the board to insert their disc during their turn
2. Inserted disc will be placed at the lowest empty cell of the column
3. Inserting a disc ends a player's turn, starting the opponent's turn to insert a disc
4. The objective of the game is to connect four of the player's discs horizontally, vertically, or diagonally in order to win
5. If the board is full (No more columns to insert a disc is available) the game ends in a draw
6. The turn indicator above the board displays the current player's turn and winner/ draw gameover message
7. A player can press on the `Main Menu` button to go back to the main menu or `Reset` button to restart the game with an empty board

## Developers Notes

Additional notes from the developers on project management and code design

### Repo Structure

Since this crate is intended to house multiple crates, the `src` folder house directories for each game and all general files, such as the main menu in `src/main.rs`. Game specific files are stored in their specific directory and should be included in that directory's `mod.rs` file as `pub mod file_name` so it can be accessed in outer scopes. As an example, the `src/connect4` folder contains `core.res` and `ai.rs` specific to the Connect 4 game and `src/connect4/mod.rs` contains the lines
```
pub mod core;
pub mod ai;
```

### Testing

This project emphasized two forms of testing - unit testing code and play testing through executing `cargo_run`. Due to the usage of an interactive, 2D game platform, some methods and implementation were tied directly into mouse and screen features that would be hard to replicate with unit tests. As such, much testing was done through `cargo run` and stepping through our UI implentation, using logs in the terminal plus the UI to gauage game state and expected behaviors

Unit tests for methods are included in their respective program files. To execute the unit tests, run `cargo test`. Test modules were labelled in the following order:
```
file_path::struc_test::method::behavior_tested
```
For for example, the unit test `connect4::ai::ai_tests::move_check::get_win_probability::should_return_inverse_probs_for_diff_teams` is in the `src/connect4/ai.rs` file, is testing the `AI` struct, the `get_win_probability` method of that struct, and is evaluating whether the method returns inverse probabilities for different  team inputs.



