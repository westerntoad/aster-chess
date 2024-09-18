use crate::types::{
    bitboard::Bitboard,
    square::Square,
    piece::Piece,
    state_metadata::BoardStateFlags
};

use super::movement::Move;
use super::movement::Flag;
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




#[derive(Clone)]
pub struct Board {
    color_bb: [Bitboard; 2],
    piece_bb: [Bitboard; 6],
    white_to_move: bool,
    state_stack: [BoardStateFlags; MAX_NUM_MOVES],
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
        state_stack: {
            let mut stack = [BoardStateFlags::NO_DATA; MAX_NUM_MOVES];

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

        let mut fen_components: Vec<&str> = fen.split_whitespace().collect();
        if fen_components.len() == 4 {
            fen_components.push("0");
            fen_components.push("1");
        } else if fen_components.len() != 6 {
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

    fn pawn_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let friend = self.color_bb[!(self.white_to_move) as usize];
        let enemy = self.color_bb[self.white_to_move as usize];

        let en_passant_bb = match self.ep_target() {
            Some(val) => val.bb(),
            None      => Bitboard::EMPTY
        };
        for pawn_sq in friend & self.piece_bb[PAWN_IDX] {
            let pawn_bb = pawn_sq.bb();
            for attack_sq in p_move_gen(pawn_bb, self.white_to_move, friend, enemy, en_passant_bb) {
                let attack_bb = attack_sq.bb();
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
                    vec![Flag::DoublePush]
                } else if !(attack_bb & enemy).is_empty() {
                    vec![Flag::Capture]
                } else if pawn_bb.file() != attack_bb.file() {
                    vec![Flag::EnPassant]
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

        for knight_sq in friend & self.piece_bb[KNIGHT_IDX] {
            for attack_sq in n_move_gen(knight_sq) & !friend {
                let attack_bb = attack_sq.bb();
                moves.push(Move::new(
                    knight_sq,
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

        for bishop_sq in friend & self.piece_bb[BISHOP_IDX] {
            for attack_sq in b_move_gen(bishop_sq, friend | enemy) & !friend {
                let attack_bb = attack_sq.bb();
                moves.push(Move::new(
                    bishop_sq,
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

        for rook_sq in friend & self.piece_bb[ROOK_IDX] {
            for attack_sq in r_move_gen(rook_sq, friend | enemy) & !friend {
                let attack_bb = attack_sq.bb();
                moves.push(Move::new(
                    rook_sq,
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

        for queen_sq in friend & self.piece_bb[QUEEN_IDX] {
            for attack_sq in q_move_gen(queen_sq, friend | enemy) & !friend {
                let attack_bb = attack_sq.bb();
                moves.push(Move::new(
                    queen_sq,
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

        for king_sq in friend & self.piece_bb[KING_IDX] {
            for attack_sq in k_move_gen(king_sq) & !friend {
                let attack_bb = attack_sq.bb();
                moves.push(Move::new(
                    king_sq,
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

    pub fn moves(&mut self) -> Vec<Move> {
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
            let mut king_bb = self.piece_bb[KING_IDX] & self.enemy();
            king_bb |= match (self.white_to_move, x.flag()) {
                (true, Flag::ShortCastle) => Bitboard::BK_MASK,
                (true, Flag::LongCastle) => Bitboard::BQ_MASK_SLIDE,
                (false, Flag::ShortCastle) => Bitboard::WK_MASK,
                (false, Flag::LongCastle) => Bitboard::WQ_MASK_SLIDE,
                _ => Bitboard::EMPTY
            };

            /*for action in &self.moves() {
                if !(action.targ().bb() & king_bb).is_empty() {
                    self.unmake_move(x);
                    return false;
                }
            }*/
            //println!("attacks for move {} = {:?}\nking_bb = {:?}", x, self.attacks(), king_bb);
            let is_legal = (self.attacks() & king_bb).is_empty();
            self.unmake_move(x);
            is_legal
        }).collect::<Vec<Move>>()
    }

    pub fn make_move(&mut self, action: &Move) {
        let action_flag = action.flag();
        let is_short_castle = action_flag == Flag::ShortCastle;
        let is_pawn_move = !(self.piece_bb[PAWN_IDX] & action.orig().bb()).is_empty();
        let (disable_wk, disable_wq, disable_bk, disable_bq) = {
            let (short_rook_home, long_rook_home, king_home) = match self.white_to_move {
                true  => (Square::H1, Square::A1, Square::E1),
                false => (Square::H8, Square::A8, Square::E8)
            };

            let disable_all = action.orig() == king_home || action.is_castle();
            let kingside = action.orig() == short_rook_home || disable_all;
            let queenside = action.orig() == long_rook_home || disable_all;

            match self.white_to_move {
                true  => ( kingside, queenside, false, false ),
                false => ( false, false, kingside, queenside )
            }
        };

        let mut captured_bb = Bitboard::EMPTY;
        let mut captured_piece = Piece::NoPiece;

        if action.is_castle() {
            let (orig_rook, dest_rook, orig_king, dest_king) = match (self.white_to_move, is_short_castle) {
                (true, true)   => (Bitboard::WK_ROOK, Bitboard::WK_ROOK_DEST, Bitboard::W_KING, Bitboard::WK_KING_DEST),
                (true, false)  => (Bitboard::WQ_ROOK, Bitboard::WQ_ROOK_DEST, Bitboard::W_KING, Bitboard::WQ_KING_DEST),
                (false, true)  => (Bitboard::BK_ROOK, Bitboard::BK_ROOK_DEST, Bitboard::B_KING, Bitboard::BK_KING_DEST),
                (false, false) => (Bitboard::BQ_ROOK, Bitboard::BQ_ROOK_DEST, Bitboard::B_KING, Bitboard::BQ_KING_DEST)
            };
            
            self.color_bb[!self.white_to_move as usize] &= !( orig_king | orig_rook );
            self.color_bb[!self.white_to_move as usize] |=    dest_king | dest_rook;

            self.piece_bb[ROOK_IDX] &= !orig_rook;
            self.piece_bb[ROOK_IDX] |=  dest_rook;
            self.piece_bb[KING_IDX] &= !orig_king;
            self.piece_bb[KING_IDX] |=  dest_king;
        } else if action.is_en_passant() {
            captured_bb = match self.white_to_move {
                true  => action.targ().bb().sout_one(),
                false => action.targ().bb().nort_one()
            };
            captured_piece = Piece::Pawn;

            self.color_bb[!self.white_to_move as usize] &= !action.orig().bb();
            self.color_bb[!self.white_to_move as usize] |=  action.targ().bb();
            self.color_bb[ self.white_to_move as usize] &= !captured_bb;

            self.piece_bb[PAWN_IDX] &= !(action.orig().bb() | captured_bb);
            self.piece_bb[PAWN_IDX] |=   action.targ().bb();
        } else {
            self.color_bb[!self.white_to_move as usize] &= !action.orig().bb();
            self.color_bb[!self.white_to_move as usize] |=  action.targ().bb();
            self.color_bb[ self.white_to_move as usize] &= !action.targ().bb();

            let mut orig_piece_idx = self.find_piece(action.orig().bb()).index();
            self.piece_bb[orig_piece_idx] &= !action.orig().bb();
            if action.is_promotion() {
                orig_piece_idx = match action.flag() {
                    Flag::PromoteCaptureN | Flag::PromoteN => KNIGHT_IDX,
                    Flag::PromoteCaptureB | Flag::PromoteB => BISHOP_IDX,
                    Flag::PromoteCaptureR | Flag::PromoteR => ROOK_IDX,
                    Flag::PromoteCaptureQ | Flag::PromoteQ => QUEEN_IDX,
                    _ => panic!("Invalid move flag")
                };
            };

            captured_piece = self.find_piece(action.targ().bb());
            if captured_piece.is_piece() {
                self.piece_bb[captured_piece.index()] &= !action.targ().bb();
            }
            self.piece_bb[orig_piece_idx] |= action.targ().bb();
        }
        
        self.state_stack[self.total_ply as usize + 1] = self.curr_flags().next_move(
            action.is_capture() && is_pawn_move,
            disable_wk,
            disable_wq,
            disable_bk,
            disable_bq,
            action.ep_square(),
            captured_piece,
            action.targ()
        );
        self.total_ply += 1;
        self.white_to_move = !self.white_to_move;
    }

    pub fn unmake_move(&mut self, action: &Move) {
        if action.is_castle() {
            let (orig_rook, dest_rook, orig_king, dest_king) = match (self.white_to_move, action.flag() == Flag::ShortCastle) {
                (true, true)   => (Bitboard::BK_ROOK_DEST, Bitboard::BK_ROOK, Bitboard::BK_KING_DEST, Bitboard::B_KING),
                (true, false)  => (Bitboard::BQ_ROOK_DEST, Bitboard::BQ_ROOK, Bitboard::BQ_KING_DEST, Bitboard::B_KING),
                (false, true)  => (Bitboard::WK_ROOK_DEST, Bitboard::WK_ROOK, Bitboard::WK_KING_DEST, Bitboard::W_KING),
                (false, false) => (Bitboard::WQ_ROOK_DEST, Bitboard::WQ_ROOK, Bitboard::WQ_KING_DEST, Bitboard::W_KING)
            };

            self.color_bb[ self.white_to_move as usize] &= !( orig_king | orig_rook );
            self.color_bb[ self.white_to_move as usize] |=    dest_king | dest_rook;

            self.piece_bb[ROOK_IDX] &= !orig_rook;
            self.piece_bb[ROOK_IDX] |=  dest_rook;
            self.piece_bb[KING_IDX] &= !orig_king;
            self.piece_bb[KING_IDX] |=  dest_king;
        } else if action.is_en_passant() {
            let captured_pawn_bb = match !self.white_to_move {
                true  => action.targ().bb().sout_one(),
                false => action.targ().bb().nort_one()
            };

            self.color_bb[ self.white_to_move as usize] &= !action.targ().bb();
            self.color_bb[ self.white_to_move as usize] |=  action.orig().bb();
            self.color_bb[!self.white_to_move as usize] |=  captured_pawn_bb;

            self.piece_bb[PAWN_IDX] |=  action.orig().bb() | captured_pawn_bb;
            self.piece_bb[PAWN_IDX] &= !action.targ().bb();
        } else {
            self.color_bb[ self.white_to_move as usize] &= !action.targ().bb();
            self.color_bb[ self.white_to_move as usize] |=  action.orig().bb();
            self.color_bb[!self.white_to_move as usize] &= !action.orig().bb();

            let targ_piece = self.find_piece(action.targ().bb());
            let mut targ_piece_idx = targ_piece.index();
            self.piece_bb[targ_piece_idx] &= !action.targ().bb();
            if action.is_promotion() {
                targ_piece_idx = PAWN_IDX;
            };
            self.piece_bb[targ_piece_idx] |= action.orig().bb();
            
            let prev_flag = self.curr_flags();
            if prev_flag.captured_piece().is_piece() {
                self.color_bb[!self.white_to_move as usize] |= prev_flag.captured_piece_sq().bb();
                self.piece_bb[prev_flag.captured_piece().index()] |= prev_flag.captured_piece_sq().bb();
            }
        }

        self.total_ply -= 1;
        self.white_to_move = !self.white_to_move;
    }

    fn find_piece(&self, loc_bb: Bitboard) -> Piece {
        let mut piece_idx = 99u8;

        for (i, piece_bb_idx) in self.piece_bb.iter().enumerate() {
            if !(*piece_bb_idx & loc_bb).is_empty() {
                piece_idx = i as u8;
                break;
            }
        }

        match piece_idx {
            0..=5 => Piece::new(piece_idx + 1).unwrap(),
            _     => Piece::NoPiece
        }
    }

    pub fn perft(&mut self, depth: u32) -> (u128, u128, u128, u128, u128) {
        let (mut captures, mut en_passants, mut castles, mut promotions) = (0u128, 0u128, 0u128, 0u128);
        let nodes = self.perft_helper(depth, &mut captures, &mut en_passants, &mut castles, &mut promotions);

        (nodes, captures, en_passants, castles, promotions)
    }

    pub fn perft_divide(&mut self, depth: u32) -> Vec<(Move, u128)> {
        if depth == 0 { return vec![]; }

        let moves = self.legal_moves();
        let mut perfts = Vec::new();

        for action in moves {
            self.make_move(&action);
            perfts.push((action.clone(), self.perft_divide_helper(depth - 1)));
            self.unmake_move(&action);
        }

        perfts
    }

    fn perft_divide_helper(&mut self, depth: u32) -> u128 {
        let moves = self.legal_moves();
        let n = moves.len() as u128;
        let mut nodes = 0;

        if depth == 0 { return 1; }
        if depth == 1 { return n; }

        for action in self.legal_moves() {
            self.make_move(&action);
            nodes += self.perft_divide_helper(depth - 1);
            self.unmake_move(&action);
        }

        nodes
    }

    fn perft_helper(
            &mut self,
            depth: u32,
            captures: &mut u128,
            en_passants: &mut u128,
            castles: &mut u128,
            promotions: &mut u128) -> u128 {
        
        if depth == 0 { return 1 };

        let mut nodes = 0;
        for action in self.legal_moves() {

            self.make_move(&action);

            if action.is_capture() { *captures += 1; }
            if action.is_en_passant() { *en_passants += 1; }
            if action.is_castle() { *castles += 1; }
            if action.is_promotion() { *promotions += 1; }

            nodes += self.perft_helper(depth - 1, captures, en_passants, castles, promotions);
            self.unmake_move(&action);
        }

        nodes
    }

    pub fn in_check(&mut self) -> bool {
        self.white_to_move = !self.white_to_move;
        let attacks = self.attacks();
        self.white_to_move = !self.white_to_move;

        !(attacks & self.piece_bb[KING_IDX] & self.friend()).is_empty()
    }

    fn in_checkmate(&mut self) -> bool {
        self.in_check() && self.legal_moves().len() == 0
    }

    fn enemy_in_check(&mut self) -> bool {
        self.white_to_move = !self.white_to_move;
        let in_check = self.in_check();
        self.white_to_move = !self.white_to_move;

        in_check
    }

    fn enemy_in_checkmate(&mut self) -> bool {
        self.white_to_move = !self.white_to_move;
        let in_checkmate = self.in_checkmate();
        self.white_to_move = !self.white_to_move;

        in_checkmate
    }

    pub fn attacks(&self) -> Bitboard {
        let mut attacks = Bitboard::EMPTY;

        let mut moves: Vec<Move> = Vec::with_capacity(256);
        moves.extend(self.pawn_moves());
        moves.extend(self.knight_moves());
        moves.extend(self.bishop_moves());
        moves.extend(self.rook_moves());
        moves.extend(self.queen_moves());
        moves.extend(self.king_moves());

        for action in moves {
            attacks |= action.targ().bb();
        }

        attacks
    }

    pub fn curr_flags(&self) -> BoardStateFlags {
        self.state_stack[self.total_ply as usize]
    }

    pub fn can_castle_short(&mut self) -> bool {
        let (flag, mask, rook_home, king_home) = match self.white_to_move {
            true  => (self.curr_flags().can_castle_wk(), Bitboard::WK_MASK, Bitboard::WK_ROOK, Bitboard::W_KING),
            false => (self.curr_flags().can_castle_bk(), Bitboard::BK_MASK, Bitboard::BK_ROOK, Bitboard::B_KING)
        };
        let none_in_mask = ((self.friend() | self.enemy()) & mask).is_empty();
        let king_is_home = !(king_home & self.friend() & self.piece_bb[KING_IDX]).is_empty();
        let rook_is_home = !(rook_home & self.friend() & self.piece_bb[ROOK_IDX]).is_empty();

        flag && none_in_mask && king_is_home && rook_is_home && !self.in_check()
    }

    pub fn can_castle_long(&mut self) -> bool {
        let (flag, mask, rook_home, king_home) = match self.white_to_move {
            true  => (self.curr_flags().can_castle_wq(), Bitboard::WQ_MASK, Bitboard::WQ_ROOK, Bitboard::W_KING),
            false => (self.curr_flags().can_castle_bq(), Bitboard::BQ_MASK, Bitboard::BQ_ROOK, Bitboard::B_KING)
        };
        let none_in_mask = ((self.friend() | self.enemy()) & mask).is_empty();
        let king_is_home = !(king_home & self.friend() & self.piece_bb[KING_IDX]).is_empty();
        let rook_is_home = !(rook_home & self.friend() & self.piece_bb[ROOK_IDX]).is_empty();

        flag && none_in_mask && king_is_home && rook_is_home && !self.in_check()
    }

    pub fn is_white_to_move(&self) -> bool {
        self.white_to_move
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

impl PartialEq for Board {
    fn eq(&self, other: &Self) -> bool {
        self.color_bb == other.color_bb
            && self.piece_bb == other.piece_bb
            && self.white_to_move == other.white_to_move
            && self.state_stack[self.total_ply as usize] == other.state_stack[other.total_ply as usize]
            && self.total_ply == other.total_ply
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        output.push_str("    color_bb\n");
        for bb in self.color_bb {
            output.push_str(&format!("{:#018x}\n", bb.val()));
        }

        output.push_str("    piece_bb\n");
        for bb in self.piece_bb {
            output.push_str(&format!("{:#018x}\n", bb.val()));
        }

        output.push_str(&format!("{:?}\n", self.curr_flags()));

        write!(f, "{}", output)
    }
}

impl fmt::Display for Board {
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

        output.push_str(" 8 ");
        for (i, element) in board_arr.iter().enumerate() {
            output.push(*element);
            
            if i % 8 != 7 {
                output.push(' ');
            } else {
                output.push('\n');
                
                output.push_str( match i {
                    7  => " 7 ",
                    15 => " 6 ",
                    23 => " 5 ",
                    31 => " 4 ",
                    39 => " 3 ",
                    47 => " 2 ",
                    55 => " 1 ",
                    _ => ""
                });
            }
        }
        output.push_str("   a b c d e f g h\n\n");

        output.push_str(&format!(" {:^18}\n",
            format!("{} to move", match self.white_to_move {
                true  => "White",
                false => "Black",
            })
        ));

        output.push_str(&format!(" {:^18}\n",
            format!("{} ply", self.total_ply)
        ));

        output.push_str(&format!("{}", self.curr_flags()));
        
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
                    true, true, true, true, None, Piece::NoPiece, Square::A1, 0
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
                    false, false, false, false, Some(Square::G3), Piece::NoPiece, Square::A1, 0
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
                    false, false, false, false, None, Piece::NoPiece, Square::A1, 3
                );

                stack
            },
            total_ply: 76
        };

        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_castle_out_of_check() {
        let mut board = Board::from_fen("r3k2r/8/2B5/8/8/8/8/2K5 b kq - 1 1").unwrap();
        let legal_moves = board.legal_moves();

        assert!(!legal_moves.contains(&Move::SHORT_CASTLE));
        assert!(!legal_moves.contains(&Move::LONG_CASTLE));
    }

    #[test]
    fn test_unmake_move_legal() {
        let original = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap();
        let mut modified = original.clone();
        modified.legal_moves();

        println!("original = {}\n\n modified = {}", original, modified);
        assert_eq!(original, modified);
    }

    #[test]
    fn test_unmake_move_pawn() {
        let mut board = Board::STARTING_POSITION;
        let pawn_push = &Move::new(Square::E2, Square::E4, Flag::DoublePush);

        board.make_move(pawn_push);
        board.unmake_move(pawn_push);

        assert_eq!(board, Board::STARTING_POSITION);
    }

    #[test]
    fn test_unmake_move_comp() {
        let mut board = Board:: STARTING_POSITION;
        let action_1 = &Move::new(Square::C2, Square::C4, Flag::DoublePush);
        let action_2 = &Move::new(Square::E7, Square::E6, Flag::Quiet);
        let action_3 = &Move::new(Square::D1, Square::A4, Flag::Quiet);
        let action_4 = &Move::new(Square::B8, Square::C6, Flag::Quiet);
        let action_5 = &Move::new(Square::A4, Square::C6, Flag::Capture);

        board.make_move(action_1);
        board.make_move(action_2);
        board.make_move(action_3);
        board.make_move(action_4);
        board.make_move(action_5);

        board.unmake_move(action_5);
        board.unmake_move(action_4);
        board.unmake_move(action_3);
        board.unmake_move(action_2);
        board.unmake_move(action_1);

        assert_eq!(board, Board::STARTING_POSITION);
    }

    fn test_perft(pos: &mut Board, expected_output: Vec<(u128, u128, u128, u128, u128)>) {
        for (i, expected) in expected_output.iter().enumerate() {
            let mut duplicate = pos.clone();
            let now = Instant::now();
            let output = duplicate.perft(i as u32 + 1);
            let elapsed = now.elapsed();

            println!("{}ms elapsed at ply={}", elapsed.as_millis(), i+1);
            println!("         |{:^9}|{:^9}|{:^9}|{:^9}|{:^9}", "nodes", "captures", "eps", "castles", "promotes");
            println!("Expected |{:<9}|{:<9}|{:<9}|{:<9}|{:<9}", expected.0, expected.1, expected.2, expected.3, expected.4);
            println!("Result   |{:<9}|{:<9}|{:<9}|{:<9}|{:<9}\n", output.0, output.1, output.2, output.3, output.4);
            assert_eq!(duplicate, *pos);
            assert_eq!(output, *expected);
        }
    }

    fn test_shallow_perft(pos: &mut Board, expected_output: Vec<(u128, u128, u128, u128, u128)>) {
        for (i, expected) in expected_output.iter().enumerate() {
            let mut duplicate = pos.clone();
            let now = Instant::now();
            let output = duplicate.perft(i as u32 + 1);
            let elapsed = now.elapsed();

            println!("{}ms elapsed at ply={}", elapsed.as_millis(), i+1);
            assert_eq!(duplicate, *pos);
            assert_eq!(output.0, expected.0);
        }
    }



    #[test]
    fn test_perft_start() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Initial_Position
        test_shallow_perft(
            &mut Board::STARTING_POSITION.clone(),
            vec![
            /*     nodes     captures      ep       castle    promote  */
                (20       , 0        , 0        , 0        , 0        ), 
                (400      , 0        , 0        , 0        , 0        ), 
                (8902     , 34       , 0        , 0        , 0        ), 
                (197281   , 1576     , 0        , 0        , 0        ), 
                (4865609  , 82719    , 258      , 0        , 0        )
            ]
        );
    }

    #[test]
    fn test_perft_pos_2() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_2
        test_shallow_perft(
            &mut Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap(),
            vec![
            /*     nodes     captures      ep       castle    promote  */
                (48       , 8        , 0        , 2        , 0        ), 
                (2039     , 351      , 1        , 91       , 0        ), 
                (97862    , 17102    , 45       , 3162     , 0        ), 
                (4085603  , 757163   , 1929     , 128013   , 15172    ), 
                (193690690, 35043416 , 73365    , 4993637  , 8392     )
            ]
        );
    }
    
    #[test]
    fn test_perft_pos_3() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_3
        test_shallow_perft(
            &mut Board::from_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - -").unwrap(),
            vec![
            /*     nodes     captures      ep       castle    promote  */
                (14       , 1        , 0        , 0        , 0        ), 
                (191      , 2812     , 209      , 0        , 0        ), 
                (2812     , 209      , 2        , 0        , 0        ), 
                (43238    , 3348     , 123      , 0        , 0        ), 
                (674624   , 52051    , 1165     , 0        , 0        )
            ]
        );
    }

    #[test]
    fn test_perft_pos_4() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_4
        test_shallow_perft(
            &mut Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap(),
            vec![
            /*     nodes     captures      ep       castle    promote  */
                (6        , 0        , 0        , 0        , 0        ), 
                (264      , 87       , 0        , 6        , 48       ), 
                (9467     , 1021     , 4        , 0        , 120      ), 
                (422333   , 131393   , 0        , 7795     , 60032    ), 
                (15833292 , 2047173  , 6512     , 0        , 329464   )
            ]
        );
    }

    /*#[test]
    fn test_perft_pos_5() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_5
        test_perft(
            &mut Board::from_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8").unwrap(),
            vec![
            /*     nodes     captures      ep       castle    promote  */
                (         ,          ,          ,          ,          ), 
                (         ,          ,          ,          ,          ), 
                (         ,          ,          ,          ,          ), 
                (         ,          ,          ,          ,          ), 
                (         ,          ,          ,          ,          )
            ]
        );
    }

    #[test]
    fn test_perft_pos_6() {
        // data found at:
        // https://www.chessprogramming.org/Perft_Results#Position_6
        test_perft(
            &mut Board::from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10").unwrap(),
            vec![
            /*     nodes     captures      ep       castle    promote  */
                (         ,          ,          ,          ,          ), 
                (         ,          ,          ,          ,          ), 
                (         ,          ,          ,          ,          ), 
                (         ,          ,          ,          ,          ), 
                (         ,          ,          ,          ,          )
            ]
        );
    }*/
}
