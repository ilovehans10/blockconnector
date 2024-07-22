use std::fmt::Display;

use rand::{distributions::Standard, prelude::Distribution};
use termion::color;

const HEIGHT: u16 = 8;
const WIDTH: u16 = 16;

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
                BlockColor::Red => write!(f, "{}[]", color::Bg(color::Red)),
                BlockColor::Blue => write!(f, "{}[]", color::Bg(color::Blue)),
                BlockColor::Green => write!(f, "{}[]", color::Bg(color::Green)),
                BlockColor::Yellow => write!(f, "{}[]", color::Bg(color::Yellow)),
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
    println!("╔{}╗", "═".repeat((WIDTH * 2).into()));
    for current_row in game_board.chunks(WIDTH.into()) {
        print!("║");
        for current_element in current_row {
            print!("{current_element}");
        }
        println!("{}║", color::Bg(color::Reset));
    }
    println!("╚{}╝", "═".repeat((WIDTH * 2).into()));
}
