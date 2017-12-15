use rand;
use layers::Layerable;

const FIRE_GLYPHS: &[u8] = &[
    b'\x25', // %
    b'\x2A', // *
    b'\x2C', // ,
    b'W',
    b'Y',
    b'^',
];

const ASH_GLYPHS: &[u8] = &[
    b'.',
    b' ',
    b' ',
    b' ',
    b' ',
    b' ',
    b' ',
];

#[derive(Copy, Clone)]
enum FireCell {
    Unlit,
    Lit { glyph: u8, ttl: usize },
    Extinguished { glyph: u8 },
}

impl From<FireCell> for Option<u8> {
    fn from(fire_cell: FireCell) -> Self {
        use self::FireCell::{Unlit, Lit, Extinguished};

        match fire_cell {
            Unlit => None,
            Lit { glyph, .. } | Extinguished { glyph } => Some(glyph),
        }
    }
}

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
        [self.top, self.right, self.bottom, self.left].into_iter().filter(|&&cell| if let FireCell::Lit {..} = cell { true } else { false }).count()
    }

    fn fire_in_neighborhood(&self) -> bool {
        self.n_fires() > 0
    }
}

#[derive(Clone)]
pub struct FireState {
    rows: usize,
    cols: usize,
    features: Vec<Vec<FireCell>>,
    n_fires: usize,
}

impl FireState {
    pub fn new(rows: usize, cols: usize) -> Self {
        let features = vec![vec![FireCell::Unlit; cols]; rows];

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

        // TODO: tweak/iterate on ttl, possibly extract into constant for maintenance
        self.features[row][col] = FireCell::Lit { glyph, ttl: 10 };
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

        for (i, row) in (&self.features).into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                use self::FireCell::{Unlit, Lit, Extinguished};

                // TODO: return *next state for cell* from this match and assign to `next[i][j]` only once instead of burying the mutations in branches
                match *cell {
                    // give cell the opportunity to combust; may not due to randomness
                    Unlit => {
                        let neighbors = self.get_neighbors(i, j);

                        let mut tries = neighbors.n_fires();
                        let mut should_combust = false;

                        while tries > 0 && !should_combust {
                            tries -= 1;
                            should_combust = rand::random();
                        }

                        if should_combust { next.set_cell_fire(i, j) }
                    },
                    Lit { glyph, ttl } => {
                        if ttl < 1 {
                            let glyph = ASH_GLYPHS[rand::random::<usize>() % ASH_GLYPHS.len()];
                            next.features[i][j] = Extinguished { glyph };
                        } else {
                            next.features[i][j] = Lit { glyph, ttl: ttl - 1 };
                        }
                    },
                    _ => {},
                }
            }
        }

        next
    }

    fn get_neighbors(&self, row: usize, col: usize) -> Neighbors {
        use self::FireCell::Unlit;

        Neighbors {
            row,
            col,
            // no consequence for modeling "out-of-bounds" neighbors as `Unlit`;
            // we just need to know if there are any real neighbors on fire
            top: if row == 0 { Unlit } else { self.features[row - 1][col] },
            right: if col + 1 == self.cols { Unlit } else { self.features[row][col + 1] },
            bottom: if row + 1 == self.rows { Unlit } else { self.features[row + 1][col] },
            left: if col == 0 { Unlit } else { self.features[row][col - 1] },
        }
    }

    pub fn is_saturated(&self) -> bool {
        (self.n_fires as f64 / (self.rows * self.cols) as f64) > 0.99f64
    }

    pub fn as_layer(&self) -> FireLayer {
        self.into()
    }
}

pub struct FireLayer {
    rows: usize,
    cols: usize,
    features: Vec<Vec<Option<u8>>>
}

impl Layerable for FireLayer {
    fn rows(&self) -> usize { self.rows }
    fn cols(&self) -> usize { self.cols }
    fn features(&self) -> &Vec<Vec<Option<u8>>> { &self.features }
}

impl<'a> From<&'a FireState> for FireLayer {
    fn from(fire_state: &'a FireState) -> Self {
        let features: Vec<Vec<Option<u8>>> = (&fire_state.features).into_iter()
            .map(|row| {
                row.into_iter().map(|&cell| cell.into()).collect::<Vec<Option<u8>>>()
            })
            .collect::<Vec<_>>();

        FireLayer {
            features,
            rows: fire_state.rows,
            cols: fire_state.cols,
        }
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
