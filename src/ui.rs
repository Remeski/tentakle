use std::cell::RefCell;

use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, Paragraph},
};

use crate::{
    app::App,
    graphical::{Logo, beszel::SystemsState, colors},
};

#[derive(Default)]
pub struct UI {
    pub systems_area: Rect,
    pub containers_area: Rect,
    pub systems_state: RefCell<SystemsState>,
}

impl UI {
    pub fn render(app: &mut App, frame: &mut Frame<'_>) {
        let bg = Block::new().style(Style::new().bg(colors::BG));

        frame.render_widget(bg, frame.area());

        frame.render_widget(Logo::new(), Rect::new(4, 1, 1, 1));

        let main_area = Rect::new(5, 12, frame.area().width - 10, frame.area().height - 12);

        // frame.render_widget(Block::bordered(), main_area);

        if let Some(beszel) = &app.beszel_handler {
            let systems_area = Rect::new(main_area.left(), main_area.top(), 70, 3);
            let containers_area = Rect::new(main_area.left(), main_area.top() + 3 + 1, 70, 3);

            app.ui.systems_area = systems_area;
            app.ui.containers_area = containers_area;

            if let Some(widget) = beszel.systems_widget() {
                frame.render_stateful_widget(
                    widget,
                    Rect::new(main_area.left(), main_area.top(), 70, 3),
                    &mut app.ui.systems_state.borrow_mut(),
                );
            } else {
                let loading = Paragraph::new("Loading...");
                frame.render_widget(
                    loading,
                    systems_area.centered_vertically(Constraint::Length(1)),
                );
            }

            if let Some(widget) = beszel.containers_widget() {
                frame.render_widget(
                    widget,
                    Rect::new(main_area.left(), main_area.top() + 5, 70, 10),
                )
            } else {
                let loading = Paragraph::new("Loading...");
                frame.render_widget(
                    loading,
                    containers_area.centered_vertically(Constraint::Length(1)),
                );
            }
        } else {
            let loading = Paragraph::new("Beszel not connected...").centered();
            frame.render_widget(
                loading,
                main_area.centered_vertically(Constraint::Length(1)),
            );
        }
    }
}
