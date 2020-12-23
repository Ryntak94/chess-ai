use std::fmt;
#[derive(Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ChessPiece {
    NoPiece = 0x00,
    WhitePawn = 0x01,
    WhiteKnight = 0x02,
    WhiteBishop = 0x03,
    WhiteRook = 0x04,
    WhiteQueen = 0x05,
    WhiteKing = 0x06,
    BlackPawn = 0x09,
    BlackKnight = 0x0A,
    BlackBishop = 0x0B,
    BlackRook = 0x0C,
    BlackQueen = 0x0D,
    BlackKing = 0x0E,
}

impl From<u8> for ChessPiece {
    fn from(byte: u8) -> Self {
        match byte {
            0x00 => Self::NoPiece,
            0x01 => Self::WhitePawn,
            0x02 => Self::WhiteKnight,
            0x03 => Self::WhiteBishop,
            0x04 => Self::WhiteRook,
            0x05 => Self::WhiteQueen,
            0x06 => Self::WhiteKing,
            0x09 => Self::BlackPawn,
            0x0A => Self::BlackKnight,
            0x0B => Self::BlackBishop,
            0x0C => Self::BlackRook,
            0x0D => Self::BlackQueen,
            0x0E => Self::BlackKing,
            _ => panic!("Invalid chess representation: 0x{:02X}", byte),
        }
    }
}

impl From<ChessPiece> for u8 {
    fn from(piece: ChessPiece) -> Self {
        match piece {
            ChessPiece::NoPiece => 0x00,
            ChessPiece::WhitePawn => 0x01,
            ChessPiece::WhiteKnight => 0x02,
            ChessPiece::WhiteBishop => 0x03,
            ChessPiece::WhiteRook => 0x04,
            ChessPiece::WhiteQueen => 0x05,
            ChessPiece::WhiteKing => 0x06,
            ChessPiece::BlackPawn => 0x09,
            ChessPiece::BlackKnight => 0x0A,
            ChessPiece::BlackBishop => 0x0B,
            ChessPiece::BlackRook => 0x0C,
            ChessPiece::BlackQueen => 0x0D,
            ChessPiece::BlackKing => 0x0E,
        }
    }
}

// Unicode values are the opposite of what they should be.
// (White --> Black, Black --> White)
// Maybe this is a windows specific issue?
impl From<ChessPiece> for char {
    fn from(piece: ChessPiece) -> Self {
        match piece {
            ChessPiece::NoPiece => '\u{0020}',
            ChessPiece::WhitePawn => '\u{265F}',
            ChessPiece::WhiteKnight => '\u{265E}',
            ChessPiece::WhiteBishop => '\u{265D}',
            ChessPiece::WhiteRook => '\u{265C}',
            ChessPiece::WhiteQueen => '\u{265B}',
            ChessPiece::WhiteKing => '\u{265A}',
            ChessPiece::BlackPawn => '\u{2659}',
            ChessPiece::BlackKnight => '\u{2658}',
            ChessPiece::BlackBishop => '\u{2657}',
            ChessPiece::BlackRook => '\u{2656}',
            ChessPiece::BlackQueen => '\u{2655}',
            ChessPiece::BlackKing => '\u{2654}',
        }
    }
}

/// Represents a chess board as nibbles
#[derive(PartialEq, Eq)]
pub struct Bitboard {
    internal: [u8; 32],
}

impl Bitboard {
    pub fn new() -> Self {
        Self {
            #[rustfmt::skip]
            internal: [
                0x42, 0x35, 0x63, 0x24,
                0x11, 0x11, 0x11, 0x11,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x99, 0x99, 0x99, 0x99,
                0xCA, 0xBD, 0xEB, 0xAC,
            ],
        }
    }

    pub fn from_bytes(internal: [u8; 32]) -> Self {
        Self {
            #[rustfmt::skip]
            internal,
        }
    }

    pub fn blank() -> Self {
        Self {
            #[rustfmt::skip]
            internal: [
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ],
        }
    }

    pub fn set_position(&mut self, position: (usize, usize), piece: ChessPiece) {
        let position = position.1 * 8 + position.0;
        let is_low_nibble = (position % 2 == 0) as usize * 4;
        let new_byte = Into::<u8>::into(piece) << is_low_nibble;
        let old_byte = self.internal[position / 2] & (0xF0 >> is_low_nibble);
        self.internal[position / 2] = new_byte | old_byte;
    }

    pub fn get_position(&self, position: (usize, usize)) -> ChessPiece {
        let position = position.1 * 8 + position.0;
        let is_low_nibble = (position % 2 == 0) as usize * 4;
        Into::into(self.internal[position / 2] >> is_low_nibble & 0x0F)
    }

    pub fn move_piece(&mut self, begin: (usize, usize), end: (usize, usize)) -> Option<ChessPiece> {
        let old_piece = self.get_position(begin);
        let occupied = self.get_position(end);

        self.set_position(end, old_piece);
        self.set_position(begin, ChessPiece::NoPiece);

        match occupied {
            ChessPiece::NoPiece => None,
            piece => Some(piece),
        }
    }
}

// Add color to me?
impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for file in (0..8).rev() {
            write!(f, "{} ", file + 1)?;
            for rank in 0..8 {
                write!(
                    f,
                    "{} ",
                    Into::<char>::into(self.get_position((rank, file)))
                )?;
            }
            write!(f, "\n")?;
        }
        write!(f, "  A B C D E F G H\n")?;
        fmt::Result::Ok(())
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for file in (0..8).rev() {
            for rank in 0..8 {
                let position = file * 8 + rank;
                if position % 2 == 0 {
                    write!(f, "{:02X?}", self.internal[position / 2])?;
                }
            }
            write!(f, "\n")?;
        }
        fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{Bitboard, ChessPiece};

    #[test]
    fn initializers() {
        let mut actual = Bitboard::blank();
        {
            // Block for collapsing this section
            actual.set_position((0, 0), ChessPiece::WhiteRook);
            actual.set_position((1, 0), ChessPiece::WhiteKnight);
            actual.set_position((2, 0), ChessPiece::WhiteBishop);
            actual.set_position((3, 0), ChessPiece::WhiteQueen);
            actual.set_position((4, 0), ChessPiece::WhiteKing);
            actual.set_position((5, 0), ChessPiece::WhiteBishop);
            actual.set_position((6, 0), ChessPiece::WhiteKnight);
            actual.set_position((7, 0), ChessPiece::WhiteRook);
            actual.set_position((0, 1), ChessPiece::WhitePawn);
            actual.set_position((1, 1), ChessPiece::WhitePawn);
            actual.set_position((2, 1), ChessPiece::WhitePawn);
            actual.set_position((3, 1), ChessPiece::WhitePawn);
            actual.set_position((4, 1), ChessPiece::WhitePawn);
            actual.set_position((5, 1), ChessPiece::WhitePawn);
            actual.set_position((6, 1), ChessPiece::WhitePawn);
            actual.set_position((7, 1), ChessPiece::WhitePawn);
            actual.set_position((0, 6), ChessPiece::BlackPawn);
            actual.set_position((1, 6), ChessPiece::BlackPawn);
            actual.set_position((2, 6), ChessPiece::BlackPawn);
            actual.set_position((3, 6), ChessPiece::BlackPawn);
            actual.set_position((4, 6), ChessPiece::BlackPawn);
            actual.set_position((5, 6), ChessPiece::BlackPawn);
            actual.set_position((6, 6), ChessPiece::BlackPawn);
            actual.set_position((7, 6), ChessPiece::BlackPawn);
            actual.set_position((0, 7), ChessPiece::BlackRook);
            actual.set_position((1, 7), ChessPiece::BlackKnight);
            actual.set_position((2, 7), ChessPiece::BlackBishop);
            actual.set_position((3, 7), ChessPiece::BlackQueen);
            actual.set_position((4, 7), ChessPiece::BlackKing);
            actual.set_position((5, 7), ChessPiece::BlackBishop);
            actual.set_position((6, 7), ChessPiece::BlackKnight);
            actual.set_position((7, 7), ChessPiece::BlackRook);
        }
        let expected = Bitboard::new();
        assert_eq!(actual, expected, "Default board initializer");

        #[rustfmt::skip]
        let actual = Bitboard::from_bytes([
            0x42, 0x35, 0x63, 0x24,
            0x11, 0x11, 0x11, 0x11,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x99, 0x99, 0x99, 0x99,
            0xCA, 0xBD, 0xEB, 0xAC,
        ]);
        let expected = Bitboard::new();
        assert_eq!(actual, expected, "From bytes initializer");
    }

    #[test]
    fn get_position() {
        let actual = Bitboard::new().get_position((0, 0));
        let expected = ChessPiece::WhiteRook;
        assert_eq!(actual, expected, "Get high nibble");

        let actual = Bitboard::new().get_position((1, 0));
        let expected = ChessPiece::WhiteKnight;
        assert_eq!(actual, expected, "Get low nibble");
    }

    #[test]
    fn set_position() {
        #[rustfmt::skip]
        let expected = Bitboard::from_bytes([
            0x10, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);
        let mut actual = Bitboard::blank();
        actual.set_position((0, 0), ChessPiece::WhitePawn);
        assert_eq!(actual, expected, "Set high nibble");

        #[rustfmt::skip]
        let expected = Bitboard::from_bytes([
            0x09, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);
        let mut actual = Bitboard::blank();
        actual.set_position((1, 0), ChessPiece::BlackPawn);
        assert_eq!(actual, expected, "Set low nibble");

        let mut actual = Bitboard::blank();
        actual.set_position((0, 0), ChessPiece::WhitePawn);
        actual.set_position((1, 0), ChessPiece::BlackPawn);
        #[rustfmt::skip]
        let expected = Bitboard::from_bytes([
            0x19, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);
        assert_eq!(actual, expected, "Set low nibble then high nibble");

        let mut actual = Bitboard::blank();
        actual.set_position((1, 0), ChessPiece::WhitePawn);
        actual.set_position((0, 0), ChessPiece::BlackPawn);
        #[rustfmt::skip]
        let expected = Bitboard::from_bytes([
            0x91, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);
        assert_eq!(actual, expected, "Set high nibble then low nibble");
    }

    #[test]
    fn move_piece() {
        #[rustfmt::skip]
        let mut actual = Bitboard::from_bytes([
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x90, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);
        actual.move_piece((4, 4), (4, 3));
        let actual = actual.move_piece((3, 1), (4, 3)).unwrap();
        let expected = ChessPiece::BlackPawn;
        assert_eq!(actual, expected, "Returns captured piece");

        #[rustfmt::skip]
        let mut actual = Bitboard::from_bytes([
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x90, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);
        actual.move_piece((4, 4), (4, 3));
        actual.move_piece((3, 2), (4, 3));
        #[rustfmt::skip]
        let expected = Bitboard::from_bytes([
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x10, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);
        assert_eq!(actual, expected, "Unsets previous position");
    }

    #[test]
    fn debug() {
        let actual = format!("{:?}", Bitboard::new());
        #[rustfmt::skip]
        let expected = { r#"CABDEBAC
99999999
00000000
00000000
00000000
00000000
11111111
42356324
"#      };
        assert_eq!(actual, expected, "Renders properly");
    }

    #[test]
    fn display() {
        let actual = format!("{}", Bitboard::new());
        #[rustfmt::skip]
        let expected = { r#"8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
6                 
5                 
4                 
3                 
2 ♟ ♟ ♟ ♟ ♟ ♟ ♟ ♟ 
1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
  A B C D E F G H
"#      };
        assert_eq!(actual, expected, "Renders properly");
    }
}
