use std::io::{self, Write};

pub struct Inputter {
    board_prototype: Option<BoardPrototype>,
    history: Vec<InputData>,
}

struct BoardPrototype {
    height: u16,
    width: u16,
}

struct InputData {
    user_input: String,
    program_output: String,
    status: Status,
}

enum Status {
    Info,
    Error,
}

impl Inputter {
    pub const fn new() -> Self {
        Self {
            board_prototype: None,
            history: Vec::new(),
        }
    }

    pub fn get_input(&mut self) {
        print!(">");
        io::stdout()
            .flush()
            .expect("Should be able to flush stdout");
        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin
            .read_line(&mut user_input)
            .expect("Should be able to read from stdin");
        user_input = user_input.trim().into();
        self.add_history(user_input, String::from("Output"));
    }

    fn add_history(&mut self, user_input: String, program_output: String) {
        self.history.push(InputData {
            user_input,
            program_output,
            status: Status::Info,
        });
    }

    pub fn print_history(&self) {
        for item in &self.history {
            println!(">{}\n<{}", item.user_input, item.program_output);
        }
    }
}
