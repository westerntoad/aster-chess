use crate::types::{
    bitboard::Bitboard,
    square::Square
};

pub fn p_move_gen(
    orig: Bitboard,
    is_white: bool,
    friend: Bitboard,
    enemy: Bitboard,
    en_passant: Bitboard
) -> Bitboard {
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
}

#[inline(always)]
pub fn n_move_gen(orig: Square) -> Bitboard {
    let orig = orig.bb();
    let mut horizontal = orig.east_one().east_one();
    horizontal |= orig.west_one().west_one();
    horizontal = horizontal.nort_one() | horizontal.sout_one();

    let mut vertical = orig.nort_one().nort_one();
    vertical |= orig.sout_one().sout_one();
    vertical = vertical.east_one() | vertical.west_one();

    horizontal | vertical
}

pub fn k_move_gen(orig: Square) -> Bitboard {
    let orig = orig.bb();
    let attacks = orig.nort_one() | orig.sout_one();

    attacks | attacks.west_one() | attacks.east_one() | orig.west_one() | orig.east_one()
}


pub fn b_move_gen(orig: Square, blockers: Bitboard) -> Bitboard {
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
}

pub fn r_move_gen(orig: Square, blockers: Bitboard) -> Bitboard {
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
}

pub fn q_move_gen(orig: Square, blockers: Bitboard) -> Bitboard {
    b_move_gen(orig, blockers) | r_move_gen(orig, blockers)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::square::*;

    #[test]
    fn test_n_move_gen() {
        let output = n_move_gen(Square::E4);
        let expected_output = Bitboard::new(0x0000284400442800);


        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_move_gen_edge_close() {
        let output = n_move_gen(Square::A4);
        let expected_output = Bitboard::new(0x0000020400040200);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_move_gen_edge_far() {
        let output = n_move_gen(Square::G5);
        let expected_output = Bitboard::new(0x00a0100010a00000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_move_gen_corner() {
        let output = n_move_gen(Square::A1);
        let expected_output = Bitboard::new(0x0000000000020400);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_move_gen() {
        let output = k_move_gen(Square::E2);
        let expected_output = Bitboard::new(0x0000000000382838);


        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_move_gen_edge() {
        let output = k_move_gen(Square::A4);
        let expected_output = Bitboard::new(0x0000000302030000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_move_gen_corner() {
        let output = k_move_gen(Square::H8);
        let expected_output = Bitboard::new(0x40c0000000000000);

        assert_eq!(output, expected_output);
    }
}
