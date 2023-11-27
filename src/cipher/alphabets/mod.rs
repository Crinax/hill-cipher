use std::{borrow::Cow, collections::HashMap};

use super::Alphabet;

pub struct Russian {
    map: HashMap<char, isize>,
}

impl Russian {
    const ALPHABET: Cow<'static, str> = Cow::Borrowed(
        "абвгдеёжзийклмнопрстуфхцчшщъыьэюяАБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ ,.?!-:;\"@()№",
    );

    pub fn new() -> Self {
        let alphabets_and_index: HashMap<char, isize> = Self::ALPHABET
            .chars()
            .enumerate()
            .map(|(i, c)| (c, i as isize))
            .collect();
        Self {
            map: alphabets_and_index,
        }
    }
}

impl Alphabet for Russian {
    fn has_letter(&self, c: &char) -> bool {
        self.map.contains_key(c)
    }

    fn code(&self, c: &char) -> Option<isize> {
        self.map.get(c).copied()
    }

    fn alphabet(&self) -> &HashMap<char, isize> {
        &self.map
    }

    fn size(&self) -> usize {
        return Russian::ALPHABET.chars().count();
    }

    fn get_char(&self, index: usize) -> Option<char> {
        Russian::ALPHABET.chars().nth(index)
    }
}
