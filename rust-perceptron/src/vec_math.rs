fn access(vec: &Vec<f64>, index: usize) -> &f64 {
    vec.get(index).unwrap_or_else(|| &0.0)
}

pub fn dot_product(vec_a: &Vec<f64>, vec_b: &Vec<f64>) -> f64 {
    vec_a
        .iter()
        .enumerate()
        .fold(0.0, |acc, (i, &x)| acc + x * access(&vec_b, i))
}

// pub fn vector_add(vec_a: &Vec<f64>, vec_b: &Vec<f64>) -> Vec<f64> {
//     vec_a
//         .iter()
//         .enumerate()
//         .map(|(i, &x)| x + access(&vec_b, i))
//         .collect()
// }

pub fn vector_add_mut(vec_a: &mut Vec<f64>, vec_b: &Vec<f64>) {
    vec_a.iter_mut().enumerate().for_each(|(i, x)| {
        *x += access(&vec_b, i);
    })
}

pub fn scalar_multiply(vec: &Vec<f64>, scalar: f64) -> Vec<f64> {
    vec.iter().map(|&x| x * scalar).collect()
}
pub fn scalar_multiply_mut(vec: &mut Vec<f64>, scalar: f64) {
    vec.iter_mut().for_each(|x| {
        *x *= scalar;
    })
}

pub fn sigmoid(a: f64) -> f64 {
    1.0 / (1.0 + (-a).exp())
}
