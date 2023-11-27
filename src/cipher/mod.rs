pub mod alphabets;

use std::collections::HashMap;

use crate::math::{self, extend_gcd};
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

pub struct Cipher<T>
where
    T: Alphabet,
{
    alphabet: T,
}

impl<T> Cipher<T>
where
    T: Alphabet,
{
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

        another_message
            .chars()
            .map(|c| self.alphabet.code(&c).unwrap())
            .collect()
    }

    pub fn matrix_from(&self, message: &str) -> DMatrix<isize> {
        let result = self.encode(message);
        DMatrix::from_vec_generic(Dyn(result.len() / 3), Dyn(3), result)
    }

    pub fn generate_random_string(&self, count: usize) -> String {
        (0..count)
            .map(|_| {
                let random: usize = rand::thread_rng().gen_range(0..self.alphabet.size());

                self.alphabet.get_char(random).unwrap()
            })
            .collect()
    }

    pub fn generate_key(&self) -> String {
        let random_part = self.generate_random_string(6);
        "сед".to_owned() + &random_part
    }

    pub fn encryption_decryption(
        &self,
        message_matrix: DMatrix<isize>,
        key_matrix: DMatrix<isize>,
    ) -> String {
        let mut rows = Vec::new();

        for row_number in 0..message_matrix.nrows() {
            rows.push(message_matrix.row(row_number) * &key_matrix);
        }

        let result = DMatrix::from_rows(&rows)
            .map(|v| math::cropping_modulo(v, self.alphabet.size() as isize))
            .into_iter()
            .map(|v| self.alphabet.get_char(*v).unwrap())
            .collect::<String>();

        result
    }

    pub fn encrypt(&self, message: &str) -> (String, String) {
        let alphabet_size = self.alphabet.size() as isize;
        let key = {
            let mut is_valid_key = false;
            let mut counter = 0;
            let mut raw_key: String = "".to_owned();

            // It's bad way, sry
            while !is_valid_key {
                counter += 1;
                raw_key = self.generate_key();
                let matrix = self.matrix_from(&raw_key);
                let det = math::find_matrix_determinant(&matrix);
                let gcd = extend_gcd(alphabet_size, det);

                is_valid_key = gcd.gcd == 1;
            }

            println!("Cycles: {:?}", counter);
            raw_key
        };
        let encrypted =
            self.encryption_decryption(self.matrix_from(message), self.matrix_from(&key));

        (key, encrypted)
    }

    pub fn decrypt(&self, encrypted: &str, key: &str) -> Option<String> {
        let matrix_key = self.matrix_from(key);
        let message_matrix = self.matrix_from(encrypted);
        let inverted =
            math::find_modulary_inverse_matrix(&matrix_key, self.alphabet.size() as isize);

        if inverted.is_none() {
            return None;
        }

        let inverted = inverted.unwrap();

        Some(self.encryption_decryption(message_matrix, inverted))
    }
}
