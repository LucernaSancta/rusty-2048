use crate::game_logic;

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