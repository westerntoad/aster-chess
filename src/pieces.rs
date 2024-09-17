use crate::bitboard::Bitboard;
use crate::square::Square;

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

pub const KNIGHT_TABLE: [Bitboard; 64] = {
    // this table initialization method took heavy inspiration from analog-hors/cozy-chess
    // https://github.com/analog-hors/cozy-chess/blob/1d8a04d1510071931132ce11e988ced0d41807c8/cozy-chess/src/moves.rs#L261
    let mut table: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    let deltas: [(i8, i8); 8] = [
        (-1,  2), ( 1,  2),
        ( 2,  1), ( 2, -1),
        ( 1, -2), (-1, -2),
        (-2, -1), (-2,  1)
    ];

    let mut i = 0u8;
    while i < 64 {
        let origin_sq = Square(i);
        let mut bb = Bitboard::EMPTY;

        let mut delta_idx = 0;
        while delta_idx < 8 {
            let (rank, file) = deltas[delta_idx];
            if let Some(sq) = origin_sq.try_offset(rank, file) {
                bb.0 |= sq.bb().0;
            }

            delta_idx += 1;
        }

        table[i as usize] = bb;
        i += 1;
    }

    table
};

pub const KING_TABLE: [Bitboard; 64] = {
    // this table initialization method took heavy inspiration from analog-hors/cozy-chess
    // https://github.com/analog-hors/cozy-chess/blob/1d8a04d1510071931132ce11e988ced0d41807c8/cozy-chess/src/moves.rs#L313
    let mut table: [Bitboard; 64] = [Bitboard::EMPTY; 64];
    let deltas: [(i8, i8); 9] = [
        (-1, -1), (-1,  0), (-1,  1),
        ( 0, -1), ( 0,  0), ( 0,  1),
        ( 1, -1), ( 1,  0), ( 1,  1)
    ];

    let mut i = 0u8;
    while i < 64 {
        let origin_sq = Square(i);
        let mut bb = Bitboard::EMPTY;

        let mut delta_idx = 0;
        while delta_idx < 9 {
            let (rank, file) = deltas[delta_idx];
            if let Some(sq) = origin_sq.try_offset(rank, file) {
                bb.0 |= sq.bb().0;
            }

            delta_idx += 1;
        }

        table[i as usize] = bb;
        i += 1;
    }

    table
};

/*pub fn p_moves(
    orig: Square,
    is_white: bool,
    friend: Bitboard,
    enemy: Bitboard,
    en_passant: Bitboard
) -> Bitboard {
    let orig = orig.bb();
    let forward = match is_white {
        true  => orig.nort_one(),
        false => orig.sout_one()
    };

    let blocks = friend | enemy;
    let attacks = (forward.east_one() | forward.west_one()) & (enemy | en_passant);
    let movement = if is_white && orig.rank() == 1 {
        let step = forward & !blocks;
        step | step.nort_one() & !blocks
    } else if orig.rank() == 6 {
        let step = forward & !blocks;
        step | step.sout_one() & !blocks
    } else {
        forward & !blocks
    };

    attacks | movement
}*/

/*pub fn n_moves(orig: Square) -> Bitboard {
    let orig = orig.bb();
    let mut horizontal = orig.east_one().east_one();
    horizontal |= orig.west_one().west_one();
    horizontal = horizontal.nort_one() | horizontal.sout_one();

    let mut vertical = orig.nort_one().nort_one();
    vertical |= orig.sout_one().sout_one();
    vertical = vertical.east_one() | vertical.west_one();

    horizontal | vertical
}*/

pub fn n_moves(orig: Square) -> Bitboard {
    KNIGHT_TABLE[orig.0 as usize]
}

/*pub fn k_moves(orig: Square) -> Bitboard {
    let orig = orig.bb();
    let attacks = orig.nort_one() | orig.sout_one();

    attacks | attacks.west_one() | attacks.east_one() | orig.west_one() | orig.east_one()
}*/

pub fn k_moves(orig: Square) -> Bitboard {
    KING_TABLE[orig.0 as usize]
}


/*pub fn b_moves(orig: Square, blockers: Bitboard) -> Bitboard {
    let orig = orig.bb();
    let mut attacks = Bitboard::EMPTY;

    let mut idx = orig.nort_one().east_one();
    while !idx.is_empty() {
        attacks |= idx;
        if !(idx & blockers).is_empty() {
            break;
        }
        idx = idx.nort_one().east_one();
    }

    idx = orig.sout_one().east_one();
    while !idx.is_empty() {
        attacks |= idx;
        if !(idx & blockers).is_empty() {
            break;
        }
        idx = idx.sout_one().east_one();
    }

    idx = orig.sout_one().west_one();
    while !idx.is_empty() {
        attacks |= idx;
        if !(idx & blockers).is_empty() {
            break;
        }
        idx = idx.sout_one().west_one();
    }

    idx = orig.nort_one().west_one();
    while !idx.is_empty() {
        attacks |= idx;
        if !(idx & blockers).is_empty() {
            break;
        }
        idx = idx.nort_one().west_one();
    }

    attacks
}*/

/*pub fn r_moves(orig: Square, blockers: Bitboard) -> Bitboard {
    let orig = orig.bb();
    let mut attacks = Bitboard::EMPTY;

    let mut nort_idx = orig.nort_one();
    while !nort_idx.is_empty() {
        attacks |= nort_idx;
        if !(nort_idx & blockers).is_empty() {
            break;
        }
        nort_idx = nort_idx.nort_one();
    }
    
    let mut east_idx = orig;
    while !east_idx.is_empty() {
        east_idx = east_idx.east_one();
        attacks |= east_idx;
        if !(east_idx & blockers).is_empty() {
            break;
        }
    }

    let mut sout_idx = orig.sout_one();
    while !sout_idx.is_empty() {
        attacks |= sout_idx;
        if !(sout_idx & blockers).is_empty() {
            break;
        }
        sout_idx = sout_idx.sout_one();
    }

    let mut west_idx = orig.west_one();
    while !west_idx.is_empty() {
        attacks |= west_idx;
        if !(west_idx & blockers).is_empty() {
            break;
        }
        west_idx = west_idx.west_one();
    }

    attacks
}*/

pub fn r_moves(orig: Square, blockers: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let rays = [(0, 1),  (1, 0), (0, -1), (-1, 0)];

    for ray in rays {
        let mut curr_sq = orig;
        loop {
            match curr_sq.try_offset(ray.0, ray.1) {
                Some(v) => {
                    let dest = v.bb();
                    attacks |= dest;
                    if (dest & blockers).is_empty() {
                        curr_sq = v;
                    } else {
                        break;
                    }
                },
                None => break
            }
        }
    }

    attacks
}

pub fn b_moves(orig: Square, blockers: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;
    let rays = [(1, 1),  (1, -1), (-1, 1), (-1, -1)];

    for ray in rays {
        let mut curr_sq = orig;
        loop {
            match curr_sq.try_offset(ray.0, ray.1) {
                Some(v) => {
                    let dest = v.bb();
                    attacks |= dest;
                    if (dest & blockers).is_empty() {
                        curr_sq = v;
                    } else {
                        break;
                    }
                },
                None => break
            }
        }
    }

    attacks
}

pub fn q_moves(orig: Square, blockers: Bitboard) -> Bitboard {
    b_moves(orig, blockers) | r_moves(orig, blockers)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::*;

    #[test]
    fn test_n_moves() {
        let output = n_moves(Square::E4);
        let expected_output = Bitboard::new(0x0000284400442800);


        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_moves_edge_close() {
        let output = n_moves(Square::A4);
        let expected_output = Bitboard::new(0x0000020400040200);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_moves_edge_far() {
        let output = n_moves(Square::G5);
        let expected_output = Bitboard::new(0x00a0100010a00000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_moves_corner() {
        let output = n_moves(Square::A1);
        let expected_output = Bitboard::new(0x0000000000020400);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_moves() {
        let output = k_moves(Square::E2);
        let expected_output = Bitboard::new(0x0000000000382838);


        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_moves_edge() {
        let output = k_moves(Square::A4);
        let expected_output = Bitboard::new(0x0000000302030000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_moves_corner() {
        let output = k_moves(Square::H8);
        let expected_output = Bitboard::new(0x40c0000000000000);

        assert_eq!(output, expected_output);
    }
}
