use std::fmt::Display;

const HEIGHT: u8 = 8;
const WIDTH: u8 = 8;

#[derive(Clone, Copy)]
enum TileTypes {
    Block(Color),
    Firework(Direction),
    DiscoBall,
}

#[derive(Clone, Copy)]
enum Color {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Clone, Copy)]
struct Direction {}

impl TileTypes {
    const fn new() -> Self {
        Self::Block(Color::Blue)
    }
}

impl Display for TileTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DiscoBall => write!(f, "*"),
            Self::Firework(_direction) => todo!(),
            Self::Block(color) => match color {
                Color::Red => write!(f, "R"),
                Color::Blue => write!(f, "B"),
                Color::Green => write!(f, "G"),
                Color::Yellow => write!(f, "Y"),
            }
        }
    }
}

fn main() {
    let game_board = vec![TileTypes::new(); (HEIGHT * WIDTH).into()];
    println!("╔{}╗", "═".repeat(WIDTH.into()));
    for current_row in game_board.chunks(WIDTH.into()) {
        print!("║");
        for current_element in current_row {
            print!("{current_element}");
        }
        println!("║");
    }
    println!("╚{}╝", "═".repeat(WIDTH.into()));
}
