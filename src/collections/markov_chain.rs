use crate::square_matrix::SquareMatrix;
use rand::Rng;

use std::collections::{BTreeMap, BTreeSet};

pub struct MarkovChain {
    pub states: Vec<String>,
    pub index: BTreeMap<String, usize>,
    pub transition_matrix: SquareMatrix,
}

impl MarkovChain {
    pub fn new() -> Self {
        Self {
            states: Vec::new(),
            index: BTreeMap::new(),
            transition_matrix: SquareMatrix::zeros(1).unwrap(),
        }
    }

    pub fn fit(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let tokens = Self::tokenize(text);

        let vocab_set: BTreeSet<String> = tokens.iter().cloned().collect();
        self.states = vocab_set.into_iter().collect();

        self.index = self
            .states
            .iter()
            .enumerate()
            .map(|(i, w)| (w.clone(), i))
            .collect();

        let n = self.states.len();
        let mut mtx = SquareMatrix::zeros(n)?;
        for w in tokens.windows(2) {
            let i = self.index[&w[0]];
            let j = self.index[&w[1]];
            mtx.data[i][j] += 1.0;
        }

        for row in mtx.data.iter_mut() {
            let sum: f64 = row.iter().sum();
            if sum > 0.0 {
                for v in row.iter_mut() {
                    *v /= sum;
                }
            }
        }

        self.transition_matrix = mtx;
        Ok(())
    }

    fn tokenize(text: &str) -> Vec<String> {
        text.to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphabetic() || c.is_whitespace() {
                    c
                } else {
                    ' '
                }
            })
            .collect::<String>()
            .split_whitespace()
            .map(|w| w.to_string())
            .collect()
    }

    pub fn next_greedy_nonself(&self, word: &str) -> Option<&str> {
        let &i = self.index.get(word)?;
        let row = &self.transition_matrix.data[i];

        let mut best_nonself: Option<(usize, f64)> = None;
        for (j, &p) in row.iter().enumerate() {
            if j == i {
                continue;
            }
            if let Some((bj, bp)) = best_nonself {
                if p > bp || (p == bp && j < bj) {
                    best_nonself = Some((j, p));
                }
            } else {
                best_nonself = Some((j, p));
            }
        }

        if let Some((j, p)) = best_nonself {
            if p > 0.0 {
                return Some(&self.states[j]);
            }
        }

        let j = row
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(j, _)| j)?;
        Some(&self.states[j])
    }

    pub fn generate_greedy_nonself(&self, start: &str, steps: usize) -> Option<String> {
        let mut cur = *self.index.get(start)?;
        let mut out = String::from(start);
        for _ in 0..steps {
            let next = self.next_greedy_nonself(&self.states[cur])?;
            out.push(' ');
            out.push_str(next);
            cur = *self.index.get(next)?;
        }
        Some(out)
    }

    fn sample_row(&self, row: &[f64], forbid: Option<usize>) -> Option<usize> {
        let mut sum = 0.0;
        for (j, &p) in row.iter().enumerate() {
            if Some(j) != forbid {
                sum += p;
            }
        }
        if sum <= 0.0 {
            return None;
        }
        let mut r = rand::thread_rng().gen_range(0.0..sum);
        for (j, &p) in row.iter().enumerate() {
            if Some(j) == forbid {
                continue;
            }
            if p == 0.0 {
                continue;
            }
            if r <= p {
                return Some(j);
            }
            r -= p;
        }
        None
    }

    pub fn generate(&self, start: &str, steps: usize) -> Option<String> {
        let mut cur = *self.index.get(start)?;
        let mut out = String::from(start);
        for _ in 0..steps {
            let next = self
                .sample_row(&self.transition_matrix.data[cur], Some(cur))
                .or_else(|| self.sample_row(&self.transition_matrix.data[cur], None))?;
            out.push(' ');
            out.push_str(&self.states[next]);
            cur = next;
        }
        Some(out)
    }
}
