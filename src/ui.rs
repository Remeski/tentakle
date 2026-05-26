use std::cell::RefCell;

use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect, Spacing},
    style::Style,
    widgets::{Block, Paragraph},
};

use crate::{
    app::App,
    graphical::{
        Logo, Popup,
        beszel::{ContainersState, LASState, SystemsState},
        colors,
    },
};

pub struct Message {
    pub title: String,
    pub content: String
}

pub struct UI {
    pub message: Option<Message>,
    pub systems_area: Rect,
    pub containers_area: Rect,
    pub las_area: Rect,
    pub systems_state: RefCell<SystemsState>,
    pub containers_state: RefCell<ContainersState>,
    pub las_state: RefCell<LASState>,
    beszel_layout: [Constraint; 3],
    focus: usize,
}

impl Default for UI {
    fn default() -> Self {
        Self {
            message: None,
            systems_area: Rect::default(),
            containers_area: Rect::default(),
            las_area: Rect::default(),
            systems_state: RefCell::default(),
            containers_state: RefCell::default(),
            las_state: RefCell::default(),
            beszel_layout: [
                Constraint::Length(4),
                Constraint::Fill(3),
                Constraint::Fill(2),
            ],
            focus: 1,
        }
    }
}

impl UI {
    pub fn render(app: &mut App, frame: &mut Frame<'_>) {
        let area = frame.area();
        let bg = Block::new().style(Style::new().bg(colors::BG));

        frame.render_widget(bg, frame.area());

        frame.render_widget(Logo::new(), Rect::new(4, 0, 1, 1));

        let main_area = Rect::new(4, 10, frame.area().width - 10, frame.area().height - 10);

        let [beszel_area, _] = main_area.layout(&Layout::horizontal([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ]));

        if let Some(beszel) = &app.beszel_handler {
            let [systems_area, containers_area, graph_area] = beszel_area.layout(
                &Layout::vertical(app.ui.beszel_layout)
                    .flex(Flex::Start)
                    .spacing(Spacing::Space(0)),
            );

            app.ui.systems_area = systems_area;
            app.ui.containers_area = containers_area;
            app.ui.las_area = graph_area;

            if let Some(widget) = beszel.systems_widget() {
                frame.render_stateful_widget(
                    widget,
                    systems_area,
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
                frame.render_stateful_widget(
                    widget,
                    containers_area,
                    &mut app.ui.containers_state.borrow_mut(),
                )
            } else {
                let loading = Paragraph::new("Loading...");
                frame.render_widget(
                    loading,
                    containers_area.centered_vertically(Constraint::Length(1)),
                );
            }

            if let Some(widget) = beszel.load_averages_widget() {
                frame.render_stateful_widget(
                    widget,
                    graph_area,
                    &mut app.ui.las_state.borrow_mut(),
                );
            }
        } else {
            let loading = Paragraph::new("Beszel not connected...").centered();
            frame.render_widget(
                loading,
                main_area.centered_vertically(Constraint::Length(1)),
            );
        }

        if let Some(message) = &app.ui.message {
            let popup_area = Rect {
                x: area.left() + area.width / 4,
                y: area.top() + area.height / 3,
                width: area.width / 2,
                height: area.height / 3,
            };
            let popup = Popup::default().content(message.content.clone()).title(message.title.clone());
            frame.render_widget(popup, popup_area);
        }
    }

    pub fn focus_containers(&mut self) -> bool {
        self.beszel_layout[1] = Constraint::Fill(3);
        self.beszel_layout[2] = Constraint::Fill(2);
        let changed = self.focus != 1;
        self.focus = 1;
        return changed;
    }

    pub fn focus_las(&mut self) -> bool {
        self.beszel_layout[1] = Constraint::Fill(2);
        self.beszel_layout[2] = Constraint::Fill(3);
        let changed = self.focus != 2;
        self.focus = 2;
        return changed;
    }
}
