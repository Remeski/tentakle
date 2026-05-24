mod logo;

pub mod beszel;
pub mod colors;
pub mod graph;

use ratatui::layout::Margin;
use ratatui::style::{Style, Stylize};
use ratatui::widgets::{Block, Widget};

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
