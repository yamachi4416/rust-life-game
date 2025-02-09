use std::{cmp, fmt::Display};

type Value = u8;
type Cells = Vec<Vec<Value>>;

const LIVE: Value = 1;
const DEAD: Value = 0;

pub struct LifeGame {
    name: String,
    width: usize,
    height: usize,
    cells: Cells,
}

impl Display for LifeGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for &cell in row {
                write!(f, "{}", if cell == LIVE { "+" } else { "." })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl LifeGame {
    pub fn new(width: usize, height: usize) -> Self {
        LifeGame {
            name: String::new(),
            width,
            height,
            cells: (0..height)
                .map(|_| (0..width).map(|_| DEAD).collect())
                .collect(),
        }
    }

    pub fn from(name: &str, input: &[Vec<Value>]) -> Self {
        let height = input.len();
        let width = input.iter().map(Vec::len).min().unwrap();
        let cells = input.iter().map(|row| row[..width].to_vec()).collect();
        LifeGame {
            name: name.into(),
            width,
            height,
            cells,
        }
    }

    pub fn set_alives(&mut self, points: &[(usize, usize)]) {
        for &(x, y) in points {
            self.cells[y][x] = LIVE;
        }
    }

    pub fn cells_iter(&self) -> impl Iterator<Item = impl Iterator<Item = bool> + '_> + '_ {
        self.cells.iter().map(|row| row.iter().map(|&c| c == LIVE))
    }

    pub fn next(&mut self) -> Option<()> {
        let next = self.to_next_cells();
        if self.cells == next {
            None
        } else {
            self.cells = next;
            Some(())
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn width(&self) -> u16 {
        self.width as u16
    }

    pub fn height(&self) -> u16 {
        self.height as u16
    }

    fn to_next_cells(&self) -> Cells {
        self.cells
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, &cell)| self.to_next_cell(cell, x, y))
                    .collect()
            })
            .collect()
    }

    fn to_next_cell(&self, cell: Value, x: usize, y: usize) -> Value {
        match (cell, self.count_alives(x, y)) {
            (DEAD, 3) | (LIVE, 2) | (LIVE, 3) => LIVE,
            _ => DEAD,
        }
    }

    fn count_alives(&self, x: usize, y: usize) -> usize {
        let ys = if y == 0 { 0 } else { y - 1 }..=cmp::min(y + 1, self.height - 1);
        let xs = if x == 0 { 0 } else { x - 1 }..=cmp::min(x + 1, self.width - 1);
        ys.flat_map(|y| xs.clone().map(move |x| (x, y)))
            .filter(|&p| p != (x, y))
            .filter(|&(x, y)| self.cells[y][x] == LIVE)
            .count()
    }
}
