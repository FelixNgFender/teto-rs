use crate::{
    game::GameState,
    ui::{gameboard::Gameboard, help::Help, next_block::NextBlock, scoreboard::Scoreboard},
};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};

impl Widget for &GameState {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [left, middle, right] =
            Layout::horizontal(Constraint::from_fills([1, 2, 1])).areas(area);
        let [top_right, bottom_right] =
            Layout::vertical(Constraint::from_fills([1, 3])).areas(right);

        Scoreboard::new(self.score, self.level).render(left, buf);
        Gameboard.render(middle, buf);
        NextBlock.render(top_right, buf);
        Help.render(bottom_right, buf);
    }
}

// #[cfg(test)]
// mod tests {
//
//     use super::*;
//     use ratatui::style::Style;
//
//     #[test]
//     fn render() {
//         let app = GameState::default();
//         let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
//
//         app.render(buf.area, &mut buf);
//
//         let mut expected = Buffer::with_lines(vec![
//             "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
//             "┃                    Value: 0                    ┃",
//             "┃                                                ┃",
//             "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
//         ]);
//         let title_style = Style::new().bold();
//         let counter_style = Style::new().yellow();
//         let key_style = Style::new().blue().bold();
//         expected.set_style(Rect::new(14, 0, 22, 1), title_style);
//         expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
//         expected.set_style(Rect::new(13, 3, 6, 1), key_style);
//         expected.set_style(Rect::new(30, 3, 7, 1), key_style);
//         expected.set_style(Rect::new(43, 3, 4, 1), key_style);
//
//         assert_eq!(buf, expected);
//     }
// }
