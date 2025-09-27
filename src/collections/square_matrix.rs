use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CreationError {
    EmptyMatrix,
    NonSquareMatrix,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::EmptyMatrix => "matrix is empty",
            CreationError::NonSquareMatrix => "matrix is not square",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

pub struct SquareMatrix {
    pub data: Vec<Vec<f64>>,
    pub size: usize,
}

impl SquareMatrix {
    pub fn new(data: Vec<Vec<f64>>) -> Result<Self, CreationError> {
        match data {
            x if x.is_empty() => Err(CreationError::EmptyMatrix),
            x if x.len() != x[0].len() => Err(CreationError::NonSquareMatrix),
            x => {
                let size = x.len();
                Ok(Self { data: x, size })
            }
        }
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
