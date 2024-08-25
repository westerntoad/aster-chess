#![allow(unused_imports)]
#![allow(dead_code)]

mod bitboard;
mod board;
mod legal_moves;
mod movement;
mod square;

use crate::bitboard::*;
use crate::board::*;
use crate::legal_moves::*;
use crate::movement::*;
use crate::square::*;

fn main() -> std::io::Result<()> {


    println!("{:#?}", Board::from_fen("k7/8/3qb3/8/3N4/1pp5/8/7K w - - 0 1").unwrap());

    Ok(())
}
