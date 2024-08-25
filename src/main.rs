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
    //println!("{:#?}", Board::from_fen("k7/8/3qb3/8/3N4/1pp5/8/7K w - - 0 1").unwrap());
    
    /*let mut board = Board::from_fen("k4p2/6P1/8/5p2/4Pp2/5P2/1PPP4/7K w - - 0 1").unwrap();
    let legal_moves = board.legal_moves();
    println!("{:#?}", board);
    board.make_move(&legal_moves[0]);
    println!("{:#?}", board);*/


    let mut board = Board::from_fen("7k/8/8/8/2K1N1r1/8/8/8 w - - 0 1").unwrap();
    let legal_moves = board.legal_moves();
    println!("{:#?}", board);
    board.make_move(&legal_moves[0]);
    println!("{:#?}", board);

    Ok(())
}
