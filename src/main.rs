use crate::collections::*;
use crate::instructions::*;

mod collections;
mod instructions;

fn main() {
    let matrix_org = SquareMatrix::new(vec![
        vec![0.0, 0.5, 0.5, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0, 0.0],
    ]);

    let tp = teleportation_model(&matrix_org, 0.85);

    println!("Teleportation Model Matrix: \n{}", tp);

    let eigenvector = calc_eigenvector(&tp);

    println!("Eigenvector: {:?}", eigenvector);
}
