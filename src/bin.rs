#![allow(unused_imports)]
#![allow(dead_code)]

use std::io;
use aster::bot::*;
use aster::types::{
    bitboard::*,
    square::*,
    movement::*
};
use aster::board::*;

fn print_legal_moves(board: &Board) {
    let legal_moves = board.clone().legal_moves();
    for (i, action) in legal_moves.iter().enumerate() {
        println!("{: <6}{}", i+1, action);
    }
}

fn print_perft_divide(board: &Board, depth: u32) {
    let perfts = board.clone().perft_divide(depth);
    let mut total: u128 = 0;

    println!("{:^58}", format!("Printing divided perft at {}", depth));
    println!(" {:^6} ║ {:^34} ║ {:^9} ", "move #", "move", "node count");
    println!("═{:═^6}═╬═{:═^34}═╬═{:═^9}══", "", "", "");
    for (i, perft) in perfts.iter().enumerate() {
        println!(" {:<6} ║ {:<34} ║ {:<9}", i+1,
            format!("{}", perft.0), perft.1
        );
        total += perft.1;
    }
    println!("═{:═^6}═╩═{:═^34}═╬═{:═^9}══", "", "", "");
    println!(" {:>43} ║ {:<9}", "Total", total);
}

fn explore_at_depth(mut board: Board, mut depth: u32) {
    let mut input = String::new();

    while input.trim().to_lowercase() != "quit" {
        input.clear();
        let legal_moves = board.legal_moves();
        println!("{}\n", board);
        if depth == 0 { break; }
        print_perft_divide(&board, depth);
        depth -= 1;

        println!("Enter move number");
        io::stdin().read_line(&mut input).unwrap();
        println!("You wrote: {}", input);

        let num = input.trim().parse::<usize>().unwrap();
        board.make_move(&legal_moves[num - 1]);
    }
}

fn start() {
    let bot = Bot::new();
    let mut buffer = String::new();

    loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Failed to read line.");
        let input = buffer.trim();
        if input == "quit" { break; }

        let output = bot.exec_command(input);
        if let Some(output) = output {
            println!("{output}");
        }
    }
}

fn main() -> std::io::Result<()> {
    /*let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ").unwrap();
    println!("{}", board.in_check());
    board.in_check();
    board.make_move(&Move::new(Square::A2, Square::A3, Flag::Quiet));
    board.make_move(&Move::new(Square::D7, Square::D6, Flag::Quiet));
    board.make_move(&Move::new(Square::E2, Square::B5, Flag::Quiet));
    explore_at_depth(board, 1);*/

    start();

    Ok(())
}
