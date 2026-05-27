use asciigraphix_core::Display;
use asciigraphix_core::shapes::{Point, Shape};
use ratatui::layout::Margin;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, StatefulWidget, Widget};

use crate::graphical::colors;

/// stands for Totally Useless But Useful -widget (i changed it to necessary :) )
pub struct Tubu {}

impl Tubu {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct TubuState {
    pub shape: Shape,
}

impl Default for TubuState {
    fn default() -> Self {
        Self {
            shape: Shape::generate_cube(Point(0.0, 0.0, 0.0), 7.0),
            // shape: Shape::generate_ring(7.0, Point(0.0, 0.0, 0.0))
        }
    }
}

impl StatefulWidget for Tubu {
    type State = TubuState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        Block::bordered().title("Totally useless but necessary -widget".fg(colors::FG_TEXT)).border_type(ratatui::widgets::BorderType::Rounded).border_style(Style::new().fg(colors::BORDER)).render(area, buf);
        let area = area.inner(Margin::new(1, 1));
        let mut display = Display::new(
            area.width as usize,
            area.height as usize,
            Point(0.0, -11.0, -0.7),
            Point(0.0, 1.0, 0.0),
            10.0,
        );

        for (i, (depth, _)) in display.render(&state.shape).iter().enumerate() {
            let x = i % area.width as usize;
            let y = (i - x) / area.width as usize;

            let str = ".";

            // linear interpolation from 0.5 - 1.0 as depth 30.0 - 100.0
            let coef = (-0.5 / 70.0 * (depth - 30.0) + 1.0).min(1.0).max(0.5);
            let c = (240.9999 * coef) as u8;
            let mut color = Color::Rgb(c, 0, 0);
            if *depth == 0.0 {
                color = colors::BG_CONT;
            }
            let mut style = Style::new().fg(color).bg(colors::BG_CONT);
            if *depth < 60.0 {
                style = style.bold();
            }
            buf.set_string(area.x + x as u16, area.y + y as u16, str, style);
        }
    }
}
