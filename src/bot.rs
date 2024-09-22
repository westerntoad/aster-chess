use std::str::SplitWhitespace;
use std::time::Duration;
use rand::seq::SliceRandom;
use crate::board::Board;
use crate::types::movement::Move;

pub struct Bot {
    board: Board
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
        output.push_str(Self::UCI_OK);

        output
    }

    fn go(&self, mut commands_iter: SplitWhitespace) {
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

        while let Some(command) = commands_iter.next() {
            match command {
                "wtime" => wtime = parse_time(commands_iter.next()),
                "btime" => btime = parse_time(commands_iter.next()),
                _ => ()
            }
        }
    }
    
    fn random_move(&mut self) -> String {
        let moves = self.board.legal_moves();
        let action = moves
            .choose(&mut rand::thread_rng())
            .expect("No legal move found.");

        format!("bestmove {}", action)
    }
}
