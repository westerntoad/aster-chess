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
