pub mod move_gen;
pub mod fen;

use crate::types::{
    bitboard::Bitboard,
    square::Square,
    piece::Piece,
    state_metadata::BoardStateFlags,
    movement::Move,
    movement::Flag
};

use move_gen::{
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
            let captured_bb = match self.white_to_move {
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

    pub fn get_move_from_algebraic(&self, source_str: &str) -> Result<Move, &'static str> {
        assert!(source_str.len() == 4 || source_str.len() == 5);
        let orig_sq = Square::from_algebraic(&source_str[0..2])?;
        let dest_sq = Square::from_algebraic(&source_str[2..4])?;
        let flag = {
            let orig_is_white = !(orig_sq.bb() & self.color_bb[WHITE_IDX]).is_empty();
            let is_capture = !(dest_sq.bb() & self.color_bb[orig_is_white as usize]).is_empty();
            let orig_piece = self.find_piece(orig_sq.bb());

            if orig_piece == Piece::Pawn && Some(dest_sq) == self.ep_target() {
                Flag::EnPassant
            } else if source_str.len() == 5 {
                let capture_char = source_str.chars().last().expect("nvalid source string size.");

                if is_capture {
                    match capture_char {
                        'n' => Flag::PromoteCaptureN,
                        'b' => Flag::PromoteCaptureB,
                        'r' => Flag::PromoteCaptureR,
                        'q' => Flag::PromoteCaptureQ,
                        _ => panic!()
                    }
                } else {
                    match capture_char {
                        'n' => Flag::PromoteN,
                        'b' => Flag::PromoteB,
                        'r' => Flag::PromoteR,
                        'q' => Flag::PromoteQ,
                        _ => panic!()
                    }
                }
            } else if orig_piece == Piece::Pawn
                        && (orig_sq.rank() == Square::RANK_2 || orig_sq.rank() == Square::RANK_7) {

                Flag::DoublePush
            } else if orig_piece == Piece::King && orig_sq.file() == Square::E_FILE
                        && (dest_sq.file() == Square::G_FILE || dest_sq.file() == Square::C_FILE) {

                if dest_sq.file() == Square::G_FILE {
                    Flag::ShortCastle
                } else {
                    Flag::LongCastle
                }
            } else if is_capture {
                Flag::Capture
            } else {
                Flag::Quiet
            }
        };

        Ok(Move::new(orig_sq, dest_sq, flag))
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
mod tests;
