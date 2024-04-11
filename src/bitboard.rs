use std::{ 
    fmt,
    ops::BitAnd,
    ops::BitOr,
    ops::BitXor,
    ops::Not
};

#[derive(Copy, Clone, PartialEq)]
pub struct Bitboard(u64);

#[allow(dead_code)]
impl Bitboard {
    pub fn new(val: u64) -> Bitboard {
        Self(val)
    }

    pub fn val(&self) -> u64 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        &self.0 == &0
    }

    pub fn nort_one(&self) -> Bitboard {
        Self(&self.0 >> 8)
    }
    
    pub fn east_one(&self) -> Bitboard {
        Self(&self.0 << 1) & !Self::A_FILE
    }
    
    pub fn sout_one(&self) -> Bitboard {
        Self(&self.0 << 8)
    }

    pub fn west_one(&self) -> Bitboard {
        Self(&self.0 >> 1) & !Self::H_FILE
    }
}

// BIT OPERATIONS :
impl BitAnd for Bitboard {
    type Output = Self;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Not for Bitboard {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        output.push_str("\n");
        let bytes = self.0.to_be_bytes();

        for i in 0..64 {
            let bit = (bytes[7 - (i / 8)] >> (i % 8)) % 2;

            output.push_str(&bit.to_string());

            output.push(' ');
            if i % 8 == 7 {
                output.push('\n');
            }
        }

        write!(f, "{}", output)
    }
}

//impl fmt::Debug for Bitboard {
//    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//        let mut output = String::new();
//        
//        for i in 1..=8 {
//            let val = (self.0 >> 64 - 8 * i) % 0x100;
//            output.push_str(format!("{:08b}\n", val).as_str());
//        }
//
//        write!(f, "{}", output)
//    }
//}

impl Bitboard {
    pub const EMPTY: Bitboard =             Bitboard(0);
    pub const UNIVERSE: Bitboard =          Bitboard(u64::MAX);

    pub const A_FILE: Bitboard =            Bitboard(0x01_01_01_01_01_01_01_01);
    pub const H_FILE: Bitboard =            Bitboard(0x80_80_80_80_80_80_80_80);

    pub const STARTING_WHITE: Bitboard =    Bitboard(0x00_00_00_00_00_00_ff_ff);
    pub const STARTING_BLACK: Bitboard =    Bitboard(0xff_ff_00_00_00_00_00_00);
    pub const STARTING_PAWNS: Bitboard =    Bitboard(0x00_ff_00_00_00_00_ff_00);
    pub const STARTING_KNIGHTS: Bitboard =  Bitboard(0x42_00_00_00_00_00_00_42);
    pub const STARTING_BISHOPS: Bitboard =  Bitboard(0x24_00_00_00_00_00_00_24);
    pub const STARTING_ROOKS: Bitboard =    Bitboard(0x81_00_00_00_00_00_00_81);
    pub const STARTING_QUEENS: Bitboard =   Bitboard(0x08_00_00_00_00_00_00_08);
    pub const STARTING_KINGS: Bitboard =    Bitboard(0x10_00_00_00_00_00_00_10);
}


// TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::*;

    #[test]
    fn test_directional_circle() {
        let mut bb: Bitboard = Square::D4.bb();

        bb = bb.nort_one();
        assert_eq!(bb, Square::D5.bb());
        bb = bb.east_one();
        assert_eq!(bb, Square::E5.bb());
        bb = bb.sout_one();
        assert_eq!(bb, Square::E4.bb());
        bb = bb.west_one();
        assert_eq!(bb, Square::D4.bb());
    }

    #[test]
    fn test_directional_overflow() {
        assert_eq!(Square::B8.bb().nort_one(), Bitboard::EMPTY);
        assert_eq!(Square::H5.bb().east_one(), Bitboard::EMPTY);
        assert_eq!(Square::D1.bb().sout_one(), Bitboard::EMPTY);
        assert_eq!(Square::A2.bb().west_one(), Bitboard::EMPTY);
    }
}
