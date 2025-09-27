use crate::square_matrix::{CreationError, SquareMatrix};

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

pub fn determinant(mtx: &SquareMatrix, n: usize) -> f64 {
    if n == 1 {
        return mtx.data[0][0];
    }

    let mut det = 0.0;
    let mut cof = SquareMatrix::zeros(n);
    let mut sign = 1.0;

    for f in 0..n {
        cofactor(mtx, &mut cof, 0, f, n);
        det += sign * mtx.data[0][f] * determinant(&cof, n - 1);
        sign = -sign;
    }

    det
}

pub fn adjoint(mtx: &SquareMatrix) -> SquareMatrix {
    let n = mtx.size;
    let mut adj = SquareMatrix::zeros(n);
    if n == 1 {
        adj.data[0][0] = 1.0;
        return adj;
    }

    let mut cof = SquareMatrix::zeros(n);

    for i in 0..n {
        for j in 0..n {
            cofactor(mtx, &mut cof, i, j, n);
            let sign = if (i + j) % 2 == 0 { 1.0 } else { -1.0 };
            adj.data[j][i] = sign * determinant(&cof, n - 1);
        }
    }

    adj
}

pub fn inverse(mtx: &SquareMatrix) -> Option<SquareMatrix> {
    let n = mtx.size;
    let det = determinant(mtx, n);
    if det == 0.0 {
        println!("Singular matrix, can't find its inverse");
        return None;
    }

    let adj = adjoint(mtx);
    let mut inv = SquareMatrix::zeros(n);

    for i in 0..n {
        for j in 0..n {
            inv.data[i][j] = adj.data[i][j] / det;
        }
    }

    Some(inv)
}
