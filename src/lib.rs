use std::{cmp, fmt::Display};

type Value = u8;
type Cells = Vec<Vec<Value>>;

const LIVE: Value = 1;
const DEAD: Value = 0;

pub struct LifeGame {
    pub width: usize,
    pub height: usize,
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
            width,
            height,
            cells: (0..height)
                .map(|_| (0..width).map(|_| DEAD).collect())
                .collect(),
        }
    }

    pub fn from(input: &[Vec<Value>]) -> Self {
        let height = input.len();
        let width = input.iter().map(|row| row.len()).min().unwrap();
        let cells = input
            .iter()
            .map(|row| row.iter().take(width).map(|&cell| cell).collect())
            .collect();
        LifeGame {
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

    pub fn get_cells(&self) -> Vec<Vec<bool>> {
        self.cells
            .iter()
            .map(|row| row.iter().map(|&c| c == LIVE).collect())
            .collect()
    }

    pub fn next(&mut self) -> Option<()> {
        let prev = &self.cells;
        let next = self.to_next_generation_cells();
        let same = Self::is_same_cells(prev, &next);
        self.cells = next;
        if same {
            None
        } else {
            Some(())
        }
    }

    fn to_next_generation_cells(&self) -> Cells {
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

    fn count_alives(&self, x: usize, y: usize) -> u8 {
        let ys = if y == 0 { 0 } else { y - 1 }..=cmp::min(y + 1, self.height - 1);
        let xs = if x == 0 { 0 } else { x - 1 }..=cmp::min(x + 1, self.width - 1);
        ys.flat_map(|y| xs.clone().map(move |x| (x, y)))
            .filter(|&p| p != (x, y))
            .map(|(x, y)| if self.cells[y][x] == LIVE { 1 } else { 0 })
            .sum()
    }

    fn is_same_cells(a: &Cells, b: &Cells) -> bool {
        a.len() == b.len()
            && a.iter()
                .zip(b)
                .all(|(r1, r2)| r1.len() == r2.len() && r1.iter().zip(r2).all(|(c1, c2)| c1 == c2))
    }
}
