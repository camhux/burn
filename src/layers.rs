use state::FireState;

pub trait Layerable {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn features(&self) -> &Vec<Vec<Option<u8>>>;

    fn get(&self, row_ix: usize, col_ix: usize) -> Option<u8> {
        assert!(row_ix < self.rows());
        assert!(col_ix < self.cols());

        if row_ix >= self.features().len() {
            return None;
        }

        let row = &self.features()[row_ix];

        if col_ix >= row.len() {
            return None;
        }

        let elt = row[col_ix];
        return elt;
    }
}

pub struct BasicLayer {
    rows: usize,
    cols: usize,
    byte_vec: Vec<Vec<Option<u8>>>,
}

impl Layerable for BasicLayer {
    fn rows(&self) -> usize { self.rows }
    fn cols(&self) -> usize { self.cols }
    fn features(&self) -> &Vec<Vec<Option<u8>>> { &self.byte_vec }
}

pub struct Compositor {
    pub rows: usize,
    pub cols: usize,
}

impl Compositor {
    /// composite produces a single field of bytes based on the presence of bytes at each index in each of the `layers`.
    /// Layers in `layers` should be ordered by ascending precedence (i.e., bottom layers first).
    pub fn composite(&self, layers: &[&Layerable]) -> Vec<Vec<u8>> {
        let mut field = vec![vec![b' '; self.cols]; self.rows];

        for (i, j) in (0..self.rows).zip(0..self.cols) {
            let comped: Option<u8> = layers.into_iter().rev().fold(None, |acc, layer| acc.or(layer.get(i, j)));
            if let Some(glyph) = comped {
                field[i][j] = glyph;
            }
        }

        field
    }
}

#[cfg(test)]
mod tests {
    use super::BasicLayer;
    use super::Layerable;

    #[test]
    pub fn test_get() {
        let byte_vec: Vec<Vec<Option<u8>>> = vec![
            b"bonjour".map(|&x| Some(x)).collect(),
            b"allo".map(|&x| Some(x)).collect(),
        ];

        let layer = BasicLayer {
            byte_vec,
            rows: 3,
            cols: 7,
        };

        assert_eq!(layer.get(0, 2), Some(b'n'));
        assert_eq!(layer.get(1, 0), Some(b'a'));
        assert_eq!(layer.get(2, 0), None);
        assert_eq!(layer.get(2, 5), None);
    }
}
