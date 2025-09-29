use std::error::Error;

use crate::collections::*;
use crate::instructions::*;

mod collections;
mod instructions;

/*
fn main() -> Result<(), Box<dyn Error>> {
    let matrix_org = SquareMatrix::new(vec![
        vec![0.0, 0.5, 0.5, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
        vec![0.0, 0.0, 1.0, 0.0],
    ])?;

    let tp = teleportation_model(&matrix_org, 0.85)?;

    println!("Teleportation Model Matrix: \n{}", tp);

    let eigenvector = calc_eigenvector(&tp)?;

    println!("Eigenvector: {:?}", eigenvector);

    Ok(())
}
*/
fn main() -> Result<(), Box<dyn Error>> {
    let text = std::fs::read_to_string("assets/quijote.txt")?;

    let mut mc = MarkovChain::new();

    mc.fit(&text)?;

    let start_word = "don"; 
    let generated = mc
        .generate_greedy_nonself(start_word, 100)
        .unwrap_or_else(|| "No se pudo generar texto".to_string());

    println!("Texto generado: {}", generated);

    let generated_random = mc
        .generate(start_word, 100)
        .unwrap_or_else(|| "No se pudo generar texto".to_string());

    println!("Texto generado (random): {}", generated_random);

    Ok(())
}
