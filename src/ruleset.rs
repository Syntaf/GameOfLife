pub const DEFAULT: (u8, u8, u8, u8) = (1u8, 2u8, 4u8, 3u8);

pub struct Ruleset {
    pub starvation: u8,
    pub living: u8,
    pub smothered: u8,
    pub born: u8
}

impl Ruleset {
    pub fn new(rules: (u8, u8, u8, u8)) -> Ruleset {
        Ruleset { starvation: rules.0,
                  living: rules.1,
                  smothered: rules.2,
                  born: rules.3, }
    }
}

