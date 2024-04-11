//! Unsigned 8-bit integer interpreted as a single square on a chess board. The
//! elenents are indexed via a value as the following:
//!
//! 1   0  1  2  3  4  5  6  7
//! 2   8  9  10 11 12 13 14 15
//! 3   16 17 18 19 20 21 22 23
//! 4   24 25 26 27 28 29 30 31
//! 5   32 33 34 35 36 37 38 39
//! 6   40 41 42 43 44 45 46 47
//! 7   48 49 50 51 52 53 54 55
//! 8   56 57 58 59 60 61 62 63
//!
//!     a  b  c  d  e  f  g  h

use std::fmt;
use super::bitboard::Bitboard;

#[derive(Copy, Clone, PartialEq)]
pub struct Square(u8);

#[allow(dead_code)]
impl Square {
    pub fn new(val: u8) -> Result<Self, &'static str> {
        match val {
            0..=63 => Ok(Self(val)),
            _ => Err("Invalid square index."),
        }
    }

    pub fn from_coord(rank: u8, file: u8) -> Result<Self, &'static str> {
        Self::new(rank * 8 + file)
    }

    pub fn from_algebraic(val: &str) -> Result<Self, &'static str> {
        let file = match val.chars().nth(0) {
            Some(v) => (v as u8) - 0x61,
            None => return Err("Invalid file parameter"),
        };
        let rank = match val.chars().nth(1) {
            Some(v) => 8 - ((v as u8) - 0x30),
            None => return Err("Invalid rank parameter"),
        };

        Self::from_coord(rank, file)
    }

    pub fn bb(&self) -> Bitboard {
        Bitboard::new(u64::pow(2, self.0 as u32))
    }

    pub fn val(&self) -> u8 {
        self.0
    }
    
    pub fn rank(&self) -> u8 {
        self.0 / 8
    }
    
    pub fn file(&self) -> u8 {
        self.0 % 8
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
        // old implementation:
        //write!(f, "{:06b}", self.0)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = match self.file() {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!()
        };

        write!(f, "{}{}", file, 8 - self.rank())
    }
}

#[allow(dead_code)]
impl Square {
    pub const A8: Square = Self(0);
    pub const B8: Square = Self(1);
    pub const C8: Square = Self(2);
    pub const D8: Square = Self(3);
    pub const E8: Square = Self(4);
    pub const F8: Square = Self(5);
    pub const G8: Square = Self(6);
    pub const H8: Square = Self(7);

    pub const A7: Square = Self(8);
    pub const B7: Square = Self(9);
    pub const C7: Square = Self(10);
    pub const D7: Square = Self(11);
    pub const E7: Square = Self(12);
    pub const F7: Square = Self(13);
    pub const G7: Square = Self(14);
    pub const H7: Square = Self(15);

    pub const A6: Square = Self(16);
    pub const B6: Square = Self(17);
    pub const C6: Square = Self(18);
    pub const D6: Square = Self(19);
    pub const E6: Square = Self(20);
    pub const F6: Square = Self(21);
    pub const G6: Square = Self(22);
    pub const H6: Square = Self(23);

    pub const A5: Square = Self(24);
    pub const B5: Square = Self(25);
    pub const C5: Square = Self(26);
    pub const D5: Square = Self(27);
    pub const E5: Square = Self(28);
    pub const F5: Square = Self(29);
    pub const G5: Square = Self(30);
    pub const H5: Square = Self(31);

    pub const A4: Square = Self(32);
    pub const B4: Square = Self(33);
    pub const C4: Square = Self(34);
    pub const D4: Square = Self(35);
    pub const E4: Square = Self(36);
    pub const F4: Square = Self(37);
    pub const G4: Square = Self(38);
    pub const H4: Square = Self(39);

    pub const A3: Square = Self(40);
    pub const B3: Square = Self(41);
    pub const C3: Square = Self(42);
    pub const D3: Square = Self(43);
    pub const E3: Square = Self(44);
    pub const F3: Square = Self(45);
    pub const G3: Square = Self(46);
    pub const H3: Square = Self(47);

    pub const A2: Square = Self(48);
    pub const B2: Square = Self(49);
    pub const C2: Square = Self(50);
    pub const D2: Square = Self(51);
    pub const E2: Square = Self(52);
    pub const F2: Square = Self(53);
    pub const G2: Square = Self(54);
    pub const H2: Square = Self(55);

    pub const A1: Square = Self(56);
    pub const B1: Square = Self(57);
    pub const C1: Square = Self(58);
    pub const D1: Square = Self(59);
    pub const E1: Square = Self(60);
    pub const F1: Square = Self(61);
    pub const G1: Square = Self(62);
    pub const H1: Square = Self(63);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let sq1: Square = match Square::new(3) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        let sq2: Square = match Square::new(36) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        let sq3: Square = match Square::new(0) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        let sq4: Square = match Square::new(63) {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        assert_eq!(sq1, Square::D8);
        assert_eq!(sq2, Square::E4);
        assert_eq!(sq3, Square::A8);
        assert_eq!(sq4, Square::H1);
    }

    #[test]
    fn test_invalid_new() {
        assert!(!Square::new(0).is_err());
        assert!(!Square::new(49).is_err());
        assert!(Square::new(150).is_err());
        assert!(Square::new(64).is_err());
        assert!(Square::new(u8::MAX).is_err());
    }

    #[test]
    fn test_from_algebraic() {
        assert_eq!(Square::from_algebraic("e4").unwrap(), Square::E4);
        assert_eq!(Square::from_algebraic("c6").unwrap(), Square::C6);
        assert_eq!(Square::from_algebraic("a1").unwrap(), Square::A1);
        assert_eq!(Square::from_algebraic("h8").unwrap(), Square::H8);
    }

    #[test]
    fn test_val() {
        let sq1: Square = match Square::new(18) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        let sq2: Square = match Square::new(27) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        let sq3: Square = match Square::new(0) {
            Ok(v) => v,
            Err(_) => panic!(),
        };
        let sq4: Square = match Square::new(63) {
            Ok(v) => v,
            Err(_) => panic!(),
        };

        assert_eq!(sq1.val(), Square::C6.val());
        assert_eq!(sq2.val(), 27);
        assert_eq!(sq3.val(), Square::A8.val());
        assert_eq!(sq4.val(), Square::H1.val());
        assert_eq!(Square::D4.val(), 35);
    }

    #[test]
    fn test_invalid_val() {
        assert!(!Square::new(0).is_err());
        assert!(!Square::new(20).is_err());
        assert!(Square::new(97).is_err());
        assert!(Square::new(64).is_err());
        assert!(Square::new(u8::MAX).is_err());
    }

    #[test]
    fn test_rank() {
        assert_eq!(Square::E3.rank(), 5);
        assert_eq!(Square::B8.rank(), 0);
        assert_eq!(Square::C1.rank(), 7);
    }

    #[test]
    fn test_file() {
        assert_eq!(Square::G3.file(), 6);
        assert_eq!(Square::A4.file(), 0);
        assert_eq!(Square::H6.file(), 7);
    }

    #[test]
    fn test_debug_print() {
        assert_eq!(format!("{:?}", Square::D4), "35");
        assert_eq!(format!("{:?}", Square::G7), "14");
        assert_eq!(format!("{:?}", Square::A8), "0");
        assert_eq!(format!("{:?}", Square::H1), "63");
    }

    #[test]
    fn test_display_print() {
        assert_eq!(format!("{}", Square::E4), "e4");
        assert_eq!(format!("{}", Square::B2), "b2");
        assert_eq!(format!("{}", Square::A8), "a8");
        assert_eq!(format!("{}", Square::H1), "h1");
    }
}
