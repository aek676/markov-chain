use std::vec;

use markov_chain::collections::{CreationError, SquareMatrix};

#[test]
fn new_returns_error_on_empty_matrix() {
    let mtx = SquareMatrix::new(vec![]);
    assert_eq!(mtx, Err(CreationError::EmptyMatrix));
}

#[test]
fn new_returns_error_when_a_row_is_empty() {
    let mtx = SquareMatrix::new(vec![vec![0.0, 0.0], vec![]]); // Vector vacío
    assert_eq!(mtx, Err(CreationError::EmptyMatrix));
}

#[test]
fn new_returns_error_on_non_square_matrix_more_columns() {
    assert_eq!(
        SquareMatrix::new(vec![vec![1.0, 2.0], vec![3.0]]), // 2x1, no cuadrada
        Err(CreationError::NonSquareMatrix)
    );
}

#[test]
fn new_returns_error_on_non_square_matrix_more_rows() {
    assert_eq!(
        SquareMatrix::new(vec![vec![1.0], vec![2.0], vec![3.0]]), // 1x3, no cuadrada
        Err(CreationError::NonSquareMatrix)
    );
}

#[test]
fn test_square_matrix_creation_valid() {
    let mtx = SquareMatrix::new(vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
    assert_eq!(
        mtx,
        Ok(SquareMatrix {
            data: vec![vec![0.0, 1.0], vec![1.0, 0.0]],
            size: 2
        })
    );
}


