// Whenever you se some empty comments where there is a grid,
// those comments keep the grid intact when cargo fmt is run

use rusty_2048::{Direction, Game, GameStatus};

//
// Test for Game::default
//

#[test]
fn game_default() {
    let game = Game::default();
    let expected: [[u16; 4]; 4] = [
        [0, 0, 0, 0], //
        [0, 0, 0, 0], //
        [0, 0, 0, 0], //
        [0, 0, 0, 0], //
    ];
    assert_eq!(game.grid, expected);
}

//
// Tests for Game.fmt, 2 variations
//

#[test]
fn game_fmt_v1() {
    let result = format!("{}", Game::default());
    let expected = "╔════════════╗
║            ║
║            ║
║            ║
║            ║
╚════════════╝"
        .to_owned();
    assert_eq!(result, expected);
}

#[test]
fn game_fmt_v2() {
    let game = Game {
        grid: [
            [0, 8, 2, 0],  //
            [4, 16, 0, 0], //
            [2, 4, 0, 0],  //
            [8, 2, 16, 2], //
        ],
    };
    let result = format!("{}", game);
    let expected = "╔════════════╗
║     8  2   ║
║  4 16      ║
║  2  4      ║
║  8  2 16  2║
╚════════════╝"
        .to_owned();
    assert_eq!(result, expected);
}

//
// Tests for Game.get_free, 3 variations (16,12,10)
//

#[test]
fn game_get_free_v1() {
    let game = Game::default();
    let result = game.get_free();
    assert_eq!(result, 16);
}

#[test]
fn game_get_free_v2() {
    let mut game = Game::default();
    game.grid[0] = [2, 4, 4, 2];
    let result = game.get_free();
    assert_eq!(result, 12);
}

#[test]
fn game_get_free_v3() {
    let mut game = Game::default();
    game.grid[0] = [2, 4, 4, 2];
    game.grid[1][2] = 16;
    game.grid[3][1] = 32768;
    let result = game.get_free();
    assert_eq!(result, 10);
}

//
// Test for Game.rand_new_tile, 8 iterations
//

#[test]
fn game_rand_new_tile8() {
    let mut game = Game::default();
    for _ in 0..8 {
        game.rand_new_tile();
    }
    let mut result: u8 = 0;
    for row in game.grid {
        for element in row {
            match element {
                2 | 4 => result += 1,
                _ => (),
            }
        }
    }
    assert_eq!(result, 8);
}

//
// Tests for Game.sum_tiles, 4 directions
//

#[test]
fn game_sum_tiles_down() {
    let mut game = Game {
        grid: [
            [2, 2, 8, 8],   //
            [2, 2, 2, 8],   //
            [16, 16, 2, 2], //
            [2, 16, 2, 2],  //
        ],
    };
    game.sum_tiles(&Direction::Down);
    let expected: [[u16; 4]; 4] = [
        [0, 0, 8, 0],  //
        [4, 4, 2, 16], //
        [16, 0, 0, 0], //
        [2, 32, 4, 4], //
    ];
    assert_eq!(game.grid, expected);
}

#[test]
fn game_sum_tiles_up() {
    let mut game = Game {
        grid: [
            [2, 2, 8, 8],   //
            [2, 2, 2, 8],   //
            [16, 16, 2, 2], //
            [2, 16, 2, 2],  //
        ],
    };
    game.sum_tiles(&Direction::Up);
    let expected: [[u16; 4]; 4] = [
        [4, 4, 8, 16],  //
        [0, 0, 4, 0],   //
        [16, 32, 0, 4], //
        [2, 0, 2, 0],   //
    ];
    assert_eq!(game.grid, expected);
}

#[test]
fn game_sum_tiles_left() {
    let mut game = Game {
        grid: [
            [2, 2, 8, 8],   //
            [2, 2, 2, 8],   //
            [16, 16, 2, 2], //
            [2, 16, 2, 2],  //
        ],
    };
    game.sum_tiles(&Direction::Left);
    let expected: [[u16; 4]; 4] = [
        [4, 0, 16, 0], //
        [4, 0, 2, 8],  //
        [32, 0, 4, 0], //
        [2, 16, 4, 0], //
    ];
    assert_eq!(game.grid, expected);
}

#[test]
fn game_sum_tiles_right() {
    let mut game = Game {
        grid: [
            [2, 2, 8, 8],   //
            [2, 2, 2, 8],   //
            [16, 16, 2, 2], //
            [2, 16, 2, 2],  //
        ],
    };
    game.sum_tiles(&Direction::Right);
    let expected: [[u16; 4]; 4] = [
        [0, 4, 0, 16], //
        [2, 0, 4, 8],  //
        [0, 32, 0, 4], //
        [2, 16, 0, 4], //
    ];
    assert_eq!(game.grid, expected);
}

//
// Tests for Game.gravity, 4 directions
//

#[test]
fn game_gravity_down() {
    let mut game = Game {
        grid: [
            [2, 0, 4, 0], //
            [0, 2, 0, 4], //
            [4, 0, 8, 2], //
            [0, 4, 2, 0], //
        ],
    };
    game.gravity(&Direction::Down);
    let expected: [[u16; 4]; 4] = [
        [0, 0, 0, 0], //
        [0, 0, 4, 0], //
        [2, 2, 8, 4], //
        [4, 4, 2, 2], //
    ];
    assert_eq!(game.grid, expected);
}

#[test]
fn game_gravity_up() {
    let mut game = Game {
        grid: [
            [2, 0, 4, 0], //
            [0, 2, 0, 4], //
            [4, 0, 8, 2], //
            [0, 4, 2, 0], //
        ],
    };
    game.gravity(&Direction::Up);
    let expected: [[u16; 4]; 4] = [
        [2, 2, 4, 4], //
        [4, 4, 8, 2], //
        [0, 0, 2, 0], //
        [0, 0, 0, 0], //
    ];
    assert_eq!(game.grid, expected);
}

#[test]
fn game_gravity_left() {
    let mut game = Game {
        grid: [
            [2, 0, 4, 0], //
            [0, 2, 0, 4], //
            [4, 0, 8, 2], //
            [0, 4, 2, 0], //
        ],
    };
    game.gravity(&Direction::Left);
    let expected: [[u16; 4]; 4] = [
        [2, 4, 0, 0], //
        [2, 4, 0, 0], //
        [4, 8, 2, 0], //
        [4, 2, 0, 0], //
    ];
    assert_eq!(game.grid, expected);
}

#[test]
fn game_gravity_right() {
    let mut game = Game {
        grid: [
            [2, 0, 4, 0], //
            [0, 2, 0, 4], //
            [4, 0, 8, 2], //
            [0, 4, 2, 0], //
        ],
    };
    game.gravity(&Direction::Right);
    let expected: [[u16; 4]; 4] = [
        [0, 0, 2, 4], //
        [0, 0, 2, 4], //
        [0, 4, 8, 2], //
        [0, 0, 4, 2], //
    ];
    assert_eq!(game.grid, expected);
}

//
// Tests for Game.check_end, 4 variations
//

#[test]
fn game_check_end_v1() {
    let game = Game {
        grid: [
            [2, 0, 4, 0], //
            [0, 2, 0, 4], //
            [4, 0, 8, 2], //
            [0, 4, 2, 0], //
        ],
    };
    let result: GameStatus = game.check_end();

    let expected = GameStatus::Running;

    assert_eq!(result, expected);
}

#[test]
fn game_check_end_v2() {
    let game = Game {
        grid: [
            [2, 2, 4, 4], //
            [4, 4, 2, 2], //
            [2, 2, 4, 4], //
            [4, 4, 2, 2], //
        ],
    };
    let result: GameStatus = game.check_end();

    let expected = GameStatus::Running;

    assert_eq!(result, expected);
}

#[test]
fn game_check_end_v3() {
    let game = Game {
        grid: [
            [2, 4, 2, 4], //
            [2, 4, 2, 4], //
            [4, 2, 4, 2], //
            [4, 2, 4, 2], //
        ],
    };
    let result: GameStatus = game.check_end();

    let expected = GameStatus::Running;

    assert_eq!(result, expected);
}

#[test]
fn game_check_end_v4() {
    let game = Game {
        grid: [
            [2, 4, 2, 4], //
            [4, 2, 4, 2], //
            [2, 4, 2, 4], //
            [4, 2, 4, 2], //
        ],
    };
    let result: GameStatus = game.check_end();

    let expected = GameStatus::Ended;

    assert_eq!(result, expected);
}
