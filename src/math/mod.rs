pub fn cropping_modulo(value: isize, modulo: isize) -> usize {
    ((modulo + (value % modulo)) % modulo) as usize
}

// Возвращаемый результат представляет собой структуру
// gcd - НОД
// x - первый коэффициент
// y - второй коэффициент
#[derive(Debug)]
pub struct GCDResult {
    pub gcd: isize,
    pub x: isize,
    pub y: isize,
}

// Расширенный алгоритм Евклида
pub fn extend_gcd(a: isize, b: isize) -> GCDResult {
    let (mut s, mut old_s): (isize, isize) = (0, 1);
    let (mut r, mut old_r): (isize, isize) = (b, a);

    while r != 0 {
        let quotinent: isize = old_r / r;
        (old_r, r) = (r, old_r - quotinent * r);
        (old_s, s) = (s, old_s - quotinent * s);
    }

    let bezout_t: isize;

    if b != 0 {
        bezout_t = (old_r - old_s * a) / b;
    } else {
        bezout_t = 0;
    }

    GCDResult {
        gcd: old_r,
        x: old_s,
        y: bezout_t,
    }
}

