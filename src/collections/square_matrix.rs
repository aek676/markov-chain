use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CreationError {
    EmptyMatrix,
    NonSquareMatrix,
    InvalidSize,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::EmptyMatrix => "matrix is empty",
            CreationError::NonSquareMatrix => "matrix is not square",
            CreationError::InvalidSize => "matrix has invalid size",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(Debug, PartialEq, Clone)]
pub struct SquareMatrix {
    pub data: Vec<Vec<f64>>,
    pub size: usize,
}

impl SquareMatrix {
    pub fn new(data: Vec<Vec<f64>>) -> Result<Self, CreationError> {
        match data {
            x if x.is_empty() => Err(CreationError::EmptyMatrix),
            x if x.iter().any(|row| row.is_empty()) => Err(CreationError::EmptyMatrix),
            x if x.iter().any(|row| row.len() != x.len()) => Err(CreationError::NonSquareMatrix),
            x => {
                let size = x.len();
                Ok(Self { data: x, size })
            }
        }
    }

    pub fn zeros(n: usize) -> Result<Self, CreationError> {
        match n {
            0 => Err(CreationError::InvalidSize),
            _ => Ok(Self {
                size: n,
                data: vec![vec![0.0; n]; n],
            }),
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
