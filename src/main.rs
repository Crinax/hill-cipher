mod cipher;
mod math;

use crate::cipher::Cipher;
use cipher::alphabets::Russian;
use input_macro::input;

fn main() {
    let ru = Russian::new();
    let cipher = Cipher::new(ru);
    let message = input!("Enter message in ru: ");
    let encrypted = cipher.encrypt(&message);
    let descrypted = cipher.decrypt(&encrypted.1, &encrypted.0);

    println!("Encrypted: {encrypted:?}");
    println!("Decrypted: {descrypted:?}");
}
