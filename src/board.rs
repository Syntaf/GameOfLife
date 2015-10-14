pub struct Board {
    map: Vec<Vec<u8>>,
    rows: usize,
    cols: usize
}

impl Board {
    pub fn new(r: usize, c: usize) -> Board {
        Board { map: vec![vec![0; c]; r], rows: r, cols: c }
    }
}
