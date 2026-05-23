use ratatui::widgets::{Chart, Dataset, Widget};

pub struct Graph<'a> {
    dataset: Dataset<'a>
}

impl<'a> Graph<'a> {
    pub fn new(dataset: Dataset<'a>) -> Self {
        Self {
            dataset
        }
    }
}

impl<'a> Widget for Graph<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
        where
            Self: Sized {
                Chart::new(vec![self.dataset]).render(area, buf);
    }
}
