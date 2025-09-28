use crate::matrix_operations::*;
use crate::square_matrix::{CreationError, SquareMatrix};

pub fn teleportation_model(mtx: &SquareMatrix, alpha: f64) -> Result<SquareMatrix, CreationError> {
    let n = mtx.size as f64;
    let teleportation_value = (1.0 - alpha) / n;

    let result = mtx
        .data
        .iter()
        .map(|row| {
            row.iter()
                .map(|&element| alpha * element + teleportation_value)
                .collect()
        })
        .collect();

    SquareMatrix::new(result)
}

pub fn calc_eigenvector(mtx: &SquareMatrix) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut subtracted_matrix = substract_identity(mtx)?;

    subtracted_matrix
        .data
        .iter_mut()
        .for_each(|row| *row.last_mut().unwrap() = 1.0);

    Ok(inverse(&subtracted_matrix)?.data.last().unwrap().clone())
}
