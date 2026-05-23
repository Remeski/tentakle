use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Widget},
};

use crate::{
    app::App,
    components::{self, Logo},
    main, trace_dbg,
};

pub fn render(app: &App, frame: &mut Frame<'_>) {
    let bg = Block::new().style(Style::new().bg(Color::Rgb(15, 15, 15)));

    frame.render_widget(bg, frame.area());

    frame.render_widget(Logo::new(), Rect::new(4, 1, 1, 1));

    let main_area = Rect::new(5, 12, frame.area().width - 10, frame.area().height - 12);
    trace_dbg!(main_area);

    // frame.render_widget(Block::bordered(), main_area);

    if let Some(beszel) = &app.beszel_handler {
        if let Some(widget) = beszel.systems_widget() {
            frame.render_widget(
                widget,
                Rect::new(main_area.left(), main_area.top(), 50, 10),
            )
        }
    }
}
