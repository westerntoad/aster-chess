
#[derive(Debug, PartialEq)]
pub enum Piece {
    NoPiece = 0b000,
    Pawn = 0b001,
    Knight = 0b010,
    Bishop = 0b011,
    Rook = 0b100,
    Queen = 0b101,
    King = 0b110
}

impl Piece {
    pub fn new(val: u8) -> Result<Self, &'static str> {
        match val {
            0b000 => Ok(Piece::NoPiece),
            0b001 => Ok(Piece::Pawn),
            0b010 => Ok(Piece::Knight),
            0b011 => Ok(Piece::Bishop),
            0b100 => Ok(Piece::Rook),
            0b101 => Ok(Piece::Queen),
            0b110 => Ok(Piece::King),
            _ => Err("Invalid piece creation range.")
        }
    }

    pub fn val(&self) -> u8 {
        match self {
            Piece::NoPiece => 0b000,
            Piece::Pawn    => 0b001,
            Piece::Knight  => 0b010,
            Piece::Bishop  => 0b011,
            Piece::Rook    => 0b100,
            Piece::Queen   => 0b101,
            Piece::King    => 0b110
        }
    }

    pub fn is_piece(&self) -> bool {
        match self {
            Piece::NoPiece => false,
            _ => true
        }
    }
    
    pub fn index(&self) -> usize {
        match self {
            Piece::NoPiece => panic!("Cannot index by no piece"),
            _ => self.val() as usize - 1
        }
    }
}
