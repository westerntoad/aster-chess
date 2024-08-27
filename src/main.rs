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


    //let board = Board::from_fen("8/8/2k5/5N2/4PPP1/1r3KP1/4PPP1/8 w - - 0 20").unwrap();
    let board = Board::STARTING_POSITION;
    let legal_moves = board.legal_moves();
    println!("{:#?}\n", board);
    for (i, action) in legal_moves.iter().enumerate() {
        println!("{: <6}{}", i+1, action);
    }
    //board.make_move(&legal_moves[0]);
    //println!("{:#?}", board);

    Ok(())
}
