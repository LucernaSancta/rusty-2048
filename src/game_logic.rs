use std::cmp::Ordering;
use std::fmt;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "Direction::Up"),
            Direction::Down => write!(f, "Direction::Down"),
            Direction::Left => write!(f, "Direction::Left"),
            Direction::Right => write!(f, "Direction::Right"),
        }
    }
}

pub enum GameStatus {
    Ongoing,
    Ended,
}

pub fn create_grid() -> [[u16; 4]; 4] {
    // Create the 4x4 grid of u16 unsigned integers

    #[rustfmt::skip]
    let grid: [[u16; 4]; 4] = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];

    // Return grid
    grid
}

pub fn rand_new() -> u16 {
    // Get a random number (either 90% 2 or  10% 4)

    let n: u8 = rand::random_range(0..10);
    // Return matched
    match n {
        0 => 4,
        _ => 2,
    }
}

pub fn get_free(grid: &[[u16; 4]; 4]) -> u8 {
    // Get the number of 0s in the grid

    let mut n: u8 = 0;
    for row in grid {
        for element in row {
            if *element == 0 {
                n += 1;
            }
        }
    }
    // Return n
    n
}

pub fn rand_new_tile(grid: &mut [[u16; 4]; 4]) {
    // Place a new value in a random empty tile,
    // does nothing if no tile is empty

    let free: u8 = get_free(grid);
    if free == 0 {
        return;
    }

    let mut reverse_counter: u8 = rand::random_range(0..free);
    // Remove 1 from the counter for every 0 in the grid,
    // place a new value if the counter is set to 0 and
    // the current tile is 0
    for row in grid {
        for element in row {
            // Is the tile 0? and is it the end of the counter
            if *element == 0 {
                if reverse_counter == 0 {
                    *element = rand_new();
                    return;
                } else {
                    reverse_counter -= 1;
                }
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
pub fn sum_tiles(grid: &mut [[u16; 4]; 4], direction: &Direction) {
    // Sum equal tiles so that there s only one tile with double the value

    match direction {
        Direction::Down => {
            // 1..4 (1,2,3) because we only need 3 additions:
            // rows 0-1, 1-2 and 2-3
            for row in (1..4).rev() {
                // Here we take all 4 elements of the row, not only 3
                for element in 0..4 {
                    if grid[row][element] == grid[row - 1][element] {
                        // Double the element below and delete the element above
                        grid[row][element] *= 2;
                        grid[row - 1][element] = 0;
                    }
                }
            }
        }
        Direction::Up => {
            for row in 0..3 {
                for element in 0..4 {
                    if grid[row][element] == grid[row + 1][element] {
                        grid[row][element] *= 2;
                        grid[row + 1][element] = 0;
                    }
                }
            }
        }
        Direction::Left => {
            // Same thing as before but now we can use directly
            // the row because we don't have to apply logic vertically
            for row in grid {
                for element in 0..3 {
                    if row[element] == row[element + 1] {
                        row[element] *= 2;
                        row[element + 1] = 0;
                    }
                }
            }
        }
        Direction::Right => {
            for row in grid {
                for element in (1..4).rev() {
                    if row[element] == row[element - 1] {
                        row[element] *= 2;
                        row[element - 1] = 0;
                    }
                }
            }
        }
    }
}

fn sort_left(_: &u16, b: &u16) -> Ordering {
    if *b == 0 {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn sort_right(a: &u16, _: &u16) -> Ordering {
    if *a == 0 {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

#[allow(clippy::needless_range_loop)]
pub fn gravity(grid: &mut [[u16; 4]; 4], direction: &Direction) {
    // Compresses all value of a row/comuln in one direction
    // [2,0,4,0], Right => [0,0,2,4]

    match direction {
        Direction::Down => {
            // Create a temporary array and place the values
            // of a column, sort the array and put the values back
            for y in 0..4 {
                let mut temp_array: [u16; 4] = [0, 0, 0, 0];
                for x in 0..4 {
                    temp_array[x] = grid[x][y];
                }
                temp_array.sort_by(sort_right);
                for x in 0..4 {
                    grid[x][y] = temp_array[x];
                }
            }
        }
        Direction::Up => {
            for y in 0..4 {
                let mut temp_array: [u16; 4] = [0, 0, 0, 0];
                for x in 0..4 {
                    temp_array[x] = grid[x][y];
                }
                temp_array.sort_by(sort_left);
                for x in 0..4 {
                    grid[x][y] = temp_array[x];
                }
            }
        }
        Direction::Left => {
            // Sort the row with the custom function
            for row in grid {
                row.sort_by(sort_left);
            }
        }
        Direction::Right => {
            for row in grid {
                row.sort_by(sort_right);
            }
        }
    }
}

#[allow(clippy::needless_range_loop)]
pub fn check_end(grid: &[[u16; 4]; 4]) -> GameStatus {
    // Check if the game is in a stall position (end game)

    // Empty spaces = game still playable
    if get_free(grid) != 0 {
        return GameStatus::Ongoing;
    }

    // Check for summable values (left, right)
    for row in grid {
        for i in 0..3 {
            if row[i] == row[i + 1] {
                return GameStatus::Ongoing;
            }
        }
    }

    // Check for summable values (up, down)
    for row in 0..3 {
        for element in 0..4 {
            if grid[row][element] == grid[row + 1][element] {
                return GameStatus::Ongoing;
            }
        }
    }

    // If all the revious checks fail than th game is in a
    // stall position -> game over
    GameStatus::Ended
}
