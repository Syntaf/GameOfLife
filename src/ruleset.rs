pub const DEFAULT: (u8, u8, u8, u8, u8, u16) = (1u8, 2u8, 4u8, 3u8, 50u8, 50u16);

pub struct Ruleset {
    pub starvation: u8,
    pub living: u8,
    pub smothered: u8,
    pub born: u8,
    pub distribution: u8,
    pub speed: u16
}

impl Ruleset {
    pub fn new(rules: (u8, u8, u8, u8, u8, u16)) -> Ruleset {
        Ruleset { starvation: rules.0,
                  living: rules.1,
                  smothered: rules.2,
                  born: rules.3, 
                  distribution: rules.4,
                  speed: rules.5, }
    }

    pub fn update(&mut self, nrules: &[i32]) {
        if nrules[0] != -1 { self.starvation = nrules[0] as u8 }
        else if nrules[1] != -1 { self.living = nrules[1] as u8}
        else if nrules[2] != -1 { self.smothered = nrules[2] as u8 }
        else if nrules[3] != -1 { self.born = nrules[3] as u8 }
        else if nrules[4] != -1 { self.distribution = nrules[4] as u8 }
        else if nrules[5] != -1 { self.speed = nrules[5] as u16 }
    }
}

