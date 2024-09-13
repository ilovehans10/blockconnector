use game_loop::game_loop;
use player_input::Inputter;
mod game_board;
mod game_loop;
mod player_input;
mod tiles;

#[cfg(test)]
mod test;

const HEIGHT: u16 = 8;
const WIDTH: u16 = 16;

fn main() {
    game_loop(HEIGHT, WIDTH);
    let input_interface = Inputter::new();
    input_interface.print_history();
}
