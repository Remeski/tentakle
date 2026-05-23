use ratatui::{
    style::{Color, Style},
    widgets::Widget,
};

pub struct Logo {
    logo: String,
}

const LOGO: &str = "
 ███████████                      █████              █████      ████          
░█░░░███░░░█                     ░░███              ░░███      ░░███          
░   ░███  ░   ██████  ████████   ███████    ██████   ░███ █████ ░███   ██████ 
    ░███     ███░░███░░███░░███ ░░░███░    ░░░░░███  ░███░░███  ░███  ███░░███
    ░███    ░███████  ░███ ░███   ░███      ███████  ░██████░   ░███ ░███████ 
    ░███    ░███░░░   ░███ ░███   ░███ ███ ███░░███  ░███░░███  ░███ ░███░░░  
    █████   ░░██████  ████ █████  ░░█████ ░░████████ ████ █████ █████░░██████ 
   ░░░░░     ░░░░░░  ░░░░ ░░░░░    ░░░░░   ░░░░░░░░ ░░░░ ░░░░░ ░░░░░  ░░░░░░  ";

impl Widget for Logo {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let style = Style::new().fg(Color::Rgb(150, 0, 0));
        for (i, line) in self.logo.lines().enumerate() {
            buf.set_string(area.left(), area.top() + i as u16, line, style);
        }
    }
}

impl Logo {
    pub fn new() -> Self {
        Self {
            logo: String::from(LOGO),
        }
    }
}
