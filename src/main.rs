fn main() {
    let mut grid: [[u16; 4]; 4] = create_grid();

    // In 2048 the initial gfrid has three random values
    rand_new_tile(&mut grid);
    rand_new_tile(&mut grid);
    rand_new_tile(&mut grid);

    // Game loop
    loop {
        println!("{}", get_free(&grid));
        rand_new_tile(&mut grid);
        for row in grid {
            println!("{:?}", row);
        }

        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);

        sum_tiles(&mut grid);
    }
}

fn create_grid() -> [[u16; 4]; 4] {
    // Create the 4x4 grid of u16 unsigned integers

    let grid: [[u16; 4]; 4] = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    return grid;
}

fn rand_new() -> u16 {
    // Get a random number (either 90% 2 or  10% 4)

    let n: u8 = rand::random_range(0..10);
    return match n {
        0 => 4,
        _ => 2
    }
}

fn get_free(grid: &[[u16; 4]; 4]) -> u8 {
    // Get the number of 0s in the grid

    let mut n: u8 = 0;
    for row in grid {
        for element in row {
            if *element == 0 {
                n += 1;
            }
        }
    }
    return n;
}

fn rand_new_tile(grid: &mut [[u16; 4]; 4]) {
    // Place a new value in a random empty tile,
    // does nothing if no tile is empty

    let free: u8 = get_free(&grid);
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

fn sum_tiles(grid: &mut [[u16; 4]; 4]) {
    // Sum equal tiles so that there s only one tile with double the value

    // 1..4 (1,2,3) because we only need 3 additions:
    // rows 0-1, 1-2 and 2-3
    for row in (1..4).rev() {
        // Here we take all 4 elements of the row, not only 3
        for element in 0..4 {
            if grid[row][element] == grid[row-1][element] {
                // Double the element below and delete the element above
                grid[row][element] *= 2;
                grid[row-1][element] = 0;
            }
        }
    }
}
