
use crate::bitboard::Bitboard;

pub fn king_move_gen(bb: Bitboard) -> Bitboard {
    let output: Bitboard = bb.nort_one() | bb.sout_one();
    output | output.west_one() | output.east_one() | bb.west_one() | bb.east_one()
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::square::*;

    #[test]
    fn test_king_move_gen() {
        let output = king_move_gen(Square::E2.bb());
        let expected_output = Bitboard::new(0x3828380000000000);


        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_king_move_gen_edge() {
        let output = king_move_gen(Square::A4.bb());
        let expected_output = Bitboard::new(0x00030203000000);

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_king_move_gen_corner() {
        let output = king_move_gen(Square::H8.bb());
        let expected_output = Bitboard::new(0x0000000000c040);

        assert_eq!(output, expected_output);
    }
}
