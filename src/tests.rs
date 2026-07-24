use crate::game_logic;
use crate::game_logic::Direction;

#[test]
fn test_grid_setup() {
    let grid1 = game_logic::create_grid();
    let grid2: [[u16; 4]; 4] = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    assert_eq!(grid1, grid2);
}

#[test]
fn test_rand_new() {
    for _ in 0..100{
        let n = game_logic::rand_new();
        assert!((n == 2) || (n == 4));
    }
}

#[test]
fn test_get_free16() {
    let grid = game_logic::create_grid();
    let n = game_logic::get_free(&grid);
    assert_eq!(n, 16);
}

#[test]
fn test_get_free12() {
    let mut grid = game_logic::create_grid();
    grid[0] = [2,4,4,2];
    let n = game_logic::get_free(&grid);
    assert_eq!(n, 12);
}

#[test]
fn test_get_free10() {
    let mut grid = game_logic::create_grid();
    grid[0] = [2,4,4,2];
    grid[1][2] = 16;
    grid[3][1] = 32768;
    let n = game_logic::get_free(&grid);
    assert_eq!(n, 10);
}

#[test]
fn test_rand_new_tile8() {
    let mut grid = game_logic::create_grid();
    for _ in 0..8 {
        game_logic::rand_new_tile(&mut grid);
    }
    let mut counter: u8 = 0;
    for row in grid {
        for element in row {
            match element {
                2 | 4 => counter += 1,
                _ => (),
            }
        }
    }
    assert_eq!(counter, 8);
}






// Unit tests for sum_tiles here
// Unit tests for sum_tiles here
// Unit tests for sum_tiles here






#[test]
fn test_gravity_down() {
    let mut grid1: [[u16; 4]; 4] = [
        [2, 0, 4, 0],
        [0, 2, 0, 4],
        [4, 0, 8, 2],
        [0, 4, 2, 0]
    ];
    game_logic::gravity(&mut grid1, Direction::Down);
    let grid2: [[u16; 4]; 4] = [
        [0, 0, 0, 0],
        [0, 0, 4, 0],
        [2, 2, 8, 4],
        [4, 4, 2, 2]
    ];
    assert_eq!(grid1, grid2);
}

#[test]
fn test_gravity_up() {
    let mut grid1: [[u16; 4]; 4] = [
        [2, 0, 4, 0],
        [0, 2, 0, 4],
        [4, 0, 8, 2],
        [0, 4, 2, 0]
    ];
    game_logic::gravity(&mut grid1, Direction::Up);
    let grid2: [[u16; 4]; 4] = [
        [2, 2, 4, 4],
        [4, 4, 8, 2],
        [0, 0, 2, 0],
        [0, 0, 0, 0]
    ];
    assert_eq!(grid1, grid2);
}

#[test]
fn test_gravity_left() {
    let mut grid1: [[u16; 4]; 4] = [
        [2, 0, 4, 0],
        [0, 2, 0, 4],
        [4, 0, 8, 2],
        [0, 4, 2, 0]
    ];
    game_logic::gravity(&mut grid1, Direction::Left);
    let grid2: [[u16; 4]; 4] = [
        [2, 4, 0, 0],
        [2, 4, 0, 0],
        [4, 8, 2, 0],
        [4, 2, 0, 0]
    ];
    assert_eq!(grid1, grid2);
}

#[test]
fn test_gravity_right() {
    let mut grid1: [[u16; 4]; 4] = [
        [2, 0, 4, 0],
        [0, 2, 0, 4],
        [4, 0, 8, 2],
        [0, 4, 2, 0]
    ];
    game_logic::gravity(&mut grid1, Direction::Right);
    let grid2: [[u16; 4]; 4] = [
        [0, 0, 2, 4],
        [0, 0, 2, 4],
        [0, 4, 8, 2],
        [0, 0, 4, 2]
    ];
    assert_eq!(grid1, grid2);
}