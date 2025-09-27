use markov_chain::collections::square_matrix::Matrix;

#[test]
fn test_scale_matrix_basic() {
    let mut matrix = Matrix::new(vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ]);
    let scalar = 2.0;
    let expected = vec![
        vec![2.0, 4.0, 6.0],
        vec![8.0, 10.0, 12.0],
        vec![14.0, 16.0, 18.0],
    ];

    matrix.scale_matrix(scalar);
    assert_eq!(matrix.data, expected);
}

#[test]
fn test_scale_matrix_with_zero() {
    let mut matrix = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let scalar = 0.0;
    let expected = vec![vec![0.0, 0.0], vec![0.0, 0.0]];

    matrix.scale_matrix(scalar);
    assert_eq!(matrix.data, expected);
}

#[test]
fn test_scale_matrix_with_negative_scalar() {
    let mut matrix = Matrix::new(vec![vec![1.0, -2.0], vec![-3.0, 4.0]]);
    let scalar = -2.0;
    let expected = vec![vec![-2.0, 4.0], vec![6.0, -8.0]];

    matrix.scale_matrix(scalar);
    assert_eq!(matrix.data, expected);
}

#[test]
fn test_scale_matrix_with_fractional_scalar() {
    let mut matrix = Matrix::new(vec![vec![2.0, 4.0], vec![6.0, 8.0]]);
    let scalar = 0.5;
    let expected = vec![vec![1.0, 2.0], vec![3.0, 4.0]];

    matrix.scale_matrix(scalar);
    assert_eq!(matrix.data, expected);
}

#[test]
fn test_scale_matrix_empty() {
    let mut matrix = Matrix::new(vec![]);
    let scalar = 2.0;
    let expected: Vec<Vec<f64>> = vec![];

    matrix.scale_matrix(scalar);
    assert_eq!(matrix.data, expected);
}

#[test]
fn test_scale_matrix_single_element() {
    let mut matrix = Matrix::new(vec![vec![5.0]]);
    let scalar = 3.0;
    let expected = vec![vec![15.0]];

    matrix.scale_matrix(scalar);
    assert_eq!(matrix.data, expected);
}
