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
    let decrypted = cipher.decrypt(&encrypted.1, &encrypted.0);

    println!("Encrypted: {:?} with key {:?}", encrypted.1, encrypted.0);

    println!("Decrypted: {:?}", decrypted.unwrap_or_else(|| "None".into()));
}
