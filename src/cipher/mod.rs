pub mod alphabets;

use std::collections::HashMap;

use nalgebra::{DMatrix, Dyn};
use rand::Rng;

use self::alphabets::Russian;

pub trait Alphabet {
    fn has_letter(&self, c: &char) -> bool;
    fn code(&self, c: &char) -> Option<isize>;
    fn alphabet(&self) -> &HashMap<char, isize>;
    fn size(&self) -> usize;
    fn get_char(&self, index: usize) -> Option<char>;
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
    pub fn encode(&self, message: &str) -> Vec<isize> {
        let mut another_message = message.to_owned();
        let message_len = another_message.chars().count();

        if message_len % 3 != 0 {
            another_message = another_message + &self.generate_random_string(3 - message_len % 3);
        }

        another_message.chars()
            .map(|c| self.alphabet.code(&c).unwrap())
            .collect()
    }

    pub fn matrix_from(&self, message: &str) -> DMatrix<isize> {
        let result = self.encode(message);
        DMatrix::from_vec_generic(Dyn(result.len() / 3), Dyn(3), result)
    }

    pub fn generate_random_string(&self, count: usize) -> String {
        (0..count).map(|_| {
            let random = rand::thread_rng().gen_range(0..self.alphabet.size());

            self.alphabet.get_char(random).unwrap()
        })
            .collect()
    }

    pub fn generate_key(&self) -> String {
        let random_part = self.generate_random_string(6);
        "сед".to_owned() + &random_part
    }

    pub fn generate_matrix_key(&self) -> DMatrix<isize> {
        self.matrix_from(&self.generate_key())
    }

    pub fn get_transpose(&self, matrix: DMatrix<isize>) -> DMatrix<isize> {
        matrix.transpose()
    }

    pub fn get_determinant(&self, matrix: DMatrix<isize>) -> isize {
        let det: f64 = matrix.map(|v| nalgebra::ComplexField::from_real(v as f64)).determinant();
        return det as isize
    }
}
