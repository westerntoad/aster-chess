use std::{ 
    fmt,
    ops::BitAnd,
    ops::BitOr,
    ops::BitXor,
    ops::Not
};

use super::square::Square;

#[derive(Copy, Clone, PartialEq)]
pub struct Bitboard(u64);

#[allow(dead_code)]
impl Bitboard {
    // constants
    pub const EMPTY: Bitboard =             Bitboard(0);
    pub const UNIVERSE: Bitboard =          Bitboard(u64::MAX);
    pub const STARTING_WHITE: Bitboard =    Bitboard(0x00_00_00_00_00_00_ff_ff);
    pub const STARTING_BLACK: Bitboard =    Bitboard(0xff_ff_00_00_00_00_00_00);
    pub const STARTING_PAWNS: Bitboard =    Bitboard(0x00_ff_00_00_00_00_ff_00);
    pub const STARTING_KNIGHTS: Bitboard =  Bitboard(0x42_00_00_00_00_00_00_42);
    pub const STARTING_BISHOPS: Bitboard =  Bitboard(0x24_00_00_00_00_00_00_24);
    pub const STARTING_ROOKS: Bitboard =    Bitboard(0x81_00_00_00_00_00_00_81);
    pub const STARTING_QUEENS: Bitboard =   Bitboard(0x08_00_00_00_00_00_00_08);
    pub const STARTING_KINGS: Bitboard =    Bitboard(0x10_00_00_00_00_00_00_10);

    pub fn from_sq(square: Square) -> Self {
        Self(u64::pow(2, square.val() as u32))
    }

    pub fn val(&self) -> u64 {
        self.0
    }

    pub fn is_empty(&self) -> bool {
        &self.0 == &0
    }

    pub fn nort_one(&self) -> Bitboard {
        Self(&self.0 << 8)
    }
    
    pub fn east_one(&self) -> Bitboard {
        Self(&self.0 >> 1)
    }
    
    pub fn sout_one(&self) -> Bitboard {
        Self(&self.0 >> 8)
    }

    pub fn west_one(&self) -> Bitboard {
        Self(&self.0 << 1)
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


// OTHER IMPL :
impl From<u64> for Bitboard {
    fn from(bb: u64) -> Self {
        Self(bb)
    }
}

impl fmt::Debug for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let bytes = self.0.to_be_bytes();

        for i in 0..64 {
            let bit = (bytes[i / 8] >> (i % 8)) % 2;

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



// TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_empty() {
        let empty = Bitboard::from(0);
        let not_empty_one = Bitboard::UNIVERSE;
        let not_empty_two = Bitboard::from(0xff_00_0f_00_0f_00_00_ff);
        let almost_empty = Bitboard::from(0x00_00_00_00_01_00_00_00);

        assert!(empty.is_empty());
        assert!(!not_empty_one.is_empty());
        assert!(!not_empty_two.is_empty());
        assert!(!almost_empty.is_empty());
    }
}
