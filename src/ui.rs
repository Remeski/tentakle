use ratatui::{Frame, layout::Rect, style::Style, widgets::Block};

use crate::{
    app::App, graphical::{Logo, colors}
};

#[derive(Default, Clone)]
pub struct UI {
    pub systems_area: Rect
}

impl UI {
    pub fn render(app: &mut App, frame: &mut Frame<'_>) {
        let bg = Block::new().style(Style::new().bg(colors::BG));

        frame.render_widget(bg, frame.area());

        frame.render_widget(Logo::new(), Rect::new(4, 1, 1, 1));

        let main_area = Rect::new(5, 12, frame.area().width - 10, frame.area().height - 12);

        // frame.render_widget(Block::bordered(), main_area);

        if let Some(beszel) = &app.beszel_handler {
            if let Some(widget) = beszel.systems_widget() {
                let systems_area = Rect::new(main_area.left(), main_area.top(), 70, 3);
                frame.render_stateful_widget(
                    widget,
                    Rect::new(main_area.left(), main_area.top(), 70, 3),
                    &mut app.systems_state.borrow_mut(),
                );
                app.ui.systems_area = systems_area;
            }

            if let Some(widget) = beszel.containers_widget() {
                frame.render_widget(
                    widget,
                    Rect::new(main_area.left(), main_area.top() + 5, 70, 10),
                )
            }
        }

    }

}
