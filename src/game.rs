use ruleset::Ruleset;
use board::Board;

pub struct Game<'a> {
    board: Board,
    ruleset: &'a Ruleset
}

impl<'a> Game<'a> {
    pub fn new(t_board: Board, t_ruleset: &'a Ruleset) -> Game {
        Game { board: t_board, ruleset: t_ruleset }
    }
}
