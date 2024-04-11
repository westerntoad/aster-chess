#![allow(unused_imports)]
#![allow(dead_code)]

mod bitboard;
mod board;
mod movement;
mod square;

use crate::bitboard::*;
use crate::board::*;
use crate::movement::*;
use crate::square::*;

fn main() {
    println!("{:?}", Board::from_fen("8/5k2/1p1p2p1/3Pnb1p/2P2b1P/5P2/3qBP2/4NKRQ w - - 3 39").unwrap());
}
