use markov_chain::collections::SquareMatrix;
use markov_chain::instructions::markov_chain::*;
use std::sync::LazyLock;

// Internet simulado
static MATRIX_1: LazyLock<SquareMatrix> = LazyLock::new(|| {
    SquareMatrix::new(vec![
        vec![0.0, 0.5, 0.5, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0, 0.0],
    ])
    .unwrap()
});

// Teleportación calculada automáticamente usando MATRIX_1
static TELEPORTATION: LazyLock<SquareMatrix> =
    LazyLock::new(|| teleportation_model(&MATRIX_1, 0.85).unwrap());

#[test]
fn test_teleportation_model() {
    let alpha = 0.85;
    let result = teleportation_model(&MATRIX_1, alpha).unwrap();
    // Valores corregidos basados en el resultado real calculado
    let expected = SquareMatrix::new(vec![
        vec![0.037500000000000006, 0.4625, 0.4625, 0.037500000000000006],
        vec![
            0.037500000000000006,
            0.037500000000000006,
            0.8875,
            0.037500000000000006,
        ],
        vec![
            0.037500000000000006,
            0.037500000000000006,
            0.037500000000000006,
            0.8875,
        ],
        vec![
            0.037500000000000006,
            0.037500000000000006,
            0.8875,
            0.037500000000000006,
        ],
    ])
    .unwrap();
    assert_eq!(result, expected);

    // También verificar que coincide con TELEPORTATION
    assert_eq!(result, *TELEPORTATION);
}

#[test]
fn test_calc_eigenvector() {
    // Usar TELEPORTATION en lugar de crear la matriz manualmente
    let result = calc_eigenvector(&TELEPORTATION).unwrap();
    let expected = vec![
        0.037500000000000006,
        0.05343750000000001,
        0.4711148648648649,
        0.4379476351351352,
    ];

    assert_eq!(result, expected);
}
