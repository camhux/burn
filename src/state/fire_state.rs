use rand;

const FIRE_GLYPHS: &[u8] = &[
    b'\x25', // %
    b'\x2A', // *
    b'\x2C', // ,
    b'W',
    b'Y',
    b'^',
];

type FireCell = Option<u8>;

struct Neighbors {
    row: usize,
    col: usize,

    top: FireCell,
    right: FireCell,
    bottom: FireCell,
    left: FireCell,
}

impl Neighbors {
    fn n_fires(&self) -> usize {
        let mut count = 0;

        for neighbor in [self.top, self.right, self.bottom, self.left].into_iter() {
            if neighbor.is_some() { count += 1 }
        }

        count
    }

    fn fire_in_neighborhood(&self) -> bool {
        self.n_fires() > 0
    }
}

#[derive(Clone)]
pub struct FireState {
    pub rows: usize,
    pub cols: usize,
    pub features: Vec<Vec<FireCell>>,
    pub n_fires: usize,
}

impl FireState {
    // (camhux): (cols, rows) order matches termion::get_terminal_size return type
    pub fn new(cols: usize, rows: usize) -> Self {
        let features = vec![vec![None; cols]; rows];

        return Self {
            rows,
            cols,
            features,
            n_fires: 0,
        };
    }

    fn set_cell_fire(&mut self, row: usize, col: usize) {
        // TODO: replace with a call to thread-local RNG's `gen_range` method. Probably more efficient
        let glyph = FIRE_GLYPHS[rand::random::<usize>() % FIRE_GLYPHS.len()];

        self.features[row][col] = Some(glyph);
        self.n_fires += 1;
    }

    pub fn start_fire(&mut self) {
        let row_ix = self.rows - 1;
        // TODO: replace with a call to thread-local RNG's `gen_range` method. Probably more efficient
        let col_ix: usize = rand::random::<usize>() % self.cols;

        self.set_cell_fire(row_ix, col_ix);
    }

    pub fn get_next(&self) -> Self {
        let mut next = self.clone();

        for i in 0..self.rows {
            for j in 0..self.cols {
                let neighbors = self.get_neighbors(i, j);

                let mut tries = neighbors.n_fires();
                let mut should_combust = false;

                while tries > 0 && !should_combust {
                    tries -= 1;
                    should_combust = rand::random();
                }

                if should_combust { next.set_cell_fire(i, j) }
            }
        }

        next
    }

    fn get_neighbors(&self, row: usize, col: usize) -> Neighbors {
        Neighbors {
            row,
            col,

            top: if row == 0 { None } else { self.features[row - 1][col] },
            right: if col + 1 == self.cols { None } else { self.features[row][col + 1] },
            bottom: if row + 1 == self.rows { None } else { self.features[row + 1][col] },
            left: if col == 0 { None } else { self.features[row][col - 1] },
        }
    }

    pub fn is_saturated(&self) -> bool {
        (self.n_fires as f64 / (self.rows * self.cols) as f64) > 0.9f64
    }
}

#[cfg(test)]
mod tests {
    use super::FireState;
    use super::FireCell;

    #[test]
    fn test_start_fire() {
        let mut fire_state = FireState::new(3, 3);

        fire_state.start_fire();

        let last_row: &mut Vec<FireCell> = (&mut fire_state.features).into_iter().last().unwrap();

        let fire_cell_count = last_row.into_iter().fold(0, |acc, &mut cell| match cell { Some(_) => acc + 1, None => acc });

        assert_eq!(fire_cell_count, 1);
    }
}
