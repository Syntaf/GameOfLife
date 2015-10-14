pub const DEFAULT: (u8, u8, u8, u8) = (1u8, 2u8, 4u8, 3u8);

pub struct Ruleset {
    starvation: u8,
    living: u8,
    smothered: u8,
    born: u8
}

impl Ruleset {
    pub fn new(rules: (u8, u8, u8, u8)) -> Ruleset {
        Ruleset { starvation: rules.0,
                  living: rules.1,
                  smothered: rules.2,
                  born: rules.3 }
    }
}

