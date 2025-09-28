use markov_chain::collections::SquareMatrix;
use markov_chain::instructions::matrix_operations::*;

#[test]
fn test_operation_error_display_singular_matrix() {
    let err = OperationError::SingularMatrix;
    assert_eq!(
        format!("{}", err),
        "matrix is singular and cannot be inverted"
    );
}

#[test]
fn test_substract_identity_1x1_diagonal_sub() {
    let mtx = SquareMatrix::new(vec![vec![5.0]]).unwrap();
    let result = substract_identity(&mtx);
    assert_eq!(result, Ok(SquareMatrix::new(vec![vec![4.0]]).unwrap()));
}

#[test]
fn test_substract_identity_2x2_diagonal_sub() {
    let mtx = SquareMatrix::new(vec![vec![5.0, 0.0], vec![0.0, 3.0]]).unwrap();
    let result = substract_identity(&mtx);
    assert_eq!(
        result,
        Ok(SquareMatrix::new(vec![vec![4.0, 0.0], vec![0.0, 2.0]]).unwrap())
    );
}
#[test]
fn test_substract_identity_3x3_diagonal_sub() {
    let mtx = SquareMatrix::new(vec![
        vec![5.0, 0.0, 0.0],
        vec![0.0, 3.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ])
    .unwrap();
    let result = substract_identity(&mtx);
    assert_eq!(
        result,
        Ok(SquareMatrix::new(vec![
            vec![4.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 0.0]
        ])
        .unwrap())
    );
}

#[test]
fn test_substract_identity_2x2_not_mut_original_matrix() {
    let mtx = SquareMatrix::new(vec![vec![5.0, 1.0], vec![2.0, 3.0]]).unwrap();
    let original_mtx = mtx.clone();
    let result = substract_identity(&mtx);
    assert_eq!(
        result,
        Ok(SquareMatrix::new(vec![vec![4.0, 1.0], vec![2.0, 2.0]]).unwrap())
    );
    assert_eq!(mtx, original_mtx); // Verifica que la matriz original no se haya modificado
}

#[test]
fn test_cofactor_2x2() {
    let data = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let mtx = SquareMatrix::new(data).unwrap();
    let mut cof = SquareMatrix::zeros(1).unwrap();

    cofactor(&mtx, &mut cof, 0, 0, 2);
    assert_eq!(cof.data[0][0], 4.0);

    cofactor(&mtx, &mut cof, 0, 1, 2);
    assert_eq!(cof.data[0][0], 3.0);

    cofactor(&mtx, &mut cof, 1, 0, 2);
    assert_eq!(cof.data[0][0], 2.0);

    cofactor(&mtx, &mut cof, 1, 1, 2);
    assert_eq!(cof.data[0][0], 1.0);
}

#[test]
fn test_cofactor_3x3() {
    let data = vec![
        vec![1.0, 2.0, 3.0],
        vec![4.0, 5.0, 6.0],
        vec![7.0, 8.0, 9.0],
    ];
    let mtx = SquareMatrix::new(data).unwrap();
    let mut cof = SquareMatrix::zeros(2).unwrap();

    cofactor(&mtx, &mut cof, 0, 0, 3);
    assert_eq!(cof.data[0][0], 5.0);
    assert_eq!(cof.data[0][1], 6.0);
    assert_eq!(cof.data[1][0], 8.0);
    assert_eq!(cof.data[1][1], 9.0);

    cofactor(&mtx, &mut cof, 1, 1, 3);
    assert_eq!(cof.data[0][0], 1.0);
    assert_eq!(cof.data[0][1], 3.0);
    assert_eq!(cof.data[1][0], 7.0);
    assert_eq!(cof.data[1][1], 9.0);
}

#[test]
fn test_cofactor_4x4_center() {
    let data = vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 10.0, 11.0, 12.0],
        vec![13.0, 14.0, 15.0, 16.0],
    ];
    let mtx = SquareMatrix::new(data).unwrap();
    let mut cof = SquareMatrix::zeros(3).unwrap();

    cofactor(&mtx, &mut cof, 1, 1, 4);
    let expected = vec![
        vec![1.0, 3.0, 4.0],
        vec![9.0, 11.0, 12.0],
        vec![13.0, 15.0, 16.0],
    ];
    assert_eq!(cof.data, expected);
}

#[test]
fn test_cofactor_removes_correct_row_column() {
    let data = vec![
        vec![10.0, 20.0, 30.0],
        vec![40.0, 50.0, 60.0],
        vec![70.0, 80.0, 90.0],
    ];
    let mtx = SquareMatrix::new(data).unwrap();
    let mut cof = SquareMatrix::zeros(2).unwrap();

    // Remove first row (0) and last column (2)
    cofactor(&mtx, &mut cof, 0, 2, 3);
    assert_eq!(cof.data[0][0], 40.0);
    assert_eq!(cof.data[0][1], 50.0);
    assert_eq!(cof.data[1][0], 70.0);
    assert_eq!(cof.data[1][1], 80.0);
}

#[test]
fn test_determinant_base_case_1x1_returns_single_element() {
    let mtx = SquareMatrix::new(vec![vec![42.0]]).unwrap();
    let det = determinant(&mtx, 1).unwrap();
    assert_eq!(det, mtx.data[0][0]);
}

#[test]
fn test_determinant_2x2_matches_ad_minus_bc() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
    let det = determinant(&mtx, 2).unwrap();
    assert_eq!(det, (1.0 * 4.0) - (2.0 * 3.0));
}

#[test]
fn test_determinant_3x3_known_value() {
    let mtx = SquareMatrix::new(vec![
        vec![-1.0, 0.5, 1.0],
        vec![0.25, -0.5, 1.0],
        vec![0.25, 0.25, 1.0],
    ])
    .unwrap();
    let det = determinant(&mtx, 3).unwrap();
    assert_eq!(det, 0.9375);
}

#[test]
fn test_determinant_identity_matrix_is_one() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
    let det = determinant(&mtx, 2).unwrap();
    assert_eq!(det, 1.0);
}

#[test]
fn test_determinant_triangular_matrix_is_product_of_diagonal() {
    let mtx = SquareMatrix::new(vec![
        vec![2.0, 1.0, 1.0],
        vec![0.0, 3.0, 1.0],
        vec![0.0, 0.0, 4.0],
    ])
    .unwrap();
    let det = determinant(&mtx, 3).unwrap();
    assert_eq!(det, 2.0 * 3.0 * 4.0);
}

#[test]
fn test_determinant_proportional_rows_returns_zero() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 2.0], vec![2.0, 4.0]]).unwrap();
    let det = determinant(&mtx, 2).unwrap();
    assert_eq!(det, 0.0);
}

#[test]
fn test_determinant_zero_row_returns_zero() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 2.0], vec![0.0, 0.0]]).unwrap();
    let det = determinant(&mtx, 2).unwrap();
    assert_eq!(det, 0.0);
}

#[test]
fn test_determinant_row_swap_changes_sign() {
    let mtx1 = SquareMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
    let mtx2 = SquareMatrix::new(vec![vec![3.0, 4.0], vec![1.0, 2.0]]).unwrap();
    let det1 = determinant(&mtx1, 2).unwrap();
    let det2 = determinant(&mtx2, 2).unwrap();
    assert_eq!(det1, -det2);
}

#[test]
fn test_adjoint_base_case_1x1_is_identity() {
    let mtx = SquareMatrix::new(vec![vec![7.0]]).unwrap();
    let adj = adjoint(&mtx).unwrap();
    assert_eq!(adj, SquareMatrix::new(vec![vec![1.0]]).unwrap());
}

#[test]
fn test_adjoint_2x2_matches_closed_form() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
    let adj = adjoint(&mtx).unwrap();
    assert_eq!(
        adj,
        SquareMatrix::new(vec![vec![4.0, -2.0], vec![-3.0, 1.0]]).unwrap()
    );
}

#[test]
fn test_adjoint_3x3_known_value() {
    let mtx = SquareMatrix::new(vec![
        vec![1.0, 2.0, 3.0],
        vec![0.0, 1.0, 4.0],
        vec![5.0, 6.0, 0.0],
    ])
    .unwrap();
    let adj = adjoint(&mtx).unwrap();
    assert_eq!(
        adj,
        SquareMatrix::new(vec![
            vec![-24.0, 18.0, 5.0],
            vec![20.0, -15.0, -4.0],
            vec![-5.0, 4.0, 1.0]
        ])
        .unwrap()
    );
}

#[test]
fn test_adjoint_of_identity_is_identity() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
    let adj = adjoint(&mtx).unwrap();
    assert_eq!(adj, mtx);
}

#[test]
fn test_adjoint_symmetric_matrix() {
    let mtx = SquareMatrix::new(vec![vec![2.0, 1.0], vec![1.0, 2.0]]).unwrap();
    let adj = adjoint(&mtx).unwrap();
    assert_eq!(
        adj,
        SquareMatrix::new(vec![vec![2.0, -1.0], vec![-1.0, 2.0]]).unwrap()
    );
}

#[test]
fn test_adjoint_non_symmetric_matrix() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]).unwrap();
    let adj = adjoint(&mtx).unwrap();
    assert_eq!(
        adj,
        SquareMatrix::new(vec![vec![4.0, -2.0], vec![-3.0, 1.0]]).unwrap()
    );
}

#[test]
fn test_inverse_2x2_known_value() {
    let mtx = SquareMatrix::new(vec![vec![4.0, 7.0], vec![2.0, 6.0]]).unwrap();
    let inv = inverse(&mtx).unwrap();
    let expected = SquareMatrix::new(vec![vec![0.6, -0.7], vec![-0.2, 0.4]]).unwrap();
    for i in 0..2 {
        for j in 0..2 {
            assert!((inv.data[i][j] - expected.data[i][j]).abs() < 1e-10);
        }
    }
}

#[test]
fn test_inverse_3x3_known_value() {
    let mtx = SquareMatrix::new(vec![
        vec![1.0, 2.0, 3.0],
        vec![0.0, 1.0, 4.0],
        vec![5.0, 6.0, 0.0],
    ])
    .unwrap();
    let inv = inverse(&mtx).unwrap();
    let expected = SquareMatrix::new(vec![
        vec![-24.0, 18.0, 5.0],
        vec![20.0, -15.0, -4.0],
        vec![-5.0, 4.0, 1.0],
    ])
    .unwrap();
    for i in 0..3 {
        for j in 0..3 {
            assert!((inv.data[i][j] - expected.data[i][j]).abs() < 1e-10);
        }
    }
}

#[test]
fn test_inverse_identity_matrix_returns_identity() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 0.0], vec![0.0, 1.0]]).unwrap();
    let inv = inverse(&mtx).unwrap();
    assert_eq!(inv, mtx);
}

#[test]
fn test_inverse_singular_matrix_returns_error() {
    let mtx = SquareMatrix::new(vec![vec![1.0, 2.0], vec![2.0, 4.0]]).unwrap();
    let result = inverse(&mtx);
    assert!(result.is_err());
    assert_eq!(
        result.unwrap_err().to_string(),
        "matrix is singular and cannot be inverted"
    );
}
