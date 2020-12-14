mod vec_math;
mod perceptron;

use {
    rand,
    perceptron::Perceptron,
};

fn main() {
    println!("Hello?");
    let mut perceptron = Perceptron::from_rng(
        [
            [0.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0]
        ],
        [0.0, 1.0, 1.0, 0.0],
        0.0,
        1_000,
        false,
        &mut rand::thread_rng()
    );
    perceptron.train();

    let test = [1.0, 1.0, 1.0];
    println!("Prediction of {:?} : {}", &test, perceptron.predict(&test));
}
