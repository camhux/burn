use rand::{self, Rng};
use rand::distributions::IndependentSample;
use layers::Layerable;
use termion::color;

const FIRE_GLYPHS: &[char] = &[
    '\x25', // %
    '\x2A', // *
    '\x2C', // ,
    'W',
    'Y',
    '^',
];

const ASH_GLYPHS: &[char] = &[
    '.',
    ' ',
    ' ',
    ' ',
    ' ',
    ' ',
    ' ',
];

const FIRE_COLORS: &[color::Rgb] = &[
    color::Rgb(232, 81, 44),
    color::Rgb(198, 46, 7),
    color::Rgb(247, 39, 67),
    color::Rgb(255, 134, 5),
    color::Rgb(255, 72, 48),
];

#[derive(Copy, Clone)]
pub enum FireCell {
    Unlit,
    Lit { ttl: usize },
    Extinguished { glyph: char },
}

impl From<FireCell> for Option<String> {
    fn from(fire_cell: FireCell) -> Self {
        use self::FireCell::{Unlit, Lit, Extinguished};

        let mut rng = rand::thread_rng();

        match fire_cell {
            Unlit => None,
            Lit {..} => {
                let glyph = *(rng.choose(FIRE_GLYPHS).unwrap());
                let fire_color = *(rng.choose(FIRE_COLORS).unwrap());

                Some(format!("{}{}{}", color::Fg(fire_color), glyph, color::Fg(color::Reset)))
            },
            Extinguished { glyph } => Some(format!("{}{}{}", color::Fg(color::Rgb(100, 100, 100)), glyph, color::Fg(color::Reset))),
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
    pub features: Vec<Vec<FireCell>>, // needs to be public for calculating smoke layer. maybe this can be refined
    n_fires: usize,
    ttl_range: rand::distributions::Range<usize>,
}

impl FireState {
    pub fn new(rows: usize, cols: usize) -> Self {
        let features = vec![vec![FireCell::Unlit; cols]; rows];

        Self {
            rows,
            cols,
            features,
            n_fires: 0,
            ttl_range: rand::distributions::Range::new(3, 26),
        }
    }

    fn set_cell_fire(&mut self, row: usize, col: usize) {
        // TODO: tweak/iterate on ttl, possibly extract into constant for maintenance
        let ttl = self.ttl_range.ind_sample(&mut rand::thread_rng());
        self.features[row][col] = FireCell::Lit { ttl };
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
                            should_combust = rand::thread_rng().gen_weighted_bool(7);
                        }

                        if should_combust { next.set_cell_fire(i, j) }
                    },
                    Lit { ttl } => {
                        if ttl < 1 {
                            let glyph = *(rand::thread_rng().choose(ASH_GLYPHS).unwrap());
                            next.features[i][j] = Extinguished { glyph };
                        } else {
                            next.features[i][j] = Lit { ttl: ttl - 1 };
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
            // no consequence for modeling 'out-of-bounds' neighbors as `Unlit`;
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
    features: Vec<Vec<Option<String>>>,
}

impl Layerable for FireLayer {
    fn rows(&self) -> usize { self.rows }
    fn cols(&self) -> usize { self.cols }
    fn features(&self) -> &Vec<Vec<Option<String>>> { &self.features }
}

impl<'a> From<&'a FireState> for FireLayer {
    fn from(fire_state: &'a FireState) -> Self {
        let features: Vec<Vec<Option<String>>> = (&fire_state.features).into_iter()
            .map(|row| {
                row.into_iter().map(|&cell| cell.into()).collect::<Vec<Option<String>>>()
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
