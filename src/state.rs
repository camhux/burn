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
pub struct FireLayer {
    pub rows: usize,
    pub cols: usize,
    pub features: Vec<Vec<FireCell>>,
}

impl FireLayer {
    pub fn new(rows: usize, cols: usize) -> Self {
        let features = vec![vec![None; cols]; rows];

        return FireLayer {
            rows,
            cols,
            features,
        };
    }

    fn set_cell_fire(&mut self, row: usize, col: usize) {
        // TODO: replace with a call to thread-local RNG's `gen_range` method. Probably more efficient
        let glyph = FIRE_GLYPHS[rand::random::<usize>() % FIRE_GLYPHS.len()];

        self.features[row][col] = Some(glyph);
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
}

#[cfg(test)]
mod tests {
    use super::FireLayer;
    use super::FireCell;

    #[test]
    fn test_start_fire() {
        let mut fire_layer = FireLayer::new(3, 3);

        fire_layer.start_fire();

        let last_row: &mut Vec<FireCell> = (&mut fire_layer.features).into_iter().last().unwrap();

        let fire_cell_count = last_row.into_iter().fold(0, |acc, &mut cell| match cell { Some(_) => acc + 1, None => acc });

        assert_eq!(fire_cell_count, 1);
    }
}
