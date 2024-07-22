use std::fmt::Display;

use rand::{distributions::Standard, prelude::Distribution};

const HEIGHT: u8 = 8;
const WIDTH: u8 = 8;

#[derive(Clone, Copy)]
enum TileTypes {
    Block(Color),
    _Firework(Direction),
    _DiscoBall,
}

#[derive(Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
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
            Self::_DiscoBall => write!(f, "*"),
            Self::_Firework(_direction) => todo!(),
            Self::Block(color) => match color {
                Color::Red => write!(f, "R"),
                Color::Blue => write!(f, "B"),
                Color::Green => write!(f, "G"),
                Color::Yellow => write!(f, "Y"),
            },
        }
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..=3) {
            0 => Color::Red,
            1 => Color::Green,
            2 => Color::Blue,
            _ => Color::Yellow,
        }
    }
}

fn main() {
    let mut game_board = vec![TileTypes::new(); (HEIGHT * WIDTH).into()];
    game_board = game_board.into_iter().map(|_| TileTypes::Block(rand::random())).collect();
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
