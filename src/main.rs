mod settings;
mod game;

use settings::{WINDOW_WIDTH, WINDOW_HEIGHT, TITLE};

fn main() {
    let mut g = game::Game::new(WINDOW_WIDTH, WINDOW_HEIGHT, TITLE);
    g.run();
}
