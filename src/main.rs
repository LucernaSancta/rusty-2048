fn main() {
    let mut grid: [[u16; 4]; 4] = create_grid();

    // In 2048 the initial gfrid has three random values
    rand_new_tile(&mut grid);
    rand_new_tile(&mut grid);
    rand_new_tile(&mut grid);

    // Game loop
    loop {
        print!("{}", get_free(&grid));
        println!("{:?}", grid);
        rand_new_tile(&mut grid);

        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
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