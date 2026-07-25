#[rustfmt::skip]
#[cfg(test)]
mod tests;

mod game_logic;
use game_logic::Direction;

fn main() {
    let mut grid: [[u16; 4]; 4] = game_logic::create_grid();

    // In 2048 the initial gfrid has three random values
    game_logic::rand_new_tile(&mut grid);
    game_logic::rand_new_tile(&mut grid);
    game_logic::rand_new_tile(&mut grid);

    // Game loop
    loop {
        game_logic::rand_new_tile(&mut grid);
        println!("{}", game_logic::get_free(&grid));
        for row in grid {
            println!("{:?}", row);
        }

        let mut buf = String::new();
        let _ = std::io::stdin().read_line(&mut buf);

        game_logic::sum_tiles(&mut grid, Direction::Down);
    }
}
