use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::Stylize,
    text::Line,
    widgets::{block::Title, Block, BorderType, Row, Table, Widget},
};

pub struct Help;

impl Widget for &Help {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let help_rows = [
            Row::new(vec![
                Line::from("Left").alignment(Alignment::Left),
                Line::from("h, ←").alignment(Alignment::Right),
            ]),
            Row::new(vec![
                Line::from("Down").alignment(Alignment::Left),
                Line::from("j, ↓").alignment(Alignment::Right),
            ]),
            Row::new(vec![
                Line::from("Rotate").alignment(Alignment::Left),
                Line::from("k, ↑").alignment(Alignment::Right),
            ]),
            Row::new(vec![
                Line::from("Right").alignment(Alignment::Left),
                Line::from("l, →").alignment(Alignment::Right),
            ]),
            Row::new(vec![
                Line::from("Drop").alignment(Alignment::Left),
                Line::from("space").alignment(Alignment::Right),
            ]),
            Row::new(vec![
                Line::from("Quit").alignment(Alignment::Left),
                Line::from("q").alignment(Alignment::Right),
            ]),
        ];
        let help_widths = Constraint::from_mins([20; 2]);
        let help_title = Title::from(" Help ".bold());
        let help_container = Block::bordered()
            .title(help_title.alignment(Alignment::Center))
            .border_type(BorderType::Thick);
        let help = Table::new(help_rows, help_widths).block(help_container);

        help.render(area, buf);
    }
}
