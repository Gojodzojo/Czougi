mod game;
use crossterm::Result;
use game::Game;
use std::io::stdout;

fn main() -> Result<()> {
    let stdout = stdout();
    let mut game = Game::new(stdout)?;

    game.init()?;
    game.run()?;
    game.uninit()?;

    Ok(())
}
