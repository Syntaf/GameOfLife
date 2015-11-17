/* Conways ascii game of life
 * Any cell that is alive or zero or just one livin neighbor is dead in the next generation
 * Any cell that is alive and has two or three living neighbors lives happily onto next generation
 * Any cell that is alive and has four of more neighbors gets smothered and dies
 * Any cell that is dead and has exactly three neighbors is born, and alive
*/
extern crate rustty;

pub mod ruleset;
pub mod game;

use ruleset::Ruleset;
use game::Game;

use std::io::{self, BufRead};

fn main() {
    let mut term = rustty::Terminal::new().unwrap();
    let ruleset = Ruleset::new(ruleset::DEFAULT);
    let mut game = Game::new(term, &ruleset);
    
    game.run();
}



