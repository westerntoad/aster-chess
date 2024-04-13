
use crate::bitboard::Bitboard;

pub fn king_move_gen(bb: Bitboard) -> Bitboard {
    let output: Bitboard = bb.nort_one() | bb.sout_one();
    output.west_one() | output.east_one()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_king_move_gen() {
        todo!();
    }

    #[test]
    fn test_king_move_gen_edge() {
        todo!();
    }

    #[test]
    fn test_king_move_gen_corner() {
        todo!();
    }
}
