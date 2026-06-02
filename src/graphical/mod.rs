use derive_setters::Setters;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::{Line, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

mod logo;

pub mod beszel;
pub mod colors;
pub mod pihole;
pub mod tubu;

use ratatui::layout::Margin;

pub use logo::Logo;

pub struct PagedBlock<'a> {
    num_pages: usize,
    page_idx: usize,
    title: &'a str,
}

impl<'a> PagedBlock<'a> {
    pub fn new(title: &'a str, page_idx: usize, num_pages: usize) -> Self {
        Self {
            page_idx,
            num_pages,
            title,
        }
    }
}

impl<'a> Widget for PagedBlock<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        Block::bordered()
            .title(self.title.fg(colors::FG_TEXT))
            .title_bottom(format!("{} / {}", self.page_idx + 1, self.num_pages))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::new().fg(colors::BORDER))
            .bg(colors::BG_CONT)
            .render(area, buf);
        Block::new()
            .style(Style::new().fg(colors::FG_TEXT))
            .bg(colors::BG_CONT)
            .render(area.inner(Margin::new(1, 1)), buf);
    }
}

#[allow(dead_code)]
#[derive(Debug, Default, Setters)]
pub struct Popup<'a> {
    #[setters(into)]
    title: Line<'a>,
    #[setters(into)]
    content: Text<'a>,
    border_style: Style,
    title_style: Style,
    style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // ensure that all cells under the popup are cleared to avoid leaking content
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(colors::PRIMARY));
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}
