use std::time::Duration;
use std::thread;
use ruleset::Ruleset;
use grid::Grid;

use rustty::{
    Terminal,
    Event,
    HasSize
};

use rustty::ui::core::{
    Widget,
    HorizontalAlign,
    VerticalAlign,
    ButtonResult,
};

use rustty::ui::{
    Dialog,
    Label,
    StdButton
};

pub struct Game {
    term: Terminal,
    ruleset: Ruleset,
    ui: Dialog,
    grid: Grid
}

impl Game {
    pub fn new(t_term: Terminal, t_ruleset: Ruleset) -> Game {
        let (t_width, t_height) = t_term.size();

        let mut ui_ = Game::create_ui(t_width, t_height/5);
        ui_.pack(&t_term, HorizontalAlign::Middle, VerticalAlign::Bottom, (0,0));

        let mut grid_ = Grid::new(t_width, t_height - t_height/5);
        grid_.draw_box();
        grid_.pack(&t_term, HorizontalAlign::Middle, VerticalAlign::Top, (0,0));

        Game { 
            term: t_term, 
            ruleset: t_ruleset, 
            ui: ui_, 
            grid: grid_ 
        }
    }

    fn create_ui(width: usize, height: usize) -> Dialog {
        let mut dlg = Dialog::new(width, height);
        dlg.draw_box();

        let mut title = Label::from_str("Welcome to the console based game of life!");
        title.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (1,1));
        dlg.add_label(title);
        
        let mut play = StdButton::new("Play", 'p', ButtonResult::Custom(1));
        play.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2,5));
        dlg.add_button(play);

        let mut stop = StdButton::new("Stop", 's', ButtonResult::Custom(2));
        stop.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2,4));
        dlg.add_button(stop);
        
        let mut settings = StdButton::new("Settings", 's', ButtonResult::Custom(3));
        settings.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2,3));
        dlg.add_button(settings);

        let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
        quit.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2,2));
        dlg.add_button(quit);

        let mut help = StdButton::new("Help", 'h', ButtonResult::Custom(4));
        help.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (2, 5));
        dlg.add_button(help);

        let mut about = StdButton::new("About", 'a', ButtonResult::Custom(5));
        about.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (2, 4));
        dlg.add_button(about);

        dlg
    }

    pub fn run(&mut self) {
        let mut play = false;
        'main: loop {
            while let Some(Event::Key(ch)) = self.term.get_event(0).unwrap() {
                match self.ui.result_for_key(ch) {
                    Some(ButtonResult::Ok) => break 'main,
                    Some(ButtonResult::Custom(i)) => {
                        match i {
                            1   => {play = true},
                            2   => {play = false},
                            3   => { /* settings */ },
                            4   => { /* help */ },
                            5   => { /* about */ },
                            _   => {}
                        }
                    }
                     _  => {},
                }
            }

            // if the game is to be played
            if play {
                thread::sleep(Duration::from_millis(1000));
                let (rows, cols) = self.grid.playable_size();
                let ref ruleset = self.ruleset;
                for i in 0..rows {
                    for j in 0..cols {
                        let ncnt = self.grid.neighbors(i, j);
                        //panic!("kk");
                        if ncnt == ruleset.starvation {
                            //panic!("4");
                            self.grid.set_dead(i, j);
                        } else if ncnt == ruleset.living {
                            //panic!("3");
                            /* nothing */
                        } else if ncnt == ruleset.smothered {
                            //panic!("2");
                            self.grid.set_dead(i, j);
                        } else if ncnt == ruleset.born {
                            let s = format!("neighbors: {}...{}, {}", ncnt, i, j);
                            //panic!(s);
                            self.grid.set_alive(i, j);
                        }
                    }
                }
            }

            self.ui.draw(&mut self.term);
            self.grid.draw(&mut self.term);
            self.term.swap_buffers().unwrap();
        }
    }
}

