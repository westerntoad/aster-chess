use crate::bitboard::Bitboard;

pub fn pawn_move_gen(orig: Bitboard, enemy:Bitboard, friend: Bitboard) -> Bitboard {
    todo!();
}

pub fn n_move_gen(orig: Bitboard, enemy: Bitboard, friend: Bitboard) -> Bitboard {
    let mut horizontal = orig.east_one().east_one();
    horizontal |= orig.west_one().west_one();
    horizontal = horizontal.nort_one() | horizontal.sout_one();

    let mut vertical = orig.nort_one().nort_one();
    vertical |= orig.sout_one().sout_one();
    vertical = vertical.east_one() | vertical.west_one();

    horizontal | vertical
}

pub fn b_move_gen(orig: Bitboard, enemy: Bitboard, friend: Bitboard) -> Bitboard {
    let mut attacks = Bitboard::EMPTY;

    let mut no_ea_idx = orig.nort_one().east_one();
    while !no_ea_idx.is_empty() && (no_ea_idx | friend).is_empty() {
        attacks |= no_ea_idx;
        no_ea_idx = no_ea_idx.nort_one().east_one();
        if !(no_ea_idx & enemy).is_empty() {
            break;
        }
    }

    let mut so_ea_idx = orig.sout_one().east_one();
    while !so_ea_idx.is_empty() && (so_ea_idx | friend).is_empty() {
        attacks |= no_ea_idx;
        so_ea_idx = so_ea_idx.sout_one().east_one();
        if !(so_ea_idx & enemy).is_empty() {
            break;
        }
    }

    let mut so_we_idx = orig.sout_one().west_one();
    while !so_we_idx.is_empty() && (so_we_idx | friend).is_empty() {
        attacks |= so_we_idx;
        so_we_idx = so_we_idx.sout_one().west_one();
        if !(so_we_idx & enemy).is_empty() {
            break;
        }
    }

    let mut no_we_idx = orig.nort_one().west_one();
    while !no_we_idx.is_empty() && (no_we_idx | friend).is_empty() {
        attacks |= no_we_idx;
        no_we_idx = no_we_idx.nort_one().west_one();
        if !(no_we_idx & enemy).is_empty() {
            break;
        }
    }

    return attacks;
}

pub fn r_move_gen(orig: Bitboard) -> Bitboard {
    todo!();
}

pub fn q_move_gen(orig: Bitboard) -> Bitboard {
    todo!();
}

pub fn k_move_gen(orig: Bitboard, enemy: Bitboard, friend: Bitboard) -> Bitboard {
    let attacks = orig.nort_one() | orig.sout_one();
    attacks | attacks.west_one() | attacks.east_one() | orig.west_one() | orig.east_one()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::*;

    #[test]
    fn test_n_move_gen() {
        let output = n_move_gen(Square::E4.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x28440044280000);


        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_move_gen_edge_close() {
        let output = n_move_gen(Square::A4.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x02040004020000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_move_gen_edge_far() {
        let output = n_move_gen(Square::G5.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x00a0100010a000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_n_move_gen_corner() {
        let output = n_move_gen(Square::A1.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x04020000000000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_move_gen() {
        let output = k_move_gen(Square::E2.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x3828380000000000);


        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_move_gen_edge() {
        let output = k_move_gen(Square::A4.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x00030203000000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_k_move_gen_corner() {
        let output = k_move_gen(Square::H8.bb(), Bitboard::EMPTY, Bitboard::EMPTY);
        let expected_output = Bitboard::new(0x0000000000c040);

        assert_eq!(output, expected_output);
    }
}
