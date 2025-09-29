use std::collections::BTreeMap;
use std::collections::HashSet;
use std::error::Error;
use std::ops::Index;

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
    let text = "I love cats. Cats are my favorite animal. I have two cats.";

    let text_tokenized = text
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || c.is_whitespace())
        .collect::<String>();

    println!("Input text: {}", text_tokenized);

    let words = text_tokenized.split_whitespace().collect::<Vec<&str>>();

    println!("Words: {:?}", words);

    let n_grams: Vec<(String, String)> = words
        .windows(2)
        .map(|w| (w[0].to_string(), w[1].to_string()))
        .collect();

    println!("N-grams: {:?}", n_grams);

    let unique_words: Vec<&str> = {
        let mut set = HashSet::new();
        for &word in &words {
            set.insert(word);
        }
        set.into_iter().collect()
    };

    println!("Unique words: {:?}", unique_words.len());

    let mut mtx_transition = SquareMatrix::zeros(unique_words.len())?;

    for (i, word) in unique_words.iter().enumerate() {
        for (j, next_word) in unique_words.iter().enumerate() {
            mtx_transition.data[i][j] = n_grams
                .iter()
                .filter(|&n_gram| n_gram.0 == *word && n_gram.1 == *next_word)
                .count() as f64;
        }
    }

    mtx_transition.data.iter_mut().for_each(|row| {
        let row_sum: f64 = row.iter().sum();
        if row_sum > 0.0 {
            row.iter_mut().for_each(|val| *val /= row_sum);
        }
    });

    println!("Transition Matrix: \n{}", mtx_transition);

    let mut current_word = "i";
    let mut generated_text = "i".to_string();

    for i in 0..10 {
        let current_word_index = unique_words
            .iter()
            .position(|&w| w == current_word)
            .unwrap();

        let probabilities = &mtx_transition.data[current_word_index];
        let next_word_index = probabilities
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(index, _)| index)
            .unwrap();

        let next_word = unique_words[next_word_index];

        generated_text.push_str(&format!(" {}", next_word));

        current_word = next_word;
    }

    println!("Generated text: {}", generated_text);

    let texto = "Hola, Mundo!\nTengo 2 manzanas y 3 peras.";
    let tokens = tokenize(text);
    println!("{:?}", tokens);

    let model = build_model(&tokens);

    println!("Model Transition Matrix: \n{}", model);

    Ok(())
}

fn tokenize(text: &str) -> Vec<String> {
    text.lines() // divide en líneas
        .flat_map(|line| {
            line.to_lowercase() // minúsculas
                .chars()
                .map(|c| {
                    if c.is_alphabetic() || c.is_whitespace() {
                        c
                    } else {
                        ' ' // reemplaza cualquier símbolo por espacio
                    }
                })
                .collect::<String>() // reconstruye la línea limpia
                .split_whitespace() // divide en palabras
                .map(|w| w.to_string()) // convierte a String
                .collect::<Vec<String>>()
        })
        .collect()
}

fn build_model(tokens: &[String]) -> SquareMatrix {
    let unique_words: Vec<&String> = {
        let mut set = HashSet::new();
        for word in tokens {
            set.insert(word);
        }
        set.into_iter().collect()
    };

    let mut mtx_transition = SquareMatrix::zeros(unique_words.len()).unwrap();

    for (i, word) in unique_words.iter().enumerate() {
        for (j, next_word) in unique_words.iter().enumerate() {
            mtx_transition.data[i][j] = tokens
                .windows(2)
                .filter(|w| w[0] == **word && w[1] == **next_word)
                .count() as f64;
        }
    }

    mtx_transition.data.iter_mut().for_each(|row| {
        let row_sum: f64 = row.iter().sum();
        if row_sum > 0.0 {
            row.iter_mut().for_each(|val| *val /= row_sum);
        }
    });

    mtx_transition
}
