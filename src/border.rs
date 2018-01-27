use layers::Layerable;

pub struct Border {
    rows: usize,
    cols: usize,
    features: Vec<Vec<Option<String>>>
}

impl Border {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut features: Vec<Vec<Option<String>>> =
            vec![vec![None; cols]; rows];

        for cell in (&mut features[0]).into_iter() {
            *cell = Some("━".into());
        }
        for cell in (&mut features[rows - 1]).into_iter() {
            *cell = Some("━".into());
        }
        for cell in (&mut features).into_iter().map(|row| &mut row[0]) {
            *cell = Some("┃".into());
        }
        for cell in (&mut features).into_iter().map(|row| &mut row[cols - 1]) {
            *cell = Some("┃".into());
        }

        features[0][0] = Some("┏".into());
        features[0][cols - 1] = Some("┓".into());
        features[rows - 1][0] = Some("┗".into());
        features[rows - 1][cols - 1] = Some("┛".into());

        Self {
            rows,
            cols,
            features
        }
    }
}

impl Layerable for Border {
    fn rows(&self) -> usize { self.rows }
    fn cols(&self) -> usize { self.cols }
    fn features(&self) -> &Vec<Vec<Option<String>>> {
        &self.features
    }
}