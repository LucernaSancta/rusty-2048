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

#[derive(PartialEq, Debug)]
pub enum GameStatus {
    Running,
    Ended,
}

pub struct Game {
    pub grid: [[u16; 4]; 4],
}

impl Default for Game {
    fn default() -> Game {
        Game {
            grid: [
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0],
                [0, 0, 0, 0]
            ]
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "╔════════════╗")?;
        for row in &self.grid {
            write!(f, "║")?;
            for &x in row {
                if x == 0 {
                    // empty cell, padded to the same width as numbers
                    write!(f, "{:>3}", " ")?;
                } else {
                    write!(f, "{:>3}", x)?;
                }
            }
            writeln!(f, "║")?;
        }
        write!(f, "╚════════════╝")?;
        Ok(())
    }
}

impl Game {

    pub fn get_free(&self) -> u8 {
        // Get the number of 0s in the Game
        let mut n: u8 = 0;
        for row in self.grid {
            for element in row {
                if element == 0 {
                    n += 1;
                }
            }
        }
        n
    }

    pub fn rand_new_tile(&mut self) {
        // Place a new value in a random empty tile,
        // does nothing if no tile is empty

        let free: u8 = self.get_free();
        if free == 0 {
            return;
        }

        let mut reverse_counter: u8 = rand::random_range(0..free);
        // Remove 1 from the counter for every 0 in the Game,
        // place a new value if the counter is set to 0 and
        // the current tile is 0
        let grid = &mut self.grid;
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
    pub fn sum_tiles(&mut self, direction: &Direction) {
        // Sum equal tiles so that there s only one tile with double the value

        let grid = &mut self.grid;
    
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
    
    #[allow(clippy::needless_range_loop)]
    pub fn gravity(&mut self, direction: &Direction) {
        // Compresses all value of a row/comuln in one direction
        // [2,0,4,0], Right => [0,0,2,4]
    
        let grid = &mut self.grid;

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
    pub fn check_end(&self) -> GameStatus {
        // Check if the game is in a stall position (end game)
    
        // Empty spaces = game still playable
        if self.get_free() != 0 {
            return GameStatus::Running;
        }
    
        // Check for summable values (left, right)
        for row in self.grid {
            for i in 0..3 {
                if row[i] == row[i + 1] {
                    return GameStatus::Running;
                }
            }
        }
    
        // Check for summable values (up, down)
        for row in 0..3 {
            for element in 0..4 {
                if self.grid[row][element] == self.grid[row + 1][element] {
                    return GameStatus::Running;
                }
            }
        }
    
        // If all the revious checks fail than th game is in a
        // stall position -> game over
        GameStatus::Ended
    }

}

fn rand_new() -> u16 {
    // Get a random number (either 90% 2 or  10% 4)

    let n: u8 = rand::random_range(0..10);
    // Return matched
    match n {
        0 => 4,
        _ => 2,
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
