use super::{
    Board,
    MAX_NUM_MOVES
};
use crate::types::{
    bitboard::Bitboard,
    square::Square,
    piece::Piece,
    state_metadata::BoardStateFlags
};

impl Board {
    pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
        let mut fen_components: Vec<&str> = fen.split_whitespace().collect();
        if fen_components.len() == 4 {
            fen_components.push("0");
            fen_components.push("1");
        } else if fen_components.len() != 6 {
            return Err("Invalid FEN length.");
        }

        let mut color_bb = [Bitboard::EMPTY; 2];
        let mut piece_bb = [Bitboard::EMPTY; 6];
        for (i, row) in fen_components.get(0).unwrap().split('/').enumerate() {
            let mut j = 0;
            for mut elem in row.chars() {
                let sq = Square::from_coord(7 - (i as u8), j as u8).unwrap();     // FIX BAD UNWRAP
                match elem {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        j += elem as u8 - 48;
                        continue;
                    },
                    _ => {

                        let color_idx = elem.is_lowercase() as usize;
                        elem.make_ascii_lowercase();
                        let piece_idx = match elem {
                            'p' => 0,
                            'n' => 1,
                            'b' => 2,
                            'r' => 3,
                            'q' => 4,
                            'k' => 5,
                             _  => return Err("Invalid piece character.")
                        };

                        color_bb[color_idx] |= sq.bb();
                        piece_bb[piece_idx] |= sq.bb();

                        j += 1;
                    },
                }
            }
        }

        let white_to_move = match fen_components.get(1).unwrap().chars().next().unwrap() {
            'w' => true,
            'b' => false,
             _  => return Err("Invalid side to move."),
        };

        let mut can_castle_wk: bool = false;
        let mut can_castle_wq: bool = false;
        let mut can_castle_bk: bool = false;
        let mut can_castle_bq: bool = false;
        for character in fen_components.get(2).unwrap().chars() {
            match character {
                'K' => can_castle_wk = true,
                'Q' => can_castle_wq = true,
                'k' => can_castle_bk = true,
                'q' => can_castle_bq = true,
                '-' => continue,
                 _  => return Err("Invalid castling validity."),

            };
        }

        let target = *fen_components.get(3).unwrap();
        let en_passant_target = match target {
            "-" => None,
             _  => match Square::from_algebraic(target) {
                Ok(v) => Some(v),
                Err(_) => return Err("Invalid en passant target."),
             },
        };

        let half_move_clock = match fen_components.get(4).unwrap().parse::<u32>() {
            Ok(v) => v,
            Err(_)    => return Err("Invalid half move."),
        };

        let full_moves = fen_components.get(5).unwrap().parse::<u16>().unwrap();
        let total_ply = match white_to_move {
            true  => (full_moves - 1) * 2,
            false => (full_moves - 1) * 2 + 1
        };

        let mut state_stack = [BoardStateFlags::NO_DATA; MAX_NUM_MOVES];
        state_stack[total_ply as usize] = BoardStateFlags::new(
            can_castle_wk,
            can_castle_wq,
            can_castle_bk,
            can_castle_bq,
            en_passant_target,
            Piece::NoPiece,
            Square::A1,
            half_move_clock
        );

        Ok(Board {
            color_bb,
            piece_bb,
            white_to_move,
            state_stack,
            total_ply
        })
    }
}
