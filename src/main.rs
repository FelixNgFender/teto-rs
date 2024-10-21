use std::io;

use crate::game::GameState;

mod game;
mod ui;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = GameState::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
