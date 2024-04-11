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
    // Constants
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
        let color_bb = [Bitboard::EMPTY; 2];
        let piece_bb = [Bitboard::EMPTY; 6];
        let white_to_move: bool;
        let can_castle_wk: bool;
        let can_castle_wq: bool;
        let can_castle_bk: bool;
        let can_castle_bq: bool;
        let en_passant_target: Option<Square>;
        let half_move_clock: u8;

        let fen_components: Vec<&str> = fen.split_whitespace().collect();
        if fen_components.len() != 6 {
            return Err("Invalid FEN length.");
        }

        for (i, row) in fen_components.get(0).unwrap().split('/').enumerate() {
            let mut j = 0;
            for elem in row.chars() {
                let bb = Square::from_coord(j as u8, i as u8).unwrap();
                //println!("{}\n{:?}", bb, Bitboard::from_sq(bb));
                match elem {
                    '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' => {
                        j += elem as u8 - 48;
                        continue;
                    },
                    _ => {
                        //println!("{} = {}", elem, elem as u8);
                        j += 1;
                    },
                }
            }
        }

        //println!("{:?}", fen_components);



        Ok(Self::STARTING_POSITION)
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(218);

        let color_bb = self.color_bb[(!self.white_to_move) as usize]; 

        // Knights

        moves
    }
    
    pub fn colors(&self) -> [Bitboard; 2] {
        self.color_bb
    }

    pub fn pieces(&self) -> [Bitboard; 6] {
        self.piece_bb
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
