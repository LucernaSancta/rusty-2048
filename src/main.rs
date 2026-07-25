#[rustfmt::skip]
#[cfg(test)]
mod tests;

mod game_logic;
use game_logic::Direction;

fn main() {
    let mut grid: [[u16; 4]; 4] = game_logic::create_grid();
    // Random direction, doesen't really matter
    let mut direction: Direction = Direction::Down;

    // In 2048 the initial gfrid has three random values
    game_logic::rand_new_tile(&mut grid);
    game_logic::rand_new_tile(&mut grid);
    game_logic::rand_new_tile(&mut grid);

    // Game loop
    loop {
        
        // Print the grid
        for row in grid {
            println!("{:?}", row);
        }

        // Get user input
        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);
        
        // Convert user input in to directions
        match buf.strip_suffix("\n").unwrap() {
            "w" => direction = Direction::Up,
            "s" => direction = Direction::Down,
            "a" => direction = Direction::Left,
            "d" => direction = Direction::Right,
            _ => (),
        }
        
        // If the user input is valid, step the game one tick
        if ["w", "a", "s", "d"].contains(&buf.strip_suffix("\n").unwrap()){
            game_logic::gravity(&mut grid, &direction);
            game_logic::sum_tiles(&mut grid, &direction);
            game_logic::gravity(&mut grid, &direction);
    
            game_logic::rand_new_tile(&mut grid);
        }
    }
}
