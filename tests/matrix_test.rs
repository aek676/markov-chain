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

#[test]
fn test_square_matrix_zeros_error() {
    let mtx = SquareMatrix::zeros(0);
    assert_eq!(mtx, Err(CreationError::InvalidSize));
}

#[test]
fn test_square_matrix_zeros() {
    let mtx = SquareMatrix::zeros(3);
    assert_eq!(
        mtx,
        Ok(SquareMatrix {
            data: vec![
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0],
                vec![0.0, 0.0, 0.0]
            ],
            size: 3
        })
    );
}

#[test]
pub fn test_square_matrix_display() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
    let display = format!("{}", mtx);
    let expected = "[1.0, 2.0]\n[3.0, 4.0]\n";
    assert_eq!(display, expected);
}

#[test]
fn test_creation_error_display() {
    let err = CreationError::EmptyMatrix;
    assert_eq!(format!("{}", err), "matrix is empty");
}

#[test]
fn test_creation_error_non_square_display() {
    let err = CreationError::NonSquareMatrix;
    assert_eq!(format!("{}", err), "matrix is not square");
}

#[test]
fn test_creation_error_invalid_size_display() {
    let err = CreationError::InvalidSize;
    assert_eq!(format!("{}", err), "matrix has invalid size");
}
