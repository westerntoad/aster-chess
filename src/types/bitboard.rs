use std::{ 
    fmt,
    ops::BitAnd,
    ops::BitAndAssign,
    ops::BitOr,
    ops::BitOrAssign,
    ops::BitXor,
    ops::Not
};
use crate::types::square::Square;

#[derive(Copy, Clone, PartialEq)]
pub struct Bitboard(pub u64);

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

    pub fn is_one(&self) -> bool {
        self.0.is_power_of_two()
    }

    pub fn nort_one(&self) -> Bitboard {
        Self(&self.0 << 8)
    }
    
    pub fn east_one(&self) -> Bitboard {
        Self(&self.0 << 1) & !Self::A_FILE
    }
    
    pub fn sout_one(&self) -> Bitboard {
        Self(&self.0 >> 8)
    }

    pub fn west_one(&self) -> Bitboard {
        Self(&self.0 >> 1) & !Self::H_FILE
    }

    pub const fn index(&self) -> usize {
        self.0.trailing_zeros() as usize
    }

    pub fn rank(&self) -> u64 {
        (&self.0.ilog2() / 8).into()
    }

    pub fn file(&self) -> u64 {
        (self.0.ilog2() % 8).into()
    }
    
    pub fn pop_lsb(&mut self) -> Square {
        let val = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;

        Square::new_unchecked(val)
    }
}

impl Iterator for Bitboard {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 { return None; };
        //let ls1b = self.0 & (!self.0 + 1);
        //self.0 = self.0 & !ls1b;

        Some(self.pop_lsb())
    }
}

// BIT OPERATIONS :
impl BitAnd for Bitboard {
    type Output = Self;
    
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
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
            let bit = (bytes[i / 8] >> (i % 8)) % 2;

            output.push_str(&bit.to_string());

            output.push(' ');
            if i % 8 == 7 {
                output.push('\n');
            }
        }

        output.push_str(&format!("\nHex:\n{:016x}", self.0));
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

    pub const A_FILE: Bitboard =            Bitboard(0x0101010101010101);
    pub const H_FILE: Bitboard =            Bitboard(0x8080808080808080);

    pub const RANK_1: Bitboard =            Bitboard(0x00000000000000ff);
    pub const RANK_2: Bitboard =            Bitboard(0x000000000000ff00);
    pub const RANK_3: Bitboard =            Bitboard(0x0000000000ff0000);
    pub const RANK_4: Bitboard =            Bitboard(0x00000000ff000000);
    pub const RANK_5: Bitboard =            Bitboard(0x000000ff00000000);
    pub const RANK_6: Bitboard =            Bitboard(0x0000ff0000000000);
    pub const RANK_7: Bitboard =            Bitboard(0x00ff000000000000);
    pub const RANK_8: Bitboard =            Bitboard(0xff00000000000000);

    pub const STARTING_WHITE: Bitboard =    Bitboard(0x000000000000ffff);
    pub const STARTING_BLACK: Bitboard =    Bitboard(0xffff000000000000);
    pub const STARTING_PAWNS: Bitboard =    Bitboard(0x00ff00000000ff00);
    pub const STARTING_KNIGHTS: Bitboard =  Bitboard(0x4200000000000042);
    pub const STARTING_BISHOPS: Bitboard =  Bitboard(0x2400000000000024);
    pub const STARTING_ROOKS: Bitboard =    Bitboard(0x8100000000000081);
    pub const STARTING_QUEENS: Bitboard =   Bitboard(0x0800000000000008);
    pub const STARTING_KINGS: Bitboard =    Bitboard(0x1000000000000010);

    pub const WK_MASK: Bitboard =           Bitboard(0x0000000000000060);
    pub const WQ_MASK: Bitboard =           Bitboard(0x000000000000000e);
    pub const BK_MASK: Bitboard =           Bitboard(0x6000000000000000);
    pub const BQ_MASK: Bitboard =           Bitboard(0x0e00000000000000);
    pub const WQ_MASK_SLIDE: Bitboard =     Bitboard(0x000000000000000c);
    pub const BQ_MASK_SLIDE: Bitboard =     Bitboard(0x0c00000000000000);

    pub const WK_ROOK: Bitboard =           Bitboard(0x0000000000000080);
    pub const WQ_ROOK: Bitboard =           Bitboard(0x0000000000000001);
    pub const BK_ROOK: Bitboard =           Bitboard(0x8000000000000000);
    pub const BQ_ROOK: Bitboard =           Bitboard(0x0100000000000000);

    pub const W_KING: Bitboard =            Bitboard(0x0000000000000010);
    pub const B_KING: Bitboard =            Bitboard(0x1000000000000000);

    pub const WK_ROOK_DEST: Bitboard =      Bitboard(0x0000000000000020);
    pub const WQ_ROOK_DEST: Bitboard =      Bitboard(0x0000000000000008);
    pub const BK_ROOK_DEST: Bitboard =      Bitboard(0x2000000000000000);
    pub const BQ_ROOK_DEST: Bitboard =      Bitboard(0x0800000000000000);

    pub const WK_KING_DEST: Bitboard =      Bitboard(0x0000000000000040);
    pub const WQ_KING_DEST: Bitboard =      Bitboard(0x0000000000000004);
    pub const BK_KING_DEST: Bitboard =      Bitboard(0x4000000000000000);
    pub const BQ_KING_DEST: Bitboard =      Bitboard(0x0400000000000000);
}


// TESTS
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::square::*;

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
