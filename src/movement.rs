use super::square::Square;
use std::fmt;

pub struct Move(u16);

#[derive(Debug)]
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
}

impl Move {
    pub fn new(orig: Square, targ: Square, flag: Flag) -> Self {
        let mut output = (orig.val() as u16) << 10;
        output        |= (targ.val() as u16) << 4;
        output        |= flag        as u16;

        Self(output)
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:016b}", self.0)
    }
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Move({} to {}, {:?})",
            Square::new((self.0 >> 10) as u8).unwrap_or(Square::A1),
            Square::new((self.0 >> 4 ) as u8).unwrap_or(Square::A1),
            Flag::new((self.0 % 0b10000) as u8).unwrap()
            //self.0 % 0b10000
        )
    }
}
