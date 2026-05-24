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

    pub fn cols(&self) -> Vec<Vec<Rect>> {
        (0..self.layout[0].iter().len())
            .map(|i| {
                self.layout
                    .iter()
                    .map(|v| v.get(i).unwrap().clone())
                    .collect()
            })
            .collect()
    }

    pub fn rows(&self) -> Vec<Vec<Rect>> {
        self.layout.clone()
    }

    pub fn single_dimensional(&self) -> Vec<Rect> {
        self.layout.iter().map(|e| e.clone()).flatten().collect()
    }
}

pub trait InsideRect {
    fn inside(&self, c: usize, r: usize) -> bool;
}

impl InsideRect for Rect {
    fn inside(&self, c: usize, r: usize) -> bool {
        let (x, x_top, y, y_top) = (
            self.x as usize,
            (self.x + self.width) as usize,
            self.y as usize,
            (self.y + self.height) as usize,
        );
        c >= x && c <= x_top && r >= y && r <= y_top
    }
}

#[cfg(test)]
mod tests {
    use ratatui::layout::{Constraint, Rect};

    use crate::utils::{Grid, InsideRect};

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

    #[test]
    fn test_rows_and_cols() {
        let grid = Grid::new(
            Rect::new(0, 0, 10, 10),
            vec![Constraint::Length(1), Constraint::Length(1)],
            vec![Constraint::Length(1), Constraint::Length(1)],
        );

        // for a in grid.layout {
        //     dbg!(a);
        // }

        for a in grid.cols() {
            dbg!(a);
        }
    }

    #[test]
    fn test_inside_rect() {
        let r = Rect::new(5, 5, 5, 5);
        assert!(!r.inside(3, 3));
        assert!(r.inside(7, 9));
        assert!(r.inside(5, 5));
        assert!(r.inside(5, 10));
        assert!(!r.inside(11, 10));
        assert!(r.inside(10, 10));
        assert!(!r.inside(15, 18));
    }
}
