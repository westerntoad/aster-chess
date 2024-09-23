use std::str::SplitWhitespace;
use std::time::Duration;
use rand::seq::SliceRandom;
use crate::board::Board;
use crate::types::movement::Move;

pub struct Bot {
    board: Board
}

fn print_legal_moves(board: &Board) {
    let legal_moves = board.clone().legal_moves();
    for (i, action) in legal_moves.iter().enumerate() {
        println!("{: <6}{}", i+1, action);
    }
}


impl Bot {
    const NAME:     &'static str = env!("CARGO_PKG_NAME");
    const VERSION:  &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS:  &'static str = env!("CARGO_PKG_AUTHORS");
    const UCI_OK:   &'static str = "uciok";
    const READY_OK: &'static str = "readyok";

    pub fn new() -> Self {
        Self {
            board: Board::STARTING_POSITION
        }
    }

    pub fn exec_command(&mut self, command: &str) -> Option<String> {
        let mut command_iter = command.split_whitespace();
        let apex = command_iter.next().expect("");

        match apex {
            "uci" => Some(self.info()),
            "isready" => Some(Self::READY_OK.to_string()),
            "go" => {
                //self.go(command_iter);
                //None
                Some(self.random_move())
            }
            "position" => {
                self.position(command_iter);
                None
            },
            _ => None
        }
    }

    fn info(&self) -> String {
        let mut output: String = "id name ".to_owned();

        output.push_str(Self::NAME);
        output.push(' ');
        output.push_str(Self::VERSION);
        output.push('\n');
        for author in Self::AUTHORS.split(":") {
            output.push_str("id author ");
            output.push_str(author);
            output.push('\n');
        }
        output.push('\n');
        output.push_str(Self::UCI_OK);

        output
    }

    fn go(&self, mut command_iter: SplitWhitespace) {
        let mut wtime: Option<Duration> = None;
        let mut btime: Option<Duration> = None;

        let parse_time = |command: Option<&str>| {
            match command {
                Some(arg) => {
                    match arg.trim().parse() {
                        Ok(ms) => Some(Duration::from_millis(ms)),
                        Err(_) => None
                    }
                },
                None => None
            }

        };

        while let Some(command) = command_iter.next() {
            match command {
                "wtime" => wtime = parse_time(command_iter.next()),
                "btime" => btime = parse_time(command_iter.next()),
                _ => ()
            }
        }
    }

    fn position(&mut self, mut command_iter: SplitWhitespace) {
        // position that isn't working as intended:
        // position startpos moves e2e4 d7d5 e4d5 h7h6 f1b5 c7c6 d5c6 d8d5 b1c3 c8f5 c3d5 g7g5 c6b7
        if let Some(pos_arg) = command_iter.next() {
            self.board = Board::from_fen(pos_arg.trim())
                .unwrap_or(Board::STARTING_POSITION);
        }

        if command_iter.next() == Some("moves") {
            for action_str in command_iter {
                let action = self.board.get_move_from_algebraic(action_str).expect("TODO: better error handling");
                self.board.make_move(&action);
                println!("{}\n{action}\n\nis_in_check = {}", self.board, self.board.clone().in_check());
                print_legal_moves(&self.board);
            }
        }
    }
    
    fn random_move(&mut self) -> String {
        let moves = self.board.legal_moves();
        let action = moves
            .choose(&mut rand::thread_rng())
            .expect("No legal move found.");

        self.board.make_move(action);

        format!("bestmove {}", action)
    }
}
