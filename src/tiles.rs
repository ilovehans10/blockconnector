use rand::{distributions::Standard, prelude::Distribution};
use std::fmt::{Debug, Display};
use termion::color;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TileTypes {
    Gap,
    Block(BlockColor),
    _Firework(Direction),
    _DiscoBall,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockColor {
    Red,
    Blue,
    Green,
    Yellow,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    _Vertical,
    _Horizontal,
}

impl TileTypes {
    pub const fn new() -> Self {
        Self::Block(BlockColor::Blue)
    }
}

impl Display for TileTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gap => write!(f, "{}  ", color::Bg(color::Reset)),
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

impl Debug for TileTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Gap => write!(f, "{}  ", color::Bg(color::Reset)),
            Self::_DiscoBall => write!(f, "*"),
            Self::_Firework(_direction) => todo!(),
            Self::Block(color) => match color {
                BlockColor::Red => {
                    write!(f, "{}[]{}", color::Bg(color::Red), color::Bg(color::Reset))
                }
                BlockColor::Blue => {
                    write!(f, "{}[]{}", color::Bg(color::Blue), color::Bg(color::Reset))
                }
                BlockColor::Green => write!(
                    f,
                    "{}[]{}",
                    color::Bg(color::Green),
                    color::Bg(color::Reset)
                ),
                BlockColor::Yellow => write!(
                    f,
                    "{}[]{}",
                    color::Bg(color::Yellow),
                    color::Bg(color::Reset)
                ),
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
