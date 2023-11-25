mod cipher;
mod math;

use cipher::alphabets::Russian;
use crate::cipher::{Cipher, Alphabet};

fn main() {
    let ru = Russian::new();
    let ru_len = ru.size() as isize;
    let cipher = Cipher::new(ru);
    let message_matrix = cipher.matrix_from("привет");
    println!("{:?}", message_matrix);
    let key = cipher.generate_matrix_key();
    println!("Key: {:?}", key);
    let det = cipher.get_determinant(key);
    let cropped_det = math::cropping_modulo(det, ru_len);
    println!("Det cropped: {:?}", math::cropping_modulo(det, ru_len));
    println!("Alphabet len: {:?}", ru_len);
    let gcd = math::extend_gcd(cropped_det as isize, ru_len);
    println!("GCD: {:?}", gcd);
}
