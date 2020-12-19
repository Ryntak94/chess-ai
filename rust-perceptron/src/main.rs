mod bitboard;
mod perceptron;
mod vec_math;

use {
    bitboard::{Bitboard, ChessPiece},
    perceptron::Perceptron,
    rand,
};

// fn main() {
//     let mut perceptron = Perceptron::from_rng(
//         vec![
//             vec![0.0, 0.0, 1.0],
//             vec![1.0, 1.0, 1.0],
//             vec![1.0, 0.0, 1.0],
//             vec![0.0, 1.0, 0.0]
//         ],
//         vec![0.0, 1.0, 1.0, 0.0],
//         0.0,
//         1_000,
//         false,
//         rand::thread_rng()
//     );
//     perceptron.train();

//     let test = vec![1.0, 1.0, 1.0];
//     println!("Prediction of {:?} : {}", &test, perceptron.predict(&test));
// }

fn main() {
    let mut bitboard = Bitboard::new();
    println!("{}", &bitboard);
    bitboard.move_piece((3, 1), (3, 3));
    println!("{}", &bitboard);
    bitboard.move_piece((3, 6), (3, 4));
    println!("{}", &bitboard);
    bitboard.move_piece((4, 1), (4, 3));
    println!("{}", &bitboard);
    let maybe_capture = bitboard.move_piece((3, 4), (4, 3));
    println!("{}", &bitboard);
    assert!(maybe_capture.is_some());

    println!("{:?}", &bitboard);
}
