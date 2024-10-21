use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    widgets::{block::Title, Block, BorderType, Widget},
};

pub struct Gameboard;

impl Widget for &Gameboard {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Tetorisu ".bold());
        let next_block = Block::bordered()
            .title(title.alignment(Alignment::Center))
            .border_type(BorderType::Thick);
        next_block.render(area, buf);
    }
}
