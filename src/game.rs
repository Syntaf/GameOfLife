extern crate rustty; 

use ruleset::Ruleset;

use self::rustty::{
    Terminal,
    Event,
    HasSize,
    CellAccessor
};

pub struct Game<'a> {
    term: Terminal,
    ruleset: &'a Ruleset
    
}

impl<'a> Game<'a> {
    pub fn new(t_term: Terminal, t_ruleset: &'a Ruleset) -> Game {
        Game { term: t_term, ruleset: t_ruleset }
    }

    pub fn run(&mut self) {
        'main: loop {
            while let Some(Event::Key(ch)) = self.term.get_event(0).unwrap() {
                match ch {
                    'q' => break 'main,
                     _  => {},
                }
            }
        }
    }
}

