pub mod alphabets;

use std::collections::HashMap;

use nalgebra::{DMatrix, Dyn};

use self::alphabets::Russian;

pub trait Alphabet {
    fn has_letter(&self, c: &char) -> bool;
    fn code(&self, c: &char) -> Option<usize>;
    fn alphabet(&self) -> &HashMap<char, usize>;
}

pub struct Cipher<T> where T: Alphabet {
    alphabet: T,
}

impl<T> Cipher<T> where T: Alphabet {
    pub fn new(alphabet: T) -> Self {
        Self { alphabet }
    }
}

impl Cipher<Russian> {
    pub fn encode(&self, message: &str) -> Vec<usize> {
        message.chars()
            .map(|c| self.alphabet.code(&c).unwrap_or(99))
            .collect()
    }

    pub fn matrix_from(&self, message: &str) -> DMatrix<usize> {
        let result = self.encode(message);
        DMatrix::from_vec_generic(Dyn(result.len() / 3), Dyn(3), result)
    }
}
