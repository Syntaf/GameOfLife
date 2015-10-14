/* Conways ascii game of life
 * Any cell that is alive or zero or just one livin neighbor is dead in the next generation
 * Any cell that is alive and has two or three living neighbors lives happily onto next generation
 * Any cell that is alive and has four of more neighbors gets smothered and dies
 * Any cell that is dead and has exactly three neighbors is born, and alive
*/

pub mod board;
pub mod ruleset;
pub mod game;

use board::Board;
use ruleset::Ruleset;
use game::Game;

use std::io::{self, BufRead};

fn main() {
    // Create reader, prompt user and grab a line
    let mut reader = io::stdin();
    let mut input_text = String::new();
    match reader.read_line(&mut input_text) {
        Err(e)  => { panic!("Error {}", e); },
        Ok(_)   => { }
    }

    // Split line into white space separated sections
    let mut input_iter = input_text.split_whitespace().map(
        |x| x.trim().parse::<usize>().ok().expect("Error, invalid integer detected"));
    let (a, b) = (input_iter.next().unwrap(), input_iter.next().unwrap());

    let mut board = Board::new(a, b);
    let ruleset = Ruleset::new(ruleset::DEFAULT);
    let mut game = Game::new(board, &ruleset);

}



