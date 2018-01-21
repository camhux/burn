use layers::Layerable;
use termion::color;
use rand::{self, Rng};

use super::fire_state::FireState;

const SMOKE_GLYPHS: &[char] = &[
    '"',
    '&',
    '@',
    '?',
];

const SMOKE_COLORS: &[color::Rgb] = &[
    color::Rgb(79, 79, 79),
    color::Rgb(140, 133, 133),
    color::Rgb(178, 173, 173),
    color::Rgb(140, 120, 120),
];

// TODO: maybe just alias Option<usize> to reap method impls?
#[derive(Copy, Clone)]
enum SmokeCell {
    Clear,
    Smoky {
        volume: usize,
    },
}

#[derive(Clone)]
pub struct SmokeState {
    rows: usize,
    cols: usize,
    features: Vec<Vec<SmokeCell>>,
}

impl SmokeState {
    pub fn new(rows: usize, cols: usize) -> Self {
        let features = vec![vec![SmokeCell::Clear; cols]; rows];

        Self {
            rows,
            cols,
            features,
        }
    }

    fn gen_smoke_movement(&self, row: usize, col: usize) -> Option<(usize, usize)> {
        let mut rng = rand::thread_rng();

        let row_delta = rng.gen_range::<isize>(-7, -3);
        let col_delta = rng.gen_range::<isize>(-2, 3);

        let new_row = row as isize + row_delta;
        let new_col = col as isize + col_delta;

        if new_row < 0 ||
           new_col < 0 ||
           new_col as usize >= self.cols {
               return None;
           }

        Some((new_row as usize, new_col as usize))
    }

    fn place_smoke(&mut self, row: usize, col: usize) {
        let features = &mut self.features;
        let next = match features[row][col] {
            SmokeCell::Clear => SmokeCell::Smoky { volume: 1 },
            SmokeCell::Smoky { volume: vol } => SmokeCell::Smoky { volume: vol + 1},
        };

        features[row][col] = next;
    }

    fn clear_smoke(&mut self, row: usize, col: usize) {
        self.features[row][col] = SmokeCell::Clear;
    }

    pub fn get_next(&self, fire_state: &FireState) -> Self {
        use super::fire_state::FireCell;

        let mut rng = rand::thread_rng();

        let mut next = self.clone();

        // update positions of extant smokebits
        for (i, row) in (&self.features).into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                if let SmokeCell::Smoky { volume: vol } = *cell {
                    for _ in 0..vol {
                        if let Some((new_row, new_col)) = self.gen_smoke_movement(i, j) {
                            next.place_smoke(new_row, new_col);
                        }
                    }
                    next.clear_smoke(i, j);
                }
            }
        }

        // spawn new smokebits based on underlying fire layer
        for (i, row) in (&fire_state.features).into_iter().enumerate() {
            for (j, cell) in row.into_iter().enumerate() {
                match (*cell, rng.gen_weighted_bool(3)) {
                    (FireCell::Lit { .. }, true) => next.place_smoke(i, j),
                    _ => {},
                }
            }
        }

        next
    }
}
