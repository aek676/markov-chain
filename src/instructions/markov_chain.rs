use crate::matrix_operations::*;
use crate::square_matrix::SquareMatrix;

pub fn teleportation_model(mtx: &SquareMatrix, alpha: f64) -> SquareMatrix {
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

pub fn calc_eigenvector(mtx: &SquareMatrix) -> Vec<f64> {
    let mut eigenvector = vec![0.0; mtx.size];

    let mut substracted_matrix = substract_identity(mtx);

    substracted_matrix
        .data
        .iter_mut()
        .for_each(|row| *row.last_mut().unwrap() = 1.0);

    if let Some(inv) = inverse(&substracted_matrix) {
        eigenvector = inv.data.last().unwrap().clone();
    } else {
        println!("Inverse Matrix: Matrix is not invertible");
    }

    eigenvector
}
