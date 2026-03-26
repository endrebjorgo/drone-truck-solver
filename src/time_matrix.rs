#[derive(Debug, Default)]
pub struct TimeMatrix {
    pub rows: usize,
    pub cols: usize,
    pub items: Vec<u32>,
}

impl TimeMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            items: vec![0; rows*cols],
        }
    }

    pub fn get(&self, from: usize, to: usize) -> u32 {
        assert!(from < self.rows);
        assert!(to < self.cols);
        return self.items[from * self.cols + to];
    }

    pub fn set(&mut self, from: usize, to: usize, value: u32) {
        assert!(from < self.rows);
        assert!(to < self.cols);
        self.items[from * self.cols + to] = value;
    }
}

