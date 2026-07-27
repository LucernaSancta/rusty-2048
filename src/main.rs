#[rustfmt::skip]
#[cfg(test)]
mod tests;

mod game_logic;
use game_logic::{Game, Direction, GameStatus};

fn main() {
    simple_game_cli();
}

#[allow(dead_code)]
fn simple_game_cli() {
    let mut game = Game::default();
    // Random direction, doesen't really matter
    let mut direction: Direction = Direction::Down;

    // In 2048 the initial gfrid has three random values
    game.rand_new_tile();
    game.rand_new_tile();
    game.rand_new_tile();

    // Game loop
    loop {
        // Print the game
        println!("{}", game);

        match game.check_end() {
            GameStatus::Running => (),
            GameStatus::Ended => break,
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
        if ["w", "a", "s", "d"].contains(&buf.strip_suffix("\n").unwrap()) {
            game.gravity(&direction);
            game.sum_tiles(&direction);
            game.gravity(&direction);

            game.rand_new_tile();
        }
    }
}
