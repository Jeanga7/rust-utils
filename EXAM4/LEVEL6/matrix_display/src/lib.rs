#[derive(Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        Matrix(slice.iter().map(|&row| row.to_vec()).collect())
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.iter()
            .map(|row| writeln!(f, "({})", row.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ")))
            .collect()
    }
}

