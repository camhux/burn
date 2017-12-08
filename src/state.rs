use rand;

const FIRE_GLYPHS: &[u8] = &[
    b'\x25', // %
    b'\x2A', // *
    b'\x2C', // ,
    b'W',
    b'Y',
    b'^',
];

pub struct FireLayer {
    pub rows: usize,
    pub cols: usize,
    pub features: Vec<Vec<Option<u8>>>,
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

    pub fn set_fire(&mut self) {
        let ix: usize = rand::random::<usize>() % self.cols;
        let glyph = FIRE_GLYPHS[rand::random::<usize>() % FIRE_GLYPHS.len()];

        let last_row = (&mut self.features).into_iter().last().unwrap();

        last_row[ix] = Some(glyph);
    }
}

#[cfg(test)]
mod tests {
    use super::FireLayer;

    #[test]
    fn test_set_fire() {
        let mut fire_layer = FireLayer::new(3, 3);

        fire_layer.set_fire();

        let last_row: &mut Vec<Option<u8>> = (&mut fire_layer.features).into_iter().last().unwrap();

        let fire_cell_count = last_row.into_iter().fold(0, |acc, &mut cell| match cell { Some(_) => acc + 1, None => acc });

        assert_eq!(fire_cell_count, 1);
    }
}
