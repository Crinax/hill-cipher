use nalgebra::{ComplexField, DMatrix};

pub fn cropping_modulo(value: isize, modulo: isize) -> usize {
    ((modulo + (value % modulo)) % modulo) as usize
}

pub fn get_modularly_inverse(a: isize, b: isize) -> Option<isize> {
    let gcd_result: GCDResult = extend_gcd(a, b);

    if gcd_result.gcd != 1 {
        return None;
    }

    if gcd_result.x > 0 {
        Some(gcd_result.x)
    } else {
        Some(b + gcd_result.x)
    }
}

pub fn get_adjugate_matrix(matrix: &DMatrix<isize>) -> DMatrix<isize> {
    matrix.transpose().map_with_location(|row, col, _| {
        let minor_r_c: f64 = matrix
            .clone()
            .remove_row(row)
            .remove_column(col)
            .map(|v| ComplexField::from_real(v as f64))
            .determinant();

        // I hope it doesn't break
        (-1 as isize).pow((row + col) as u32) * (minor_r_c as isize)
    })
}

pub fn find_modulary_inverse_matrix(
    matrix: &DMatrix<isize>,
    modulo: isize,
) -> Option<DMatrix<isize>> {
    let det = get_modularly_inverse(find_matrix_determinant(&matrix), modulo);

    if det.is_none() {
        return None;
    }

    let det = cropping_modulo(det.unwrap(), modulo) as isize;

    // And hope it too (but it break down)
    Some(
        get_adjugate_matrix(matrix)
            .map(|v| ((v % modulo) * det) % modulo)
            .transpose()
            .map(|v| if v >= 0 { v } else { modulo + v }),
    )
}

pub fn find_matrix_determinant(matrix: &DMatrix<isize>) -> isize {
    let det: f64 = matrix
        .map(|v| nalgebra::ComplexField::from_real(v as f64))
        .determinant();
    return det as isize;
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
