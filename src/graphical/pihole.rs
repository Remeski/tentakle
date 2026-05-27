use ratatui::{
    layout::{Constraint, Margin},
    style::{Style, Stylize},
    widgets::{Block, Paragraph, StatefulWidget, Widget},
};

use crate::{
    app::App,
    event::{AppEvent, PiholeEvent},
    graphical::colors,
};

pub struct PiholeStatus {
    blocking: bool,
}

impl PiholeStatus {
    pub fn new(blocking: bool) -> Self {
        Self { blocking }
    }
}

#[derive(Default)]
pub struct PiholeStatusState {}

impl PiholeStatusState {
    pub fn click(&mut self, app: &App) {
        if let Some(handler) = &app.pihole_handler {
            if let Some(blocking) = handler.blocking {
                let event = if blocking {
                    PiholeEvent::BlockingOff
                } else {
                    PiholeEvent::BlockingOn
                };
                app.event_handler
                    .send(AppEvent::Pihole(event))
                    .expect("unable to send AppEvent");
                app.event_handler
                    .send(AppEvent::Pihole(PiholeEvent::Update))
                    .expect("unable to send AppEvent");
            }
        }
    }
}

impl StatefulWidget for PiholeStatus {
    type State = PiholeStatusState;

    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        _: &mut Self::State,
    ) {
        let bg_color = if self.blocking {
            colors::SUCCESS
        } else {
            colors::ERROR
        };

        Block::bordered().border_type(ratatui::widgets::BorderType::Rounded).title("PIHOLE blocking".fg(colors::FG_TEXT)).border_style(Style::new().fg(colors::BORDER)).render(area, buf);
        let text = if self.blocking { "ON" } else { "OFF" };
        let inner_area = area.inner(Margin::new(1, 1));
        Block::new().bg(bg_color).render(inner_area, buf);
        let center = inner_area.centered(Constraint::Length(text.len() as u16), Constraint::Length(1));
        Paragraph::new(text).render(center, buf);
    }
}
