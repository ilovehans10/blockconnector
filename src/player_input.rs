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

struct Status;

impl Inputter {
    pub const fn new() -> Self {
        Self {
            board_prototype: None,
            history: Vec::new(),
        }
    }

    fn add_history(&mut self, user_input: String, program_output: String) {
        self.history.push(Input_Data {
            user_input,
            program_output,
            status: Status,
        });
    }

    pub fn print_history(&self) {
        for item in &self.history {
            println!(">{}\n{}", item.user_input, item.program_output);
        }
    }
}
