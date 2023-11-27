mod cipher;
mod math;

use cipher::alphabets::Russian;
use crate::cipher::Cipher;
use input_macro::input;

fn main() {
    let ru = Russian::new();
    let cipher = Cipher::new(ru);
    let encrypted = cipher.encrypt(input!("Enter message in ru: ").as_str());

    println!("Encrypted: {encrypted:?}");
}
