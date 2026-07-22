fn main() {
    let mut grid: [[u16; 4]; 4] = create_grid();
    grid[0][0] = 2;
    println!("{:?}", grid);
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