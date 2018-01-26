use state::FireState;

pub trait Layerable {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn features(&self) -> &Vec<Vec<Option<String>>>;

    fn get(&self, row_ix: usize, col_ix: usize) -> Option<String> {
        assert!(row_ix < self.rows());
        assert!(col_ix < self.cols());

        if row_ix >= self.features().len() {
            return None;
        }

        let row = &self.features()[row_ix];

        if col_ix >= row.len() {
            return None;
        }

        let elt = row[col_ix].clone();
        return elt;
    }
}

pub struct BasicLayer {
    rows: usize,
    cols: usize,
    features: Vec<Vec<Option<String>>>,
}

impl BasicLayer {
    pub fn create(rows: usize, cols: usize, features: Vec<Vec<Option<String>>>) -> Self {
        BasicLayer {
            rows,
            cols,
            features,
        }
    }
}

impl Layerable for BasicLayer {
    fn rows(&self) -> usize { self.rows }
    fn cols(&self) -> usize { self.cols }
    fn features(&self) -> &Vec<Vec<Option<String>>> {
        &self.features
    }
}

pub struct Compositor {
    pub rows: usize,
    pub cols: usize,
}

impl Compositor {
    /// composite produces a single field of bytes based on the presence of bytes at each index in each of the `layers`.
    /// Layers in `layers` should be ordered by ascending precedence (i.e., bottom layers first).
    pub fn composite(&self, layers: &[&Layerable]) -> Vec<Vec<u8>> {
        let mut field = vec![vec![" ".to_string(); self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                let comped: Option<String> = layers.into_iter().rev().fold(None, |acc, layer| acc.or(layer.get(i, j)));
                if let Some(string) = comped {
                    field[i][j] = string;
                }
            }
        }

        let byte_field: Vec<Vec<u8>> = field.into_iter().map(|row| {
            row.into_iter().flat_map(|s| s.into_bytes().into_iter()).collect()
        }).collect();

        byte_field
    }

    pub fn intermediate_composite(&self, layers: &[&Layerable]) -> IntermediateLayer {
        let mut field: Vec<Vec<Option<String>>> = vec![vec![None; self.cols]; self.rows];

        for i in 0..self.rows {
            for j in 0..self.cols {
                let comped: Option<String> = layers.into_iter().rev().fold(None, |acc, layer| acc.or(layer.get(i, j)));
                field[i][j] = comped;
            }
        }

        IntermediateLayer {
            rows: self.rows,
            cols: self.cols,
            features: field,
        }
    }
}

pub struct IntermediateLayer {
    rows: usize,
    cols: usize,
    features: Vec<Vec<Option<String>>>,
}

impl Layerable for IntermediateLayer {
    fn rows(&self) -> usize { self.rows }
    fn cols(&self) -> usize { self.cols }
    fn features(&self) -> &Vec<Vec<Option<String>>> { &self.features }
}

#[cfg(test)]
mod tests {
    use super::BasicLayer;
    use super::Layerable;

    #[test]
    pub fn test_get() {
        let features: Vec<Vec<Option<u8>>> = vec![
            b"bonjour".map(|&x| Some(x)).collect(),
            b"allo".map(|&x| Some(x)).collect(),
        ];

        let layer = BasicLayer {
            features,
            rows: 3,
            cols: 7,
        };

        assert_eq!(layer.get(0, 2), Some(b'n'));
        assert_eq!(layer.get(1, 0), Some(b'a'));
        assert_eq!(layer.get(2, 0), None);
        assert_eq!(layer.get(2, 5), None);
    }
}
