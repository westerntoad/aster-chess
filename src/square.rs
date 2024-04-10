use std::fmt;

#[derive(Copy, Clone)]
pub struct Square(u8);

#[allow(dead_code)]
impl Square {
    // Constants
    pub const A1: Square = Self(0);
    pub const A2: Square = Self(1);
    pub const A3: Square = Self(2);
    pub const A4: Square = Self(3);
    pub const A5: Square = Self(4);
    pub const A6: Square = Self(5);
    pub const A7: Square = Self(6);
    pub const A8: Square = Self(7);

    pub const B1: Square = Self(8);
    pub const B2: Square = Self(9);
    pub const B3: Square = Self(10);
    pub const B4: Square = Self(11);
    pub const B5: Square = Self(12);
    pub const B6: Square = Self(13);
    pub const B7: Square = Self(14);
    pub const B8: Square = Self(15);

    pub const C1: Square = Self(16);
    pub const C2: Square = Self(17);
    pub const C3: Square = Self(18);
    pub const C4: Square = Self(19);
    pub const C5: Square = Self(20);
    pub const C6: Square = Self(21);
    pub const C7: Square = Self(22);
    pub const C8: Square = Self(23);

    pub const D1: Square = Self(24);
    pub const D2: Square = Self(25);
    pub const D3: Square = Self(26);
    pub const D4: Square = Self(27);
    pub const D5: Square = Self(28);
    pub const D6: Square = Self(29);
    pub const D7: Square = Self(30);
    pub const D8: Square = Self(31);

    pub const E1: Square = Self(32);
    pub const E2: Square = Self(33);
    pub const E3: Square = Self(34);
    pub const E4: Square = Self(35);
    pub const E5: Square = Self(36);
    pub const E6: Square = Self(37);
    pub const E7: Square = Self(38);
    pub const E8: Square = Self(39);

    pub const F1: Square = Self(40);
    pub const F2: Square = Self(41);
    pub const F3: Square = Self(42);
    pub const F4: Square = Self(43);
    pub const F5: Square = Self(44);
    pub const F6: Square = Self(45);
    pub const F7: Square = Self(46);
    pub const F8: Square = Self(47);

    pub const G1: Square = Self(48);
    pub const G2: Square = Self(49);
    pub const G3: Square = Self(50);
    pub const G4: Square = Self(51);
    pub const G5: Square = Self(52);
    pub const G6: Square = Self(53);
    pub const G7: Square = Self(54);
    pub const G8: Square = Self(55);

    pub const H1: Square = Self(56);
    pub const H2: Square = Self(57);
    pub const H3: Square = Self(58);
    pub const H4: Square = Self(59);
    pub const H5: Square = Self(60);
    pub const H6: Square = Self(61);
    pub const H7: Square = Self(62);
    pub const H8: Square = Self(63);
    
    pub fn new(val: u8) -> Result<Self, &'static str> {
        match val {
            0..=63 => Ok(Self(val)),
            _ => Err("Invalid square index."),
        }
    }

    pub fn from_coord(row: u8, column: u8) -> Result<Self, &'static str> {
        Self::new(row + column * 8)
    }

    pub fn val(&self) -> u8 {
        self.0
    }

    pub fn file(&self) -> u8 {
        self.0 / 8
    }

    pub fn rank(&self) -> u8 {
        self.0 % 8
    }
}

impl fmt::Debug for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:06b}", self.0)
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

        write!(f, "{}{}", file, self.rank() + 1)
    }
}
