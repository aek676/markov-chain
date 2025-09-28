use crate::square_matrix::{CreationError, SquareMatrix};
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum OperationError {
    SingularMatrix,
}

impl fmt::Display for OperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            OperationError::SingularMatrix => "matrix is singular and cannot be inverted",
        };
        f.write_str(description)
    }
}

impl Error for OperationError {}

pub fn substract_identity(mtx: &SquareMatrix) -> Result<SquareMatrix, CreationError> {
    let mut result = mtx.data.clone();
    for (i, row) in result.iter_mut().enumerate() {
        row[i] -= 1.0;
    }

    SquareMatrix::new(result)
}

pub fn cofactor(mtx: &SquareMatrix, cof: &mut SquareMatrix, p: usize, q: usize, n: usize) {
    let (mut i, mut j) = (0, 0);

    for row in 0..n {
        for col in 0..n {
            if row != p && col != q {
                cof.data[i][j] = mtx.data[row][col];
                j += 1;

                if j == n - 1 {
                    j = 0;
                    i += 1;
                }
            }
        }
    }
}

pub fn determinant(mtx: &SquareMatrix, n: usize) -> Result<f64, CreationError> {
    if n == 1 {
        return Ok(mtx.data[0][0]);
    }

    let mut det = 0.0;
    let mut cof = SquareMatrix::zeros(n)?;
    let mut sign = 1.0;

    for f in 0..n {
        cofactor(mtx, &mut cof, 0, f, n);
        det += sign * mtx.data[0][f] * determinant(&cof, n - 1)?;
        sign = -sign;
    }

    Ok(det)
}

pub fn adjoint(mtx: &SquareMatrix) -> Result<SquareMatrix, CreationError> {
    let n = mtx.size;
    let mut adj = SquareMatrix::zeros(n)?;
    if n == 1 {
        adj.data[0][0] = 1.0;
        return Ok(adj);
    }

    let mut cof = SquareMatrix::zeros(n)?;

    for i in 0..n {
        for j in 0..n {
            cofactor(mtx, &mut cof, i, j, n);
            let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            adj.data[j][i] = sign * determinant(&cof, n - 1)?;
        }
    }

    Ok(adj)
}

pub fn inverse(mtx: &SquareMatrix) -> Result<SquareMatrix, Box<dyn Error>> {
    let n = mtx.size;
    let det = determinant(mtx, n)?;
    if det == 0.0 {
        return Err(Box::new(OperationError::SingularMatrix));
    }

    let adj = adjoint(mtx)?;
    let mut inv = SquareMatrix::zeros(n)?;

    for i in 0..n {
        for j in 0..n {
            inv.data[i][j] = adj.data[i][j] / det;
        }
    }

    Ok(inv)
}
