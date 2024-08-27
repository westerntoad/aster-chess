use super::bitboard::Bitboard;
use super::movement::Move;
use super::movement::Flag;
use super::square::Square;
use super::legal_moves::{
    n_move_gen,
    k_move_gen,
    b_move_gen,
    r_move_gen,
    q_move_gen,
    p_move_gen
};
use std::fmt;

const PAWN_IDX: usize = 0;
const KNIGHT_IDX: usize = 1;
const BISHOP_IDX: usize = 2;
const ROOK_IDX: usize = 3;
const QUEEN_IDX: usize = 4;
const KING_IDX: usize = 5;

#[derive(Clone, PartialEq)]
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
    total_ply: u16
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
        total_ply: 0
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

        let fen_components: Vec<&str> = fen.split_whitespace().collect();
        println!("{:?}", fen_components);
        if fen_components.len() != 6 {
            return Err("Invalid FEN length.");
        }

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

        white_to_move = match fen_components.get(1).unwrap().chars().next().unwrap() {
            'w' => true,
            'b' => false,
             _  => return Err("Invalid side to move."),
        };

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
        println!("{target}");
        en_passant_target = match target {
            "-" => None,
             _  => match Square::from_algebraic(target) {
                Ok(v) => Some(v),
                Err(_) => return Err("Invalid en passant target."),
             },
        };

        let half_move_clock = match fen_components.get(4).unwrap().parse::<u8>() {
            Ok(v) => v,
            Err(_)    => return Err("Invalid half move."),
        };

        let full_moves = fen_components.get(5).unwrap().parse::<u16>().unwrap();
        let total_ply = match white_to_move {
            true  => (full_moves - 1) * 2,
            false => (full_moves - 1) * 2 + 1
        };

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
            total_ply
        })
    }

    fn pawn_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let friend = self.color_bb[!(self.white_to_move) as usize];
        let enemy = self.color_bb[self.white_to_move as usize];

        let en_passant_bb = match self.en_passant_target {
            Some(val) => val.bb(),
            None      => Bitboard::EMPTY
        };
        for pawn_bb in friend & self.piece_bb[PAWN_IDX] {
            for attack_bb in p_move_gen(pawn_bb, self.white_to_move, friend, enemy, en_passant_bb) {
                let flags: Vec<Flag> = if !(attack_bb & (Bitboard::RANK_1 | Bitboard::RANK_8)).is_empty() {
                    if !(attack_bb & enemy).is_empty() {
                        vec![
                            Flag::PromoteCaptureN,
                            Flag::PromoteCaptureB,
                            Flag::PromoteCaptureR,
                            Flag::PromoteCaptureQ
                        ]
                    } else {
                        vec![
                            Flag::PromoteN,
                            Flag::PromoteB,
                            Flag::PromoteR,
                            Flag::PromoteQ
                        ]
                    }
                } else if (!(attack_bb & Bitboard::RANK_4).is_empty() &&  self.white_to_move) ||
                          (!(attack_bb & Bitboard::RANK_5).is_empty() && !self.white_to_move) {
                    vec![Flag::DoublePush]
                } else if !(attack_bb & enemy).is_empty() {
                    vec![Flag::Capture]
                } else {
                    vec![Flag::Quiet]
                };
                for flag in flags {
                    moves.push(Move::new(
                        Square::from_bb(pawn_bb).unwrap(),
                        Square::from_bb(attack_bb).unwrap(),
                        flag
                    ));
                }
            }
        }

        moves
    }

    fn knight_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let friend = self.color_bb[!(self.white_to_move) as usize];
        let enemy = self.color_bb[self.white_to_move as usize];

        for knight_bb in friend & self.piece_bb[KNIGHT_IDX] {
            for attack_bb in n_move_gen(knight_bb) & !friend {
                moves.push(Move::new(
                    Square::from_bb(knight_bb).unwrap(),
                    Square::from_bb(attack_bb).unwrap(),
                    if (attack_bb & enemy).is_empty() {
                        Flag::Quiet
                    } else {
                        Flag::Capture
                    }
                ));
            }
        }

        moves
    }

    fn bishop_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let friend = self.color_bb[!(self.white_to_move) as usize];
        let enemy = self.color_bb[self.white_to_move as usize];

        for bishop_bb in friend & self.piece_bb[BISHOP_IDX] {
            for attack_bb in b_move_gen(bishop_bb, friend | enemy) & !friend {
                moves.push(Move::new(
                    Square::from_bb(bishop_bb).unwrap(),
                    Square::from_bb(attack_bb).unwrap(),
                    if (attack_bb & enemy).is_empty() {
                        Flag::Quiet
                    } else {
                        Flag::Capture
                    }
                ));
            }
        }

        moves
    }

    fn rook_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let friend = self.color_bb[!(self.white_to_move) as usize];
        let enemy = self.color_bb[self.white_to_move as usize];

        for rook_bb in friend & self.piece_bb[ROOK_IDX] {
            for attack_bb in r_move_gen(rook_bb, friend | enemy) & !friend {
                moves.push(Move::new(
                    Square::from_bb(rook_bb).unwrap(),
                    Square::from_bb(attack_bb).unwrap(),
                    if (attack_bb & enemy).is_empty() {
                        Flag::Quiet
                    } else {
                        Flag::Capture
                    }
                ));
            }
        }

        moves
    }

    fn queen_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let friend = self.color_bb[!(self.white_to_move) as usize];
        let enemy = self.color_bb[self.white_to_move as usize];

        for queen_bb in friend & self.piece_bb[QUEEN_IDX] {
            for attack_bb in q_move_gen(queen_bb, friend | enemy) & !friend {
                moves.push(Move::new(
                    Square::from_bb(queen_bb).unwrap(),
                    Square::from_bb(attack_bb).unwrap(),
                    if (attack_bb & enemy).is_empty() {
                        Flag::Quiet
                    } else {
                        Flag::Capture
                    }
                ));
            }
        }

        moves
    }

    fn king_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let friend = self.color_bb[!(self.white_to_move) as usize];
        let enemy = self.color_bb[self.white_to_move as usize];

        for king_bb in friend & self.piece_bb[KING_IDX] {
            for attack_bb in k_move_gen(king_bb) & !friend {
                moves.push(Move::new(
                    Square::from_bb(king_bb).unwrap(),
                    Square::from_bb(attack_bb).unwrap(),
                    if (attack_bb & enemy).is_empty() {
                        Flag::Quiet
                    } else {
                        Flag::Capture
                    }
                ));
            }
        }

        moves
    }

    pub fn legal_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(218);
        moves.extend(self.pawn_moves());
        moves.extend(self.knight_moves());
        moves.extend(self.bishop_moves());
        moves.extend(self.rook_moves());
        moves.extend(self.queen_moves());
        moves.extend(self.king_moves());

        // TODO disgusting filter to determine checks and pins
        let mut new_moves: Vec<Move> = Vec::with_capacity(218);
        moves.into_iter().filter(|x| {
            new_moves.clear();
            // this might be the first problem area. possibly make and unmake moves for current
            // position instead?
            let mut new_pos = self.clone();
            new_pos.make_move(x);
            let king_bb = new_pos.piece_bb[KING_IDX] & new_pos.enemy();

            new_moves.extend(new_pos.pawn_moves());
            new_moves.extend(new_pos.knight_moves());
            new_moves.extend(new_pos.bishop_moves());
            new_moves.extend(new_pos.rook_moves());
            new_moves.extend(new_pos.queen_moves());
            new_moves.extend(new_pos.king_moves());

            for action in &new_moves {
                if !(action.targ().bb() & king_bb).is_empty() {
                    return false;
                }
            }

            true
        }).collect::<Vec<Move>>()
    }

    pub fn make_move(&mut self, action: &Move) {
        self.color_bb[!(self.white_to_move) as usize] &= !action.orig().bb();
        self.color_bb[!(self.white_to_move) as usize] |=  action.targ().bb();
        self.color_bb[  self.white_to_move  as usize] &= !action.targ().bb();

        let mut piece_idx = 99;
        for (i, piece_bb_idx) in self.piece_bb.iter().enumerate() {
            if !(*piece_bb_idx & action.orig().bb()).is_empty() {
                piece_idx = i;
                break;
            }
        }

        self.piece_bb[piece_idx] &= !action.orig().bb();
        self.piece_bb[piece_idx] |=  action.targ().bb();
        
        self.half_move_clock += 1;
        self.white_to_move = !self.white_to_move;
    }

    pub fn friend(&self) -> Bitboard {
        self.color_bb[!(self.white_to_move) as usize]
    }

    pub fn enemy(&self) -> Bitboard {
        self.color_bb[self.white_to_move as usize]
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
        output.push('\n');

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
            output.push(*element);
            
            if i % 8 != 7 {
                output.push(' ');
            } else {
                output.push('\n');
            }
        }

        output.push_str(&format!(" {} to move\n",
            match self.white_to_move  {
                true  => "White",
                false => "Black"
            }
        ));

        output.push_str(&format!("{:^15}\n",
            format!("{} ply", self.total_ply)
        ));

        output.push_str(&format!("{:^15}\n", format!("ep_targ {}",
            match self.en_passant_target {
                Some(val) => format!("{}", val),
                None      => "--".to_string()
            }
        )));


        output.push_str(&format!("{:^15}\n",
            format!("hm_clock {}", self.half_move_clock)
        ));

        output.push_str(&format!("Castle legality\n     w    b\nk    {}    {}\nq    {}    {}\n",
            match self.can_castle_wk {
                true  => "✓",
                false => "𐄂"
            },
            match self.can_castle_bk {
                true  => "✓",
                false => "𐄂"
            },
            match self.can_castle_wq {
                true  => "✓",
                false => "𐄂"
            },
            match self.can_castle_bq {
                true  => "✓",
                false => "𐄂"
            },
        ));

        /*output.push_str("\nLegal moves: \n");
        for (i, action) in self.legal_moves().iter().enumerate() {
            output.push_str(&format!("{: <6}{}\n", i+1, action));
        }*/

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use super::*;

    #[test]
    fn test_fen_starting() {
        let output = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();

        assert_eq!(output, Board::STARTING_POSITION);
    }

    #[test]
    fn test_fen_opening() {
        let output = Board::from_fen("rn1qkbnr/pp3ppp/2p1p3/3pPb2/3P4/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 5").unwrap();
        let expected_output = Board {
            color_bb: [
                Bitboard::new(0x000000100820e7bf), // white
                Bitboard::new(0xfbe3142800000000), // black
            ],
            piece_bb: [
                Bitboard::new(0x00e314180800e700), // pawn
                Bitboard::new(0x4200000000200002), // knight
                Bitboard::new(0x2000002000000024), // bishop
                Bitboard::new(0x8100000000000081), // rooks
                Bitboard::new(0x0800000000000008), // queens
                Bitboard::new(0x1000000000000010), // queens
            ],
            white_to_move: true,
            can_castle_wk: true,
            can_castle_wq: true,
            can_castle_bk: true,
            can_castle_bq: true,
            en_passant_target: None,
            half_move_clock: 0,
            total_ply: 8
        };

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_fen_middlegame() {
        let output = Board::from_fen("r1bq1rk1/1p2bpp1/p1nppn1p/8/3NP1PP/2N1B3/PPPQBP2/2KR3R b - g3 0 11").unwrap();
        let expected_output = Board {
            color_bb: [
                Bitboard::new(0x00000000d8143f8c), // white
                Bitboard::new(0x6d72bd0000000000), // black
            ],
            piece_bb: [
                Bitboard::new(0x00629900d0002700), // pawn
                Bitboard::new(0x0000240008040000), // knight
                Bitboard::new(0x0410000000101000), // bishop
                Bitboard::new(0x2100000000000088), // rooks
                Bitboard::new(0x0800000000000800), // queens
                Bitboard::new(0x4000000000000004), // queens
            ],
            white_to_move: false,
            can_castle_wk: false,
            can_castle_wq: false,
            can_castle_bk: false,
            can_castle_bq: false,
            en_passant_target: Some(Square::G3),
            half_move_clock: 0,
            total_ply: 21
        };

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_fen_endgame() {
        let output: Board = Board::from_fen("8/5k2/1p1p2p1/3Pnb1p/2P2b1P/5P2/3qBP2/4NKRQ w - - 3 39").unwrap();
        let expected_output = Board {
            color_bb: [
                Bitboard::new(0x00000008842030f0), // white
                Bitboard::new(0x00204ab020000800), // black
            ],
            piece_bb: [
                Bitboard::new(0x00004a8884202000), // pawn
                Bitboard::new(0x0000001000000010), // knight
                Bitboard::new(0x0000002020001000), // bishop
                Bitboard::new(0x0000000000000040), // rooks
                Bitboard::new(0x0000000000000880), // queens
                Bitboard::new(0x0020000000000020), // kings
            ],
            white_to_move: true,
            can_castle_wk: false,
            can_castle_wq: false,
            can_castle_bk: false,
            can_castle_bq: false,
            en_passant_target: None,
            half_move_clock: 3,
            total_ply: 76
        };

        assert_eq!(output, expected_output);
    }

    fn num_of_positions(position: Board, depth: u32) -> u64 {
        if depth == 0 { return 1 };

        let mut total_positions = 0;
        for action in position.legal_moves() {
            let mut new_position = position.clone();
            new_position.make_move(&action);
            total_positions += num_of_positions(new_position, depth - 1);
        }

        total_positions
    }

    #[test]
    fn test_num_pos_starting() {
        const PLY_AMOUNT: usize = 5;
        const EXPECTED_OUTPUT: [u64; PLY_AMOUNT+1] = [1, 20, 400, 8_902, 197_281, 4_865_609];

        for i in 0..=PLY_AMOUNT {
            let now = Instant::now();
            let output = num_of_positions(Board::STARTING_POSITION, i as u32);
            let elapsed = now.elapsed();
            println!("Running test_num_pos_starting at ply={}. Elapsed time={:.2?}", i, elapsed);
            assert_eq!(output, EXPECTED_OUTPUT[i]);
        }
    }
}
