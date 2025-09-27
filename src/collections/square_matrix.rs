use std::{fmt, vec};

pub struct SquareMatrix {
    pub data: Vec<Vec<f64>>,
    pub size: usize,
}

impl SquareMatrix {
    pub fn new(data: Vec<Vec<f64>>) -> Self {
        if data.is_empty() {
            return Self {
                data: vec![],
                size: 0,
            };
        }
        let size = data.len();

        let data = data
            .into_iter()
            .map(|mut row| {
                row.resize(size, 0.0);
                row
            })
            .collect::<Vec<_>>();

        Self { data, size }
    }

    pub fn zeros(n: usize) -> Self {
        Self {
            size: n,
            data: vec![vec![0.0; n]; n],
        }
    }
}

impl fmt::Display for SquareMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}
