#![allow(unused_imports)]
#![allow(dead_code)]

use std::io;
use aster::types::{
    bitboard::*,
    square::*
};
use aster::board::*;
use aster::legal_moves::*;
use aster::movement::*;

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

fn main() -> std::io::Result<()> {
    //println!("{:#?}", Board::from_fen("k7/8/3qb3/8/3N4/1pp5/8/7K w - - 0 1").unwrap());
    
    /*let mut board = Board::from_fen("k4p2/6P1/8/5p2/4Pp2/5P2/1PPP4/7K w - - 0 1").unwrap();
    let legal_moves = board.legal_moves();
    println!("{:#?}", board);
    board.make_move(&legal_moves[0]);
    println!("{:#?}", board);*/


    //let board = Board::from_fen("8/8/2k5/5N2/4PPP1/1r3KP1/4PPP1/8 w - - 0 20").unwrap();
    /*let mut board = Board::from_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1").unwrap();
    let mut legal_moves = board.legal_moves();
    println!("{:#?}\n", board);
    for (i, action) in legal_moves.iter().enumerate() {
        println!("{: <6}{}", i+1, action);
    }

    let mut board2 = Board::STARTING_POSITION;
    let action = &Move::new(Square::E2, Square::E4, Flag::DoublePush);
    println!("{:#?}", board2);
    board2.make_move(&Move::new(Square::E2, Square::E4, Flag::DoublePush));
    println!("{:#?}", board2);
    board2.unmake_move(action);
    println!("{:#?}", board2);*/

    /*for _ in 0..20 {
        println!("{:#?}\n", board);
        for (i, action) in legal_moves.iter().enumerate() {
            println!("{: <6}{}", i+1, action);
        }
        board.make_move(&legal_moves[0]);
        legal_moves = board.legal_moves();
    }

    let mut board = Board::STARTING_POSITION;
    let action_1 = &Move::new(Square::E2, Square::E4, Flag::DoublePush);
    let action_2 = &Move::new(Square::D7, Square::D5, Flag::DoublePush);
    let action_3 = &Move::new(Square::E4, Square::D5, Flag::Capture);
    let action_4 = &Move::new(Square::G7, Square::G6, Flag::Quiet);
    let action_5 = &Move::new(Square::G1, Square::F3, Flag::Quiet);

    println!("{}", board);

    board.make_move(action_1);
    board.make_move(action_2);
    board.make_move(action_3);
    board.make_move(action_4);
    board.make_move(action_5);

    println!("{}", board);

    board.unmake_move(action_5);
    board.unmake_move(action_4);
    board.unmake_move(action_3);
    board.unmake_move(action_2);
    board.unmake_move(action_1);

    println!("{}", board);*/

    //let mut board_1 = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap();
    //println!("{}", board_1);
    //let mut board_2 = board_1.clone();
    //board_2.perft(1);
    //println!("{}\n{}", board_1, board_2);
    //assert_eq!(board_1, board_2);
    
    //let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq -").unwrap();
    //board.make_move(&Move::SHORT_CASTLE);
    //board.make_move(&Move::new(Square::A2, Square::A3, Flag::Quiet));
    //println!("{}", board.can_castle_short());
    //println!("{}", board.curr_flags().can_castle_bk());
    //println!("{}", board.is_white_to_move());
    //board.make_move(&Move::new(Square::A1, Square::B1, Flag::Quiet));
    //board.make_move(&Move::new(Square::G6, Square::G5, Flag::Quiet));
    //println!("{}\n", board);
    //print_legal_moves(board);

    let mut board = Board::from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - ").unwrap();
    println!("{}", board.in_check());
    board.in_check();
    board.make_move(&Move::new(Square::A2, Square::A3, Flag::Quiet));
    board.make_move(&Move::new(Square::D7, Square::D6, Flag::Quiet));
    board.make_move(&Move::new(Square::E2, Square::B5, Flag::Quiet));
    explore_at_depth(board, 1);


    //board.make_move(&Move::new(Square::F1, Square::F2, Flag::Quiet));
    //board.make_move(&Move::new(Square::B2, Square::B1, Flag::PromoteR));
    //board.make_move(&Move::new(Square::D2, Square::D4, Flag::DoublePush));
    //board.make_move(&Move::LONG_CASTLE);
    //println!("{}\n", board);
    //print_perft_divide(&board, 4);
    //print_legal_moves(board);


    Ok(())
}
