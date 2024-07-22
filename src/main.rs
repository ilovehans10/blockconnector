use std::fmt::Display;

use rand::{distributions::Standard, prelude::Distribution};

const HEIGHT: u8 = 8;
const WIDTH: u8 = 8;

#[derive(Clone, Copy)]
enum TileTypes {
    Block(BlockColor),
    _Firework(Direction),
    _DiscoBall,
}

#[derive(Clone, Copy)]
enum BlockColor {
    Red,
    Green,
    Blue,
    Yellow,
}

#[derive(Clone, Copy)]
enum Direction {
    _Vertical,
    _Horizontal,
}

impl TileTypes {
    const fn new() -> Self {
        Self::Block(BlockColor::Blue)
    }
}

impl Display for TileTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::_DiscoBall => write!(f, "*"),
            Self::_Firework(_direction) => todo!(),
            Self::Block(color) => match color {
                BlockColor::Red => write!(f, "R"),
                BlockColor::Blue => write!(f, "B"),
                BlockColor::Green => write!(f, "G"),
                BlockColor::Yellow => write!(f, "Y"),
            },
        }
    }
}

impl Distribution<BlockColor> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BlockColor {
        match rng.gen_range(0..=3) {
            0 => BlockColor::Red,
            1 => BlockColor::Green,
            2 => BlockColor::Blue,
            _ => BlockColor::Yellow,
        }
    }
}

fn main() {
    let mut game_board = vec![TileTypes::new(); (HEIGHT * WIDTH).into()];
    game_board.fill_with(|| TileTypes::Block(rand::random::<BlockColor>()));
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
