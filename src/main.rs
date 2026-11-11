mod game;
mod story;
mod utils;

use crate::game::Game;
use std::io::Error;

fn main() -> Result<(), Error> {
    Game::new().main_story()?;

    Ok(())
}
