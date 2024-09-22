use crate::board::Board;

pub struct Bot {
    board: Board,
    debug: bool
}

impl Bot {
    const NAME:    &'static str = env!("CARGO_PKG_NAME");
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

    const UCI_OK:  &'static str = "uciok";
    const READY_OK:&'static str = "readyok";

    pub fn new() -> Self {
        Self {
            board: Board::STARTING_POSITION,
            debug: false
        }
    }

    pub fn exec_command(&self, command: &str) -> Option<String> {
        match command {
            "uci" => Some(self.info()),
            "isready" => Some(Self::READY_OK.to_string()),
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
}
