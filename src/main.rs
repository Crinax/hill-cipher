mod cipher;

use cipher::alphabets::Russian;
use crate::cipher::Cipher;

fn main() {
    let ru = Russian::new();
    let cipher = Cipher::new(ru);
    println!("{:?}", cipher.matrix_from("привет"));
}
