use core::fmt;
use crate::types::{
    piece::Piece,
    square::Square
};

#[derive(Copy, Clone, PartialEq)]
pub struct BoardStateFlags(u32);

impl BoardStateFlags {

    pub const STARTING: Self = Self(0b1_1111_0_000000_000_000000_00000000000);
    pub const NO_DATA:  Self = Self(0);

    const HAS_DATA_MASK:      u32 = 0b1_0000_0_000000_000_000000_00000000000;
    const CASTLE_WK_MASK:     u32 = 0b0_1000_0_000000_000_000000_00000000000;
    const CASTLE_WQ_MASK:     u32 = 0b0_0100_0_000000_000_000000_00000000000;
    const CASTLE_BK_MASK:     u32 = 0b0_0010_0_000000_000_000000_00000000000;
    const CASTLE_BQ_MASK:     u32 = 0b0_0001_0_000000_000_000000_00000000000;
    const CASTLE_BITS_MASK:   u32 = 0b0_1111_0_000000_000_000000_00000000000;
    const EP_TARG_EXIST_MASK: u32 = 0b0_0000_1_000000_000_000000_00000000000;
    const EP_TARG_MASK:       u32 = 0b0_0000_0_111111_000_000000_00000000000;
    const EP_MASK:            u32 = 0b0_0000_1_111111_000_000000_00000000000;
    const CAPTURED_PIECE_MASK:u32 = 0b0_0000_0_000000_111_000000_00000000000;
    const PREV_SQUARE_MASK:   u32 = 0b0_0000_0_000000_000_111111_00000000000;
    const CAPTURED_MASK:      u32 = 0b0_0000_0_000000_111_111111_00000000000;
    const HALF_MOVE_MASK:     u32 = 0b0_0000_0_000000_000_000000_11111111111;

    const HAS_DATA_PAD:       u32 = Self::HAS_DATA_MASK.trailing_zeros();
    const CASTLE_WK_PAD:      u32 = Self::CASTLE_WK_MASK.trailing_zeros();
    const CASTLE_WQ_PAD:      u32 = Self::CASTLE_WQ_MASK.trailing_zeros();
    const CASTLE_BK_PAD:      u32 = Self::CASTLE_BK_MASK.trailing_zeros();
    const CASTLE_BQ_PAD:      u32 = Self::CASTLE_BQ_MASK.trailing_zeros();
    const CASTLE_BITS_PAD:    u32 = Self::CASTLE_BITS_MASK.trailing_zeros();
    const EP_TARG_EXIST_PAD:  u32 = Self::EP_TARG_EXIST_MASK.trailing_zeros();
    const EP_TARG_PAD:        u32 = Self::EP_TARG_MASK.trailing_zeros();
    const CAPTURED_PIECE_PAD: u32 = Self::CAPTURED_PIECE_MASK.trailing_zeros();
    const PREV_SQUARE_PAD:    u32 = Self::PREV_SQUARE_MASK.trailing_zeros();
    const HALF_MOVE_PAD:      u32 = Self::HALF_MOVE_MASK.trailing_zeros();

    fn set_bit(val: &mut u32, mask: u32, state: bool) {
        match state {
            true  => *val |=  mask,
            false => *val &= !mask
        };
    }

    pub fn new(
            can_castle_wk: bool,
            can_castle_wq: bool,
            can_castle_bk: bool,
            can_castle_bq: bool,
            en_passant_target: Option<Square>,
            captured_piece: Piece,
            captured_square: Square,
            half_move_clock: u32
        ) -> Self {
        let mut val = 0u32;

        Self::set_bit(&mut val, Self::HAS_DATA_MASK, true);

        Self::set_bit(&mut val, Self::CASTLE_WK_MASK, can_castle_wk);
        Self::set_bit(&mut val, Self::CASTLE_WQ_MASK, can_castle_wq);
        Self::set_bit(&mut val, Self::CASTLE_BK_MASK, can_castle_bk);
        Self::set_bit(&mut val, Self::CASTLE_BQ_MASK, can_castle_bq);

        match en_passant_target {
            Some(sq) => {
                Self::set_bit(&mut val, Self::EP_TARG_EXIST_MASK, true);
                val += (sq.val() as u32) << Self::EP_TARG_PAD;
            },
            None => Self::set_bit(&mut val, Self::EP_TARG_EXIST_MASK, false)
        }

        if captured_piece.is_piece() {
            val += (captured_piece        as u32) << Self::CAPTURED_PIECE_PAD;
            val += (captured_square.val() as u32) << Self::PREV_SQUARE_PAD;
        }

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
                ep_target: Option<Square>,
                captured_piece: Piece,
                captured_square: Square
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
            Some(sq) => {
                new.0 |= Self::EP_TARG_EXIST_MASK;
                new.0 += (sq.val() as u32) << Self::EP_TARG_PAD;
            }
            None     => ()
        };

        new.0 &= !Self::CAPTURED_MASK;
        if captured_piece.is_piece() {
            new.0 += (captured_piece        as u32) << Self::CAPTURED_PIECE_PAD;
            new.0 += (captured_square.val() as u32) << Self::PREV_SQUARE_PAD;
        }
        
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
        Square::new(((self.0 & Self::EP_TARG_MASK) >> Self::EP_TARG_PAD) as u8).unwrap()
    }

    pub fn ep_target(&self) -> Option<Square> {
        match self.has_ep_target() {
            true  => Some(self.ep_target_unchecked()),
            false => None
        }
    }

    pub fn captured_piece(&self) -> Piece {
        let val = ((self.0 & Self::CAPTURED_PIECE_MASK) >> Self::CAPTURED_PIECE_PAD) as u8;

        Piece::new(val).unwrap()
    }

    pub fn captured_piece_sq(&self) -> Square {
        Square::new(((self.0 & Self::PREV_SQUARE_MASK) >> Self::PREV_SQUARE_PAD) as u8).unwrap()
    }

    pub fn half_moves(&self) -> u32 {
        self.0 & Self::HALF_MOVE_MASK
    }

    pub fn fifty_move_rule(&self) -> bool {
        self.half_moves() >= 100
    }
}



impl fmt::Debug for BoardStateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let has_data_bit = (self.0 & Self::HAS_DATA_MASK) >> Self::HAS_DATA_PAD;
        let castle_bits = (self.0 & Self::CASTLE_BITS_MASK) >> Self::CASTLE_BITS_PAD;
        let ep_exist_bit = (self.0 & Self::EP_TARG_EXIST_MASK) >> Self::EP_TARG_EXIST_PAD;
        let ep_square = (self.0 & Self::EP_TARG_MASK) >> Self::EP_TARG_PAD;
        let captured_piece = self.captured_piece();
        let captured_piece_sq = self.captured_piece_sq();
        let half_move_clock = self.0 & Self::HALF_MOVE_MASK;
        write!(f, "{:b} {:04b} {:01b} {:08b} {:?} {} {}",
            has_data_bit,
            castle_bits,
            ep_exist_bit,
            ep_square,
            captured_piece,
            captured_piece_sq,
            half_move_clock
        )
    }
}

impl fmt::Display for BoardStateFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        output.push_str(&format!(" {:^18}\n", format!("ep_targ {}",
            match self.ep_target() {
                Some(val) => format!("{}", val),
                None      => "--".to_string()
            }
        )));
        
        output.push_str(&format!(" {:^18}\n", "prev_cap"));

        let prev_cap = {
            let captured_piece = self.captured_piece();

            if captured_piece.is_piece() {
                format!("{:?} on {}", captured_piece, self.captured_piece_sq())
            } else {
                format!("{:?}", captured_piece)
            }
        };

        output.push_str(&format!(" {:^18}\n", prev_cap));

        output.push_str(&format!(" {:^18}\n",
            format!("hm_clock {}", self.half_moves())
        ));

        output.push_str(&format!("\nCastle legality\n     w    b\nk    {}    {}\nq    {}    {}\n",
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

