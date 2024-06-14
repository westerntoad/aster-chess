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

    let bb = n_move_gen(Square::E4.bb());

    println!("{:?}", bb);


    Ok(())
}
