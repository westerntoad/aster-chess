use super::square::Square;
use super::piece::Piece;
use std::fmt;

const ORIG_MASK: u16 = 0b1111_1100_0000_0000;
const TARG_MASK: u16 = 0b0000_0011_1111_0000;
const FLAG_MASK: u16 = 0b0000_0000_0000_1111;

#[derive(Clone, PartialEq)]
pub struct Move(u16);

#[derive(Debug, PartialEq)]
pub enum Flag {
    Quiet = 0b0000,
    DoublePush = 0b0001,
    ShortCastle = 0b0010,
    LongCastle = 0b0011,
    Capture = 0b0100,
    EnPassant = 0b0101,
    PromoteN = 0b1000,
    PromoteB = 0b1001,
    PromoteR = 0b1010,
    PromoteQ = 0b1011,
    PromoteCaptureN = 0b1100,
    PromoteCaptureB = 0b1101,
    PromoteCaptureR = 0b1110,
    PromoteCaptureQ = 0b1111,
}

impl Flag {
    pub fn new(val: u8) -> Result<Self, &'static str> {
        match val {
            0b0000 => Ok(Flag::Quiet),
            0b0001 => Ok(Flag::DoublePush),
            0b0010 => Ok(Flag::ShortCastle),
            0b0011 => Ok(Flag::LongCastle),
            0b0100 => Ok(Flag::Capture),
            0b0101 => Ok(Flag::EnPassant),
            0b1000 => Ok(Flag::PromoteN),
            0b1001 => Ok(Flag::PromoteB),
            0b1010 => Ok(Flag::PromoteR),
            0b1011 => Ok(Flag::PromoteQ),
            0b1100 => Ok(Flag::PromoteCaptureN),
            0b1101 => Ok(Flag::PromoteCaptureB),
            0b1110 => Ok(Flag::PromoteCaptureR),
            0b1111 => Ok(Flag::PromoteCaptureQ),
            _ => Err("Invalid move flag range.")
        }
    }

    pub fn promotion_piece(&self) -> Piece {
        match self {
            Flag::PromoteN | Flag::PromoteCaptureN => Piece::Knight,
            Flag::PromoteB | Flag::PromoteCaptureB => Piece::Bishop,
            Flag::PromoteR | Flag::PromoteCaptureR => Piece::Rook,
            Flag::PromoteQ | Flag::PromoteCaptureQ => Piece::Queen,
            _ => Piece::NoPiece
        }
    }
}

impl Move {
    // TODO remove
    pub const SHORT_CASTLE: Self = Self(0b0010);
    pub const LONG_CASTLE:  Self = Self(0b0011);

    pub const WHITE_SHORT_CASTLE: Self = Self(0x1062);
    pub const WHITE_LONG_CASTLE:  Self = Self(0x1023);
    pub const BLACK_SHORT_CASTLE: Self = Self(0xf3e2);
    pub const BLACK_LONG_CASTLE:  Self = Self(0xf3a3);

    pub fn new(orig: Square, targ: Square, flag: Flag) -> Self {
        let mut output = (orig.val() as u16) << 10;
        output        |= (targ.val() as u16) << 4;
        output        |= flag        as u16;

        Self(output)
    }

    pub fn orig(&self) -> Square {
        Square::new((self.0 >> 10) as u8).unwrap()
    }

    pub fn targ(&self) -> Square {
        Square::new(((self.0 & TARG_MASK ) >> 4) as u8).unwrap()
    }

    pub fn flag(&self) -> Flag {
        Flag::new((self.0 & FLAG_MASK) as u8).unwrap()
    }

    pub fn is_capture(&self) -> bool {
        self.0 & 0b100 != 0 || self.is_en_passant()
    }

    pub fn is_castle(&self) -> bool {
        self.0 & 0b1110 == 0b0010
    }

    pub fn is_promotion(&self) -> bool {
        self.0 & 0b1000 == 0b1000
    }

    pub fn is_en_passant(&self) -> bool {
        self.0 & 0b1111 == 0b0101
    }

    pub fn ep_square(&self) -> Option<Square> {
        if self.0 & 0b1111 == 0b0001 {
            let rank = match self.orig().rank() {
                1 => 2,
                6 => 5,
                _ => {
                    panic!("Invalid double push. Move={}", self)
                }
            };
            Some(Square::from_coord(rank, self.targ().file()).unwrap())
        } else {
            None
        }
    }

    pub fn promotion_piece(&self) -> Piece {
        self.flag().promotion_piece()
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*write!(f, "Move({} to {}, {:?})",
            self.orig(),
            self.targ(),
            self.flag()
        )*/

        let promote_char = match self.promotion_piece() {
            Piece::Knight => "n",
            Piece::Bishop => "b",
            Piece::Rook   => "r",
            Piece::Queen  => "q",
            _ => ""
        };

        write!(f, "{}{}{} {:?}",
            self.orig(),
            self.targ(),
            promote_char,
            self.flag()
        )
    }
}
