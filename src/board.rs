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

// length is longest possible game of Chess
// source: https://www.reddit.com/r/chess/comments/168qmk6/longest_possible_chess_game_88485_moves
const MAX_NUM_MOVES: usize = 17_697;

const WHITE_IDX: usize = 0;
const BLACK_IDX: usize = 1;

const PAWN_IDX: usize = 0;
const KNIGHT_IDX: usize = 1;
const BISHOP_IDX: usize = 2;
const ROOK_IDX: usize = 3;
const QUEEN_IDX: usize = 4;
const KING_IDX: usize = 5;

#[derive(Copy, Clone, PartialEq)]
pub struct BoardStateFlags(u32);

#[derive(Clone, PartialEq)]
pub struct Board {
    color_bb: [Bitboard; 2],
    piece_bb: [Bitboard; 6],
    white_to_move: bool,
    //state_stack: Vec<BoardStateFlags>,
    state_stack: [BoardStateFlags; MAX_NUM_MOVES],
    total_ply: u16
}

impl BoardStateFlags {

    pub const STARTING: Self = Self(0b1_1111_0_000000_00000000000000000000);
    pub const NO_DATA:  Self = Self(0);

    const HAS_DATA_MASK:      u32 = 0b1_0000_0_000000_00000000000000000000;
    const CASTLE_WK_MASK:     u32 = 0b0_1000_0_000000_00000000000000000000;
    const CASTLE_WQ_MASK:     u32 = 0b0_0100_0_000000_00000000000000000000;
    const CASTLE_BK_MASK:     u32 = 0b0_0010_0_000000_00000000000000000000;
    const CASTLE_BQ_MASK:     u32 = 0b0_0001_0_000000_00000000000000000000;
    const EP_TARG_EXIST_MASK: u32 = 0b0_0000_1_000000_00000000000000000000;
    const EP_TARG_MASK:       u32 = 0b0_0000_0_111111_00000000000000000000;
    const EP_MASK:            u32 = 0b0_0000_1_111111_00000000000000000000;
    const HALF_MOVE_MASK:     u32 = 0b0_0000_0_000000_11111111111111111111;

    const EP_TARG_NUM_BITS:   u32 = 6;
    const HALF_MOVE_NUM_BITS: u32 = 20;

    fn set_bit(&mut self, mask: u32, state: bool) {
        match state {
            true  => self.0 |  mask,
            false => self.0 & !mask
        };
    }

    pub fn new(
            can_castle_wk: bool,
            can_castle_wq: bool,
            can_castle_bk: bool,
            can_castle_bq: bool,
            en_passant_target: Option<Square>,
            half_move_clock: u32
        ) -> Self {
        let mut val = 0u32;

        let mut add_bool_flag = |flag: bool| {
            if flag {
                val += 1;
            }
            val = val << 1;
        };

        add_bool_flag(true);
        add_bool_flag(can_castle_wk);
        add_bool_flag(can_castle_wq);
        add_bool_flag(can_castle_bk);
        add_bool_flag(can_castle_bq);
        let mut ep_sq = 0;
        add_bool_flag(match en_passant_target {
            Some(sq) => {
                ep_sq = sq.val();
                true
            },
            None => false
        });

        val = val << 5;
        val += ep_sq as u32;

        val = val << 20;
        val += half_move_clock;

        Self(val)
    }

    pub fn next_move(
                &self,
                reset_clock: bool,
                disable_wk: bool,
                disable_wq: bool,
                disable_bk: bool,
                disable_bq: bool,
                ep_target: Option<Square>
            ) -> Self {
        let mut new = self.clone();

        if reset_clock {
            new.0 &= !Self::HALF_MOVE_MASK;
        } else {
            new.0 += 1;
        }

        if disable_wk { new.0 &= !Self::CASTLE_WK_MASK; }
        if disable_wq { new.0 &= !Self::CASTLE_WQ_MASK; }
        if disable_bk { new.0 &= !Self::CASTLE_BK_MASK; }
        if disable_bq { new.0 &= !Self::CASTLE_BQ_MASK; }


        new.0 &= !Self::EP_MASK;
        match ep_target {
            Some(sq) => new.0 |= ((sq.val() as u32) << 20) + Self::EP_TARG_EXIST_MASK,
            None     => ()
        };
        
        new
    }

    pub fn has_data(&self) -> bool {
        self.0 & Self::HAS_DATA_MASK != 0
    }

    pub fn can_castle_wk(&self) -> bool {
        self.0 & Self::CASTLE_WK_MASK != 0
    }

    pub fn can_castle_wq(&self) -> bool {
        self.0 & Self::CASTLE_WQ_MASK != 0
    }

    pub fn can_castle_bk(&self) -> bool {
        self.0 & Self::CASTLE_BK_MASK != 0
    }

    pub fn can_castle_bq(&self) -> bool {
        self.0 & Self::CASTLE_BQ_MASK != 0
    }

    pub fn has_ep_target(&self) -> bool {
        self.0 & Self::EP_TARG_EXIST_MASK != 0
    }

    pub fn ep_target_unchecked(&self) -> Square {
        Square::new(((self.0 & Self::EP_TARG_MASK) >> Self::HALF_MOVE_NUM_BITS) as u8).unwrap()
    }

    pub fn ep_target(&self) -> Option<Square> {
        match self.has_ep_target() {
            true  => Some(self.ep_target_unchecked()),
            false => None
        }
    }

    pub fn half_moves(&self) -> u32 {
        self.0 & Self::HALF_MOVE_MASK
    }

    pub fn fifty_move_rule(&self) -> bool {
        self.half_moves() >= 100
    }
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
        state_stack: {
            let mut stack = [BoardStateFlags::NO_DATA; 17_697];

            stack[0] = BoardStateFlags::STARTING;
            stack
        },
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

        let half_move_clock = match fen_components.get(4).unwrap().parse::<u32>() {
            Ok(v) => v,
            Err(_)    => return Err("Invalid half move."),
        };

        let full_moves = fen_components.get(5).unwrap().parse::<u16>().unwrap();
        let total_ply = match white_to_move {
            true  => (full_moves - 1) * 2,
            false => (full_moves - 1) * 2 + 1
        };

        let mut state_stack = [BoardStateFlags::NO_DATA; 17_697];
        state_stack[total_ply as usize] = BoardStateFlags::new(
            can_castle_wk,
            can_castle_wq,
            can_castle_bk,
            can_castle_bq,
            en_passant_target,
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

    fn pawn_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let friend = self.color_bb[!(self.white_to_move) as usize];
        let enemy = self.color_bb[self.white_to_move as usize];

        let en_passant_bb = match self.ep_target() {
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
                } else if (self.white_to_move
                            && !(attack_bb & Bitboard::RANK_4).is_empty()
                            && !(pawn_bb   & Bitboard::RANK_2).is_empty())
                            ||(!(attack_bb & Bitboard::RANK_5).is_empty()
                            && !(pawn_bb   & Bitboard::RANK_7).is_empty()) {
                          //(!(attack_bb & Bitboard::RANK_4).is_empty() &&  self.white_to_move) ||
                          //(!(attack_bb & Bitboard::RANK_5).is_empty() && !self.white_to_move) {
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

    pub fn moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::with_capacity(256);
        moves.extend(self.pawn_moves());
        moves.extend(self.knight_moves());
        moves.extend(self.bishop_moves());
        moves.extend(self.rook_moves());
        moves.extend(self.queen_moves());
        moves.extend(self.king_moves());
        if self.can_castle_short() { moves.push(Move::SHORT_CASTLE) }
        if self.can_castle_long()  { moves.push(Move::LONG_CASTLE) }

        moves
    }

    pub fn legal_moves(&mut self) -> Vec<Move> {
        self.moves().into_iter().filter(|x| {
            self.make_move(x);
            let king_bb = self.piece_bb[KING_IDX] & self.enemy();

            for action in &self.moves() {
                if !(action.targ().bb() & king_bb).is_empty() {
                    self.unmake_move(x);
                    return false;
                }
            }

            self.unmake_move(x);
            true
        }).collect::<Vec<Move>>()
    }

    pub fn make_move(&mut self, action: &Move) {
        let action_flag = action.flag();
        let is_short_castle = action_flag == Flag::ShortCastle;
        let is_long_castle = action_flag == Flag::LongCastle;
        let is_pawn_move = !(self.piece_bb[PAWN_IDX] & action.orig().bb()).is_empty();
        let (wk_castle, wq_castle) = {
            let mut kingside  = false;
            let mut queenside = false;

            if self.white_to_move {
                let king_home = !(Square::E1.bb() & self.friend() & self.piece_bb[KING_IDX]).is_empty();

                if self.can_castle_short() {
                    kingside  = !king_home || (Square::H1.bb() & self.friend() & self.piece_bb[ROOK_IDX]).is_empty()
                        || is_short_castle;
                }

                if self.can_castle_long() {
                    queenside = !king_home || (Square::A1.bb() & self.friend() & self.piece_bb[ROOK_IDX]).is_empty()
                        || is_long_castle;
                }
            }

            (kingside, queenside)
        };

        let (bk_castle, bq_castle) = {
            let mut kingside  = false;
            let mut queenside = false;

            if !self.white_to_move {
                let king_home = !(Square::E8.bb() & self.friend() & self.piece_bb[KING_IDX]).is_empty();

                if self.can_castle_short() {
                    kingside  = !king_home || (Square::H8.bb() & self.friend() & self.piece_bb[ROOK_IDX]).is_empty()
                        || is_short_castle;
                }

                if self.can_castle_long() {
                    queenside = !king_home || (Square::A8.bb() & self.friend() & self.piece_bb[ROOK_IDX]).is_empty()
                        || is_long_castle;
                }
            }

            (kingside, queenside)
        };

        if action.is_castle() {
            let orig_king = self.friend() & self.piece_bb[KING_IDX];
            let (orig_rook, dest_rook, dest_king) = match (self.white_to_move, is_short_castle) {
                (true, true)   => (Bitboard::WK_ROOK, Bitboard::WK_ROOK_DEST, Bitboard::WK_KING_DEST),
                (true, false)  => (Bitboard::WQ_ROOK, Bitboard::WQ_ROOK_DEST, Bitboard::WQ_KING_DEST),
                (false, true)  => (Bitboard::BK_ROOK, Bitboard::BK_ROOK_DEST, Bitboard::BK_KING_DEST),
                (false, false) => (Bitboard::BQ_ROOK, Bitboard::BQ_ROOK_DEST, Bitboard::BQ_KING_DEST),
            };
            
            self.color_bb[!(self.white_to_move) as usize] &= !( orig_king | orig_rook );
            self.color_bb[!(self.white_to_move) as usize] |=    dest_king | dest_rook;

            self.piece_bb[ROOK_IDX] &= !orig_rook;
            self.piece_bb[ROOK_IDX] |=  orig_rook;
            self.piece_bb[KING_IDX] &= !orig_king;
            self.piece_bb[KING_IDX] |=  orig_king;
        } else {
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
            if action.is_promotion() {
                piece_idx = match action.flag() {
                    Flag::PromoteCaptureN => KNIGHT_IDX,
                    Flag::PromoteCaptureB => BISHOP_IDX,
                    Flag::PromoteCaptureR => ROOK_IDX,
                    Flag::PromoteCaptureQ => QUEEN_IDX,
                    _ => panic!("Invalid move flag")
                };
            };
            self.piece_bb[piece_idx] |=  action.targ().bb();
        }
            
        self.state_stack[self.total_ply as usize + 1] = self.curr_flags().next_move(
            action.is_capture() && is_pawn_move,
            wk_castle,
            wq_castle,
            bk_castle,
            bq_castle,
            action.ep_square()
        );
        self.total_ply += 1;
        self.white_to_move = !self.white_to_move;
    }

    pub fn unmake_move(&mut self, action: &Move) {
        let is_short_castle = action.flag() == Flag::ShortCastle;

        if action.is_castle() {
            let dest_king = self.friend() & self.piece_bb[KING_IDX];
            let (orig_rook, dest_rook, orig_king) = match (!self.white_to_move, is_short_castle) {
                (true, true)   => (Bitboard::WK_ROOK_DEST, Bitboard::WK_ROOK, Bitboard::WK_KING_DEST),
                (true, false)  => (Bitboard::WQ_ROOK_DEST, Bitboard::WQ_ROOK, Bitboard::WQ_KING_DEST),
                (false, true)  => (Bitboard::BK_ROOK_DEST, Bitboard::BK_ROOK, Bitboard::BK_KING_DEST),
                (false, false) => (Bitboard::BQ_ROOK_DEST, Bitboard::BQ_ROOK, Bitboard::BQ_KING_DEST),
            };

            // TODO
            self.color_bb[!(self.white_to_move) as usize] &= !( orig_king | orig_rook );
            self.color_bb[!(self.white_to_move) as usize] |=    dest_king | dest_rook;

            self.piece_bb[ROOK_IDX] &= !orig_rook;
            self.piece_bb[ROOK_IDX] |=  orig_rook;
            self.piece_bb[KING_IDX] &= !orig_king;
            self.piece_bb[KING_IDX] |=  orig_king;

        } else {
            self.color_bb[!(self.white_to_move) as usize] &= !action.targ().bb();
            self.color_bb[!(self.white_to_move) as usize] |=  action.orig().bb();
            self.color_bb[  self.white_to_move  as usize] &= !action.orig().bb();

            let mut piece_idx = 99;
            for (i, piece_bb_idx) in self.piece_bb.iter().enumerate() {
                if !(*piece_bb_idx & action.targ().bb()).is_empty() {
                    piece_idx = i;
                    break;
                }
            }

            self.piece_bb[piece_idx] &= !action.targ().bb();
            if action.is_promotion() {
                piece_idx = match action.flag() {
                    Flag::PromoteCaptureN => KNIGHT_IDX,
                    Flag::PromoteCaptureB => BISHOP_IDX,
                    Flag::PromoteCaptureR => ROOK_IDX,
                    Flag::PromoteCaptureQ => QUEEN_IDX,
                    _ => panic!("Invalid move flag")
                };
            };
            self.piece_bb[piece_idx] |=  action.orig().bb();
        }

        self.total_ply -= 1;
        self.white_to_move = !self.white_to_move;
    }

    pub fn perft(&mut self, depth: u32) -> u128 {
        if depth == 0 { return 1 };

        let mut nodes = 0;
        for action in self.legal_moves() {
            let mut new_position = self.clone();
            new_position.make_move(&action);
            nodes += new_position.perft(depth - 1);
        }

        nodes
    }

    fn curr_flags(&self) -> BoardStateFlags {
        self.state_stack[self.total_ply as usize]
    }

    fn can_castle_short(&self) -> bool {
        let (flag, mask) = match self.white_to_move {
            true  => (self.curr_flags().can_castle_wk(), Bitboard::WK_MASK),
            false => (self.curr_flags().can_castle_bk(), Bitboard::BK_MASK)
        };

        flag && ((self.friend() | self.enemy()) & mask).is_empty()
    }

    fn can_castle_long(&self) -> bool {
        let (flag, mask) = match self.white_to_move {
            true  => (self.curr_flags().can_castle_wq(), Bitboard::WQ_MASK),
            false => (self.curr_flags().can_castle_bq(), Bitboard::BQ_MASK)
        };

        flag && ((self.friend() | self.enemy()) & mask).is_empty()
    }

    pub fn ep_target(&self) -> Option<Square> {
        self.curr_flags().ep_target()
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

impl fmt::Debug for BoardStateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let castle_bits = self.0 >> 28;
        let ep_exist_bit = (self.0 >> 27) & 0b1;
        let ep_square = (self.0 >> 26) & 0b1111_11;
        let half_move_clock = self.0 & 0b1_1111_1111_1111_1111_1111;
        write!(f, "{:04b} {:01b} {:08b} {:021b}", castle_bits, ep_exist_bit, ep_square, half_move_clock)
    }
}

impl fmt::Display for BoardStateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        output.push_str(&format!("{:^15}\n", format!("ep_targ {}",
            match self.ep_target() {
                Some(val) => format!("{}", val),
                None      => "--".to_string()
            }
        )));


        output.push_str(&format!("{:^15}\n",
            format!("hm_clock {}", self.half_moves())
        ));

        output.push_str(&format!("Castle legality\n     w    b\nk    {}    {}\nq    {}    {}\n",
            match self.can_castle_wk() {
                true  => "✓",
                false => "𐄂"
            },
            match self.can_castle_bk() {
                true  => "✓",
                false => "𐄂"
            },
            match self.can_castle_wq() {
                true  => "✓",
                false => "𐄂"
            },
            match self.can_castle_bq() {
                true  => "✓",
                false => "𐄂"
            },
        ));


        write!(f, "{}", output)
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
            state_stack: {
                let mut stack = [BoardStateFlags::NO_DATA; MAX_NUM_MOVES];

                stack[8] = BoardStateFlags::new(
                    true, true, true, true, None, 0
                );

                stack
            },
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
            state_stack: {
                let mut stack = [BoardStateFlags::NO_DATA; MAX_NUM_MOVES];

                stack[21] = BoardStateFlags::new(
                    false, false, false, false, Some(Square::G3), 0
                );

                stack
            },
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
            state_stack: {
                let mut stack = [BoardStateFlags::NO_DATA; MAX_NUM_MOVES];

                stack[76] = BoardStateFlags::new(
                    false, false, false, false, None, 3
                );

                stack
            },
            total_ply: 76
        };

        assert_eq!(output, expected_output);
    }

    fn test_perft(pos: &mut Board, expected_output: Vec<u128>) {
        for (i, expected) in expected_output.iter().enumerate() {
            let now = Instant::now();
            let output = pos.perft(i as u32);
            let elapsed = now.elapsed();

            println!("{}ms elapsed at ply={}", elapsed.as_millis(), i);
            assert_eq!(output, *expected);
        }
    }

    #[test]
    fn test_perft_start() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Initial_Position
        test_perft(
            &mut Board::STARTING_POSITION,
            vec![1, 20, 400, 8_902, 197_281, 4_865_609]
        );
    }

    #[test]
    fn test_perft_pos_2() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_2
        test_perft(
            &mut Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap(),
            vec![1, 48, 2_039, 97_862, 4_085_603, 193_690_690]
        );
    }
    
    #[test]
    fn test_perft_pos_3() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_3
        test_perft(
            &mut Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap(),
            vec![1, 14, 191, 2_812, 43_238, 674_624]
        );
    }

    #[test]
    fn test_perft_pos_4() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_4
        test_perft(
            &mut Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap(),
            vec![1, 6, 264, 9_467, 422_333, 15_833_292]
        );
    }

    #[test]
    fn test_perft_pos_5() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_5
        test_perft(
            &mut Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap(),
            vec![1, 44, 1_486, 62_379, 2_103_487, 89_941_194]
        );
    }

    #[test]
    fn test_perft_pos_6() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_6
        test_perft(
            &mut Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap(),
            vec![1, 46, 2_079, 89_890, 3_894_594, 164_075_551]
        );
    }
}
