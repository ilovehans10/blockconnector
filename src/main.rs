use game_loop::game_loop;
mod game_board;
mod game_loop;
mod tiles;

#[cfg(test)]
mod test;

const HEIGHT: u16 = 8;
const WIDTH: u16 = 16;

fn main() {
    game_loop(HEIGHT, WIDTH);
}
