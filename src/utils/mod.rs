use ratatui::layout::{Constraint, Layout, Rect};

pub struct Grid {
    pub cols: usize,
    pub rows: usize,
    pub layout: Vec<Vec<Rect>>,
}

impl Grid {
    pub fn new(
        area: Rect,
        cols_constraints: Vec<Constraint>,
        rows_constraints: Vec<Constraint>,
    ) -> Self {
        let cols = cols_constraints.len();
        let rows = rows_constraints.len();
        let layout_rows = Layout::vertical(rows_constraints).split(area);

        let layout: Vec<Vec<Rect>> = layout_rows
            .iter()
            .map(|row| {
                Layout::horizontal(cols_constraints.clone())
                    .split(row.clone())
                    .to_vec()
            })
            .collect();

        Self { cols, rows, layout }
    }

    pub fn single_dimensional(&self) -> Vec<Rect> {
        self.layout.iter().map(|e| e.clone()).flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::{Constraint, Rect};

    use crate::utils::Grid;

    #[test]
    fn test_single_dimensional() {
        let grid = Grid::new(
            Rect::new(0, 0, 10, 10),
            vec![Constraint::Length(1), Constraint::Length(1)],
            vec![Constraint::Length(1), Constraint::Length(1)],
        );

        for a in grid.single_dimensional() {
            dbg!(a);
        }
    }
}

