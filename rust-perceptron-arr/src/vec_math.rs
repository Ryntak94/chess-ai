fn access(vec: &[f64; 3], index: usize) -> &f64 {
    vec.get(index).unwrap_or_else(|| &0.0)
}

pub fn dot_product(vec_a: &[f64; 3], vec_b: &[f64; 3]) -> f64 {
    vec_a
        .iter()
        .enumerate()
        .fold(0.0, |acc, (i, &x)| acc + x * access(&vec_b, i))
}

// pub fn vector_add(vec_a: &[f64; 3], vec_b: &[f64; 3]) -> [f64; 3] {
//     vec_a
//         .iter()
//         .enumerate()
//         .map(|(i, &x)| x + access(&vec_b, i))
//         .collect()
// }

pub fn vector_add_mut(vec_a: &mut [f64; 3], vec_b: &[f64; 3]) {
    vec_a
        .iter_mut()
        .enumerate()
        .for_each(|(i, x)| { *x += access(&vec_b, i); })
}

pub fn scalar_multiply(vec: &[f64; 3], scalar: f64) -> [f64; 3] {
    let mut new_vec = vec.clone();
    for x in &mut new_vec {
        *x *= scalar;
    }
    return new_vec;
}

pub fn scalar_multiply_mut(vec: &mut [f64; 3], scalar: f64) {
    vec
        .iter_mut()
        .for_each(|x| { *x *= scalar; })
}

pub fn sigmoid(a: f64) -> f64 {
    1.0 / (1.0 + (-a).exp())
}

