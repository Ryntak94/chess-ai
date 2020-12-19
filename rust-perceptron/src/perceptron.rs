use crate::vec_math::{
    dot_product,
    scalar_multiply,
    scalar_multiply_mut,
    sigmoid,
    // vector_add,
    vector_add_mut,
};
use {
    rand::{distributions::Uniform, Rng},
    std::time::{SystemTime, UNIX_EPOCH},
};

pub struct Perceptron {
    input: Vec<Vec<f64>>,
    actual_output: Vec<f64>,
    weights: Vec<f64>,
    bias: f64,
    epochs: usize,
    terminate_early: bool,
}
const EPOCH_DEBUG: bool = false;

impl Perceptron {
    fn forward_pass(&self, vec: &Vec<f64>) -> f64 {
        sigmoid(dot_product(&self.weights, vec) + self.bias)
    }

    fn bias_gradient(&self, predicted: f64, actual: f64) -> f64 {
        -(predicted - actual) * predicted * (1.0 - predicted)
    }

    fn epoch_info(&self, epoch: usize, d_weights: &Vec<f64>) {
        if EPOCH_DEBUG {
            println!(
                "\r\nEpoch {} / {}\r\n  Bias:    {}\r\n  Weights: {:?}\r\n  d_weights: {:?}",
                epoch, self.epochs, self.bias, self.weights, d_weights
            );
        }
    }

    fn terminate_early(&self, d_weights: &Vec<f64>) -> bool {
        self.terminate_early
            && self
                .weights
                .iter()
                .enumerate()
                .fold(false, |acc, (i, weight)| {
                    acc || (d_weights[i] / weight).abs() < 0.0001
                })
    }

    pub fn from_rng<R: Rng>(
        input: Vec<Vec<f64>>,
        actual_output: Vec<f64>,
        bias: f64,
        epochs: usize,
        terminate_early: bool,
        rng: R,
    ) -> Self {
        let weights_len = input.get(0).unwrap_or(&vec![]).len();
        Self {
            input,
            actual_output,
            weights: rng
                .sample_iter(Uniform::new_inclusive(0.0, 1.0))
                .take(weights_len)
                .collect(),
            bias,
            epochs,
            terminate_early,
        }
    }

    pub fn train(&mut self) {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        for epoch in 0..self.epochs {
            let mut d_bias = 0.0;
            let mut d_weights = self.weights.iter().map(|_| 0.0).collect();

            for (i, x) in self.input.iter().enumerate() {
                let predicted = self.forward_pass(x);
                d_bias += self.bias_gradient(predicted, self.actual_output[i]);
                // d_weights = vector_add(&d_weights, &scalar_multiply(x, d_bias));
                vector_add_mut(&mut d_weights, &scalar_multiply(x, d_bias));
            }
            // d_weights = scalar_multiply(&d_weights, 2.0 / self.actual_output.len() as f64);
            scalar_multiply_mut(&mut d_weights, 2.0 / self.actual_output.len() as f64);

            // self.weights = vector_add(&self.weights, &d_weights);
            vector_add_mut(&mut self.weights, &d_weights);
            self.bias += d_bias * 2.0 / self.actual_output.len() as f64;
            self.epoch_info(epoch, &d_weights);

            if self.terminate_early(&d_weights) {
                break;
            }
        }

        let stop_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        println!("\r\nTraining duration: {:?}", stop_time - start_time);
    }

    pub fn predict(&self, vec: &Vec<f64>) -> f64 {
        sigmoid(dot_product(vec, &self.weights))
    }
}
