use std::fmt;
#[derive(Debug)]
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

pub struct Bitboard {
    internal: [u8; 32],
}

impl Bitboard {
    pub fn new() -> Self {
        Self {
            internal: [
                0x42, 0x35, 0x63, 0x24,
                0x11, 0x11, 0x11, 0x11,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 
                0x99, 0x99, 0x99, 0x99,
                0xCA, 0xBE, 0xDB, 0xAC,
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
                write!(f, "{} ", Into::<char>::into(self.get_position((rank, file))))?;
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

// The rows are in the wrong order
// impl fmt::Debug for Bitboard {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for i in (0..self.internal.len()).rev() {
//             if (i + 1) % 4 == 0 {
//                 write!(f, "\n")?;
//             }
//             write!(f, "{:02X?}", self.internal[i])?;
//         }
//         write!(f, "\n\n")
//     }
// }
