pub mod bitboard;
pub mod perceptron;
pub mod vec_math;

use {
    bitboard::{Bitboard, ChessPiece},
    perceptron::Perceptron,
    rand,
};

fn main() {
    bitboard_example();
    perceptron_example();
}

fn bitboard_example() {
    let mut bitboard = Bitboard::new();
    let maybe_capture: Option<ChessPiece> = bitboard.move_piece((3, 1), (3, 3));
    assert!(maybe_capture.is_none());
    println!("{}", &bitboard);
}

fn perceptron_example() {
    let mut perceptron = Perceptron::from_rng(
        vec![
            vec![0.0, 0.0, 1.0],
            vec![1.0, 1.0, 1.0],
            vec![1.0, 0.0, 1.0],
            vec![0.0, 1.0, 0.0],
        ],
        vec![0.0, 1.0, 1.0, 0.0],
        0.0,
        1_000,
        false,
        rand::thread_rng(),
    );
    perceptron.train();

    let test = vec![1.0, 1.0, 1.0];
    println!("Prediction of {:?} : {}", &test, perceptron.predict(&test));
}
