mod cli;
mod command;
mod direction;
mod game;
mod point;
mod snake;

use game::Game;

fn main() {
	Game::from_args(cli::get_args()).run();
}
