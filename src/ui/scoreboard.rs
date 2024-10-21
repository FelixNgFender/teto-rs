use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::Stylize,
    text::Line,
    widgets::{block::Title, Block, BorderType, Row, Table, Widget},
};

pub struct Scoreboard {
    score: u32,
    level: u8,
}

impl Scoreboard {
    pub fn new(score: u32, level: u8) -> Self {
        Scoreboard { score, level }
    }
}

impl Widget for &Scoreboard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let scoreboard_rows = [
            Row::new(vec![
                Line::from("Score").alignment(Alignment::Left),
                Line::from(self.score.to_string().yellow()).alignment(Alignment::Right),
            ]),
            Row::new(vec![
                Line::from("Level").alignment(Alignment::Left),
                Line::from(self.level.to_string().blue()).alignment(Alignment::Right),
            ]),
        ];
        let scoreboard_widths = Constraint::from_mins([40; 2]);
        let scoreboard_title = Title::from(" Stats ".bold());
        let scoreboard_container = Block::bordered()
            .title(scoreboard_title.alignment(Alignment::Center))
            .border_type(BorderType::Thick);
        let scoreboard = Table::new(scoreboard_rows, scoreboard_widths).block(scoreboard_container);

        scoreboard.render(area, buf);
    }
}
