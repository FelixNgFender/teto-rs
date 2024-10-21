use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, BorderType, Paragraph, Row, Table, Widget,
    },
};

use crate::game::GameState;

impl Widget for &GameState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let gameboard_title = Title::from(" Tetorisu ".bold());
        let gameboard_container = Block::bordered()
            .title(gameboard_title.alignment(Alignment::Center))
            .border_type(BorderType::Thick);
        let gameboard_paragraph = Paragraph::new("").centered().block(gameboard_container);

        gameboard_paragraph.render(area, buf);

        let next_block_title = Title::from(" Next ".bold());
        let next_block_container = Block::bordered()
            .title(next_block_title.alignment(Alignment::Center))
            .border_type(BorderType::Thick);

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
        let scoreboard_widths = [Constraint::Length(7), Constraint::Length(7)];
        let scoreboard_title = Title::from(" Stats ".bold());
        let scoreboard_container = Block::bordered()
            .title(scoreboard_title.alignment(Alignment::Center))
            .border_type(BorderType::Thick);
        let scoreboard = Table::new(scoreboard_rows, scoreboard_widths).block(scoreboard_container);

        scoreboard.render(area, buf);

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
        let help_widths = [Constraint::Length(7), Constraint::Length(7)];
        let help_title = Title::from(" Help ".bold());
        let help_container = Block::bordered()
            .title(help_title.alignment(Alignment::Center))
            .border_type(BorderType::Thick);
        let help = Table::new(help_rows, help_widths).block(help_container);

        help.render(area, buf);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use ratatui::style::Style;

    #[test]
    fn render() {
        let app = GameState::default();
        let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

        app.render(buf.area, &mut buf);

        let mut expected = Buffer::with_lines(vec![
            "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
        let title_style = Style::new().bold();
        let counter_style = Style::new().yellow();
        let key_style = Style::new().blue().bold();
        expected.set_style(Rect::new(14, 0, 22, 1), title_style);
        expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

        assert_eq!(buf, expected);
    }
}
