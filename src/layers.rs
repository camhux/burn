use state::FireState;

pub trait Layerable {
    type E: Copy + Into<Option<u8>>;

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn features(&self) -> &Vec<Vec<Self::E>>;

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
        return elt.into();
    }
}

pub struct BasicLayer {
    rows: usize,
    cols: usize,
    byte_vec: Vec<Vec<Option<u8>>>,
}

impl Layerable for BasicLayer {
    type E = Option<u8>;

    fn rows(&self) -> usize { self.rows }
    fn cols(&self) -> usize { self.cols }
    fn features(&self) -> &Vec<Vec<Self::E>> { &self.byte_vec }
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