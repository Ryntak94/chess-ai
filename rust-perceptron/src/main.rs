mod vec_math;
mod perceptron;

use {
    rand,
    perceptron::Perceptron,
};

fn main() {
    let mut perceptron = Perceptron::from_rng(
        vec![
            vec![0.0, 0.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![1.0, 0.0, 1.0],
            vec![0.0, 1.0, 0.0]
        ],
        vec![0.0, 1.0, 1.0, 0.0],
        0.0,
        1_000,
        false,
        rand::thread_rng()
    );
    perceptron.train();

    let test = vec![1.0, 1.0, 1.0];
    println!("Prediction of {:?} : {}", &test, perceptron.predict(&test));
}
