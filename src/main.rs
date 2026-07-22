fn main() {
    let mut grid: [[u16; 4]; 4] = create_grid();
    grid[0][0] = 2;
    println!("{:?}", grid);
    println!("{}", rand_new());
}

fn create_grid() -> [[u16; 4]; 4] {
    let grid: [[u16; 4]; 4] = [
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0],
        [0, 0, 0, 0]
    ];
    return grid;
}

fn rand_new() -> u16 {
    let n: u8 = rand::random_range(0..10);
    return match n {
        0 => 4,
        _ => 2
    }
}