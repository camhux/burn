mod fire_state;
mod smoke_state;

use layers::{Compositor, IntermediateLayer};

pub use self::fire_state::{FireState, FireLayer};
use self::smoke_state::{SmokeState, SmokeLayer};

pub struct CombustionState {
    rows: usize,
    cols: usize,

    fire_state: FireState,
    smoke_state: SmokeState,
}

impl CombustionState {
    pub fn new(rows: usize, cols: usize) -> Self {
        let fire_state = FireState::new(rows, cols);
        let smoke_state = SmokeState::new(rows, cols);

        CombustionState {
            rows,
            cols,
            fire_state,
            smoke_state,
        }
    }

    pub fn get_next(&self) -> Self {
        let next_fire_state = self.fire_state.get_next();
        let next_smoke_state = self.smoke_state.get_next(&next_fire_state);

        CombustionState {
            rows: self.rows,
            cols: self.cols,
            fire_state: next_fire_state,
            smoke_state: next_smoke_state,
        }
    }

    // TODO: improve when `impl Trait` lands?
    pub fn as_layer(&self) -> IntermediateLayer {
        let compositor = Compositor { rows: self.rows, cols: self.cols };

        compositor.intermediate_composite(&[&self.fire_state.as_layer(), &self.smoke_state.as_layer()])
    }
}