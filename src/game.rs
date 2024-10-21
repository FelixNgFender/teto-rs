use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame};
use std::io;

#[derive(Debug, Default)]
pub struct GameState {
    pub score: u32,
    pub level: u8,
    should_quit: bool,
}

impl GameState {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.decrement_counter(),
            KeyCode::Right => self.increment_counter(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.should_quit = true;
    }

    fn increment_counter(&mut self) {
        self.score = self.score.saturating_add(1);
    }

    fn decrement_counter(&mut self) {
        self.score = self.score.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = GameState::default();
        app.handle_key_event(KeyCode::Right.into());
        assert_eq!(app.score, 1);

        app.handle_key_event(KeyCode::Left.into());
        assert_eq!(app.score, 0);

        let mut app = GameState::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.should_quit);

        Ok(())
    }
}
