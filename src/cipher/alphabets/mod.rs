use std::{collections::HashMap, borrow::Cow};

use super::Alphabet;

pub struct Russian {
    map: HashMap<char, usize>,
}

impl Russian {
    const ALPHABET: Cow<'static, str> = Cow::Borrowed(
        "абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ ,.?!-"
    );

    pub fn new() -> Self {
        let alphabets_and_index: HashMap<char, usize> =
            Self::ALPHABET
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i))
            .collect();
        Self { map: alphabets_and_index }
    }
}

impl Alphabet for Russian {
    fn has_letter(&self, c: &char) -> bool {
        self.map.contains_key(c)
    }

    fn code(&self, c: &char) -> Option<usize> {
        self.map.get(c).copied()
    }

    fn alphabet(&self) -> &HashMap<char, usize> {
        &self.map
    }
}
