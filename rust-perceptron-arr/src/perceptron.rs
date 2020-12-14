use {
    rand::{
        Rng,
        distributions::Uniform,
    },
    std::time::{
        SystemTime,
        UNIX_EPOCH
    }
};
use crate::vec_math::{
    sigmoid,
    dot_product,
    vector_add_mut,
    scalar_multiply,
    scalar_multiply_mut,
};

pub struct Perceptron {
    input: [[f64; 3]; 4],
	actual_output: [f64; 4],
	weights: [f64; 3],
	bias: f64,
    epochs: usize,
    terminate_early: bool,
}
const EPOCH_DEBUG: bool = false;

impl Perceptron {
    fn forward_pass(&self, vec: &[f64; 3]) -> f64 {
        sigmoid(dot_product(&self.weights, vec) + self.bias)
    }

    fn bias_gradient(&self, predicted: f64, actual: f64) -> f64 {
        -(predicted - actual) * predicted * (1.0 - predicted)
    }

    fn epoch_info(&self, epoch: usize, d_weights: &[f64; 3]) {
        if EPOCH_DEBUG {
            println!(
                "\r\nEpoch {} / {}\r\n  Bias:    {}\r\n  Weights: {:?}\r\n  d_weights: {:?}",
                epoch,
                self.epochs,
                self.bias,
                self.weights,
                d_weights
            );
        }
    }
 
    fn terminate_early(&self, d_weights: &[f64; 3]) -> bool {
        self.terminate_early
            && self
                .weights
                .iter()
                .enumerate()
                .fold(false, |acc, (i, weight)| acc || (d_weights[i] / weight).abs() < 0.0001)
    }
    
    pub fn from_rng<R: Rng>(
        input: [[f64; 3]; 4],
        actual_output: [f64; 4],
        bias: f64,
        epochs: usize,
        terminate_early: bool,
        rng: &mut R,  
    ) -> Self {
        Self {
            input,
            actual_output,
            weights: [
                rng.sample(Uniform::new_inclusive(0.0, 1.0)),
                rng.sample(Uniform::new_inclusive(0.0, 1.0)),
                rng.sample(Uniform::new_inclusive(0.0, 1.0)),
            ],
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
            let mut d_weights = [0.0, 0.0, 0.0];

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

    pub fn predict(&self, vec: &[f64; 3]) -> f64 {
        sigmoid(dot_product(vec, &self.weights))
    }
}