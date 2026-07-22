fn main() {
    let mut grid: [[u16; 4]; 4] = create_grid();
    grid[0][0] = 2;
    println!("{:?}", grid);
    println!("{}", rand_new());
    print!("{}", get_free(&grid))
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

fn get_free(grid: &[[u16; 4]; 4]) -> u16 {
    // Get the number of zeros in the grid
    let mut n: u16 = 0;
    for row in grid {
        for element in row {
            if *element == 0 {
                n += 1;
            }
        }
    }
    return n;
}