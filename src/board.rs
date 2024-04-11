use super::bitboard::Bitboard;
use super::movement::Move;
use super::square::Square;
use std::fmt;

pub struct Board {
    color_bb: [Bitboard; 2],
    piece_bb: [Bitboard; 6],
    white_to_move: bool,
    can_castle_wk: bool,
    can_castle_wq: bool,
    can_castle_bk: bool,
    can_castle_bq: bool,
    en_passant_target: Option<Square>,
    half_move_clock: u8,
}

impl Board {
    pub const STARTING_POSITION: Board = Board {
        color_bb: [
            Bitboard::STARTING_WHITE,
            Bitboard::STARTING_BLACK,
        ],
        piece_bb: [
            Bitboard::STARTING_PAWNS,
            Bitboard::STARTING_KNIGHTS,
            Bitboard::STARTING_BISHOPS,
            Bitboard::STARTING_ROOKS,
            Bitboard::STARTING_QUEENS,
            Bitboard::STARTING_KINGS,
        ],
        white_to_move: true,
        can_castle_wk: true,
        can_castle_wq: true,
        can_castle_bk: true,
        can_castle_bq: true,
        en_passant_target: None,
        half_move_clock: 0,
    };

    pub fn from_fen(fen: &str) -> Result<Self, &'static str> {
        let mut color_bb = [Bitboard::EMPTY; 2];
        let mut piece_bb = [Bitboard::EMPTY; 6];
        let mut white_to_move: bool = false;
        let mut can_castle_wk: bool = false;
        let mut can_castle_wq: bool = false;
        let mut can_castle_bk: bool = false;
        let mut can_castle_bq: bool = false;
        let mut en_passant_target: Option<Square> = None;
        let mut half_move_clock: u8 = 0;

        let fen_components: Vec<&str> = fen.split_whitespace().collect();
        if fen_components.len() != 6 {
            return Err("Invalid FEN length.");
        }

        for (i, row) in fen_components.get(0).unwrap().split('/').enumerate() {
            let mut j = 0;
            for mut elem in row.chars() {
                let sq = Square::from_coord(7 - (i as u8), j as u8).unwrap();     // FIX BAD UNWRAP
                let bb = sq.bb();
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

        white_to_move = match fen_components.get(1).unwrap().chars().next().unwrap() {
            'w' => true,
            'b' => false,
             _  => return Err("Invalid side to move."),
        };

        for character in fen_components.get(2).unwrap().chars() {
            match character {
                'K' => can_castle_wk = true,
                'Q' => can_castle_wq = true,
                'k' => can_castle_wq = true,
                'q' => can_castle_wq = true,
                '-' => continue,
                 _  => return Err("Invalid castling validity."),

            };
        }

        let target = *fen_components.get(3).unwrap();
        en_passant_target = match target {
            "-" => None,
             _  => match Square::from_algebraic(target) {
                Ok(v) => Some(v),
                Err(_) => return Err("Invalid en passant target."),
             },
        };

        half_move_clock = match fen_components.get(4).unwrap().parse::<u8>() {
            Ok(v) => v,
            Err(_)    => return Err("Invalid half move."),
        };




        println!("{:?}", fen_components);

        Ok(Board {
            color_bb,
            piece_bb,
            white_to_move,
            can_castle_wk,
            can_castle_wq,
            can_castle_bk,
            can_castle_bq,
            en_passant_target,
            half_move_clock,
        })
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(218);

        let color_bb = self.color_bb[(!self.white_to_move) as usize]; 

        todo!();

        moves
    }
    
    pub fn colors(&self) -> [Bitboard; 2] {
        self.color_bb
    }

    pub fn pieces(&self) -> [Bitboard; 6] {
        self.piece_bb
    }
}

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.color_bb == other.color_bb && self.piece_bb == other.piece_bb
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        let mut board_arr: [char; 64] = ['?'; 64];

        for i in 0..board_arr.len() {
            if i % 16 > 7 && i % 2 == 0
            || i % 16 <= 8 && i % 2 == 1 {
                board_arr[i] = '.';
            } else {
                board_arr[i] = '.';
            }
        }

        for (i, color) in self.color_bb.iter().enumerate() {
            for (j, piece) in self.piece_bb.iter().enumerate() {
                let bytes = (color.val() & piece.val()).to_be_bytes();
                for k in 0..64 {
                    let bit = (bytes[k / 8] >> (k % 8)) % 2;

                    if bit != 0 {
                        board_arr[k] = char::from_u32((9823 - j - i * 6) as u32).unwrap_or('?');
                    }
                }
            }
        }

        for (i, element) in board_arr.iter().enumerate() {
            output.push(board_arr[i]);
            
            if i % 8 != 7 {
                output.push(' ');
            } else {
                output.push('\n');
            }
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fen_starting() {
        let output: Board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_eq!(output, Board::STARTING_POSITION);
    }

    #[test]
    fn test_fen_opening() {
        let output: Board = Board::from_fen("").unwrap();

        todo!();
    }

    #[test]
    fn test_fen_middlegame() {
        let output: Board = Board::from_fen("").unwrap();

        todo!();
    }

    #[test]
    fn test_fen_endgame() {
        let output: Board = Board::from_fen("8/5k2/1p1p2p1/3Pnb1p/2P2b1P/5P2/3qBP2/4NKRQ w - - 3 39").unwrap();
        
        todo!();
        
        /*let expected_output: Board = Board {
            color_bb: [
                Bitboard::new(),
                Bitboard::new(),
            ],
            piece_bb: [
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
                Bitboard::new(),
            ],
            white_to_move: true,
            can_castle_wk: true,
            can_castle_wq: true,
            can_castle_bk: true,
            can_castle_bq: true,
            en_passant_target: None,
            half_move_clock: 0,
        };*/
    }
}
