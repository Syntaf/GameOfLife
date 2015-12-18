use ruleset::Ruleset;
use grid::Grid;

use rand::distributions::{IndependentSample, Range};
use rand;

use rustty::{
    Terminal,
    Event,
    HasSize,
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

use settings;
use help;
use about;
use preset;
use editor;

// Hold all UI elements and perform game logic within a 
// main loop
pub struct Game {
    term: Terminal,
    ruleset: Ruleset,
    ui: Dialog,
    stat_ui: Dialog,
    grid: Grid
}

impl Game {
    pub fn new(t_term: Terminal, t_ruleset: Ruleset) -> Game {
        let (t_width, t_height) = t_term.size();

        if t_width < 80 || t_height < 40 {
            panic!(format!("Terminal size too small({}x{}), must be at least \
                            {}x{} (columns x rows)",
                t_width, t_height, 80, 34));
        }

        let mut ui_ = Game::create_ui(2 * t_width/3, t_height/5);
        ui_.pack(&t_term, HorizontalAlign::Left, VerticalAlign::Bottom, (0,0));

        let mut stat_ui_ = Game::create_stats(t_width/3, t_height/5);
        stat_ui_.pack(&t_term, HorizontalAlign::Right, VerticalAlign::Bottom, (0,0));

        let mut grid_ = Grid::new(t_width, t_height - t_height/5);
        grid_.draw_box();
        grid_.pack(&t_term, HorizontalAlign::Middle, VerticalAlign::Top, (0,0));

        Game { 
            term: t_term, 
            ruleset: t_ruleset, 
            ui: ui_, 
            stat_ui: stat_ui_,
            grid: grid_ 
        }
    }


    pub fn run(&mut self) {
        let mut play = false;
        // We don't want to wait 750 ms before the program starts, so 0 timeout
        let mut timeout = 0;
        // Generation count
        let mut gen: u64 = 0;

        let mut gen_lbl = Label::new(self.stat_ui.frame().size().0-4, 1);
        gen_lbl.pack(&self.stat_ui, HorizontalAlign::Left, VerticalAlign::Top, (2,3));
        gen_lbl.set_text("Generation:");
        
        let mut aliv_lbl = Label::new(self.stat_ui.frame().size().0-4, 1);
        aliv_lbl.pack(&self.stat_ui, HorizontalAlign::Left, VerticalAlign::Top, (2,4));
        aliv_lbl.set_text("Living Cells:");
        'main: loop {
            while let Some(Event::Key(ch)) = self.term.get_event(timeout).unwrap() {
                match self.ui.result_for_key(ch) {
                    Some(ButtonResult::Ok) => break 'main,
                    Some(ButtonResult::Custom(i)) => {
                        match i {
                            1   => { play = true; },
                            2   => { /*  */ },
                            3   => { settings::open(&mut self.ruleset, 
                                                    &mut self.term);
                                     gen = 0;                           },
                            4   => { help::open(&self.ruleset, 
                                                &mut self.term);        },
                            5   => { about::open(&mut self.term);       },
                            6   => { editor::open(&mut self.grid,
                                                  &mut self.term);
                                     gen = 0;                           },
                            7   => { self.randomize_grid();             },
                            8   => { preset::open(&mut self.grid,
                                                  &mut self.term);
                                     gen = 0;                           },
                            _   => {}
                        }
                        if i != 1 { play = false; }
                    }
                     _  => {},
                }
            }
            timeout = self.ruleset.speed as isize;

            // if the game is to be played
            if play {
                let mut alive = 0;
                let (cols, rows) = self.grid.playable_size();
                let ruleset = &self.ruleset;

                // Iterate over the playable region
                for y in 1..rows {
                    for x in 1..cols {
                        let ncnt = self.grid.neighbors(x, y);
                        // conditions for only if the cell is alive
                        if self.grid.is_alive(x, y) {
                            alive += 1;
                            if ncnt <= ruleset.starvation {
                                self.grid.set_dead(x, y);
                            } else if ncnt == ruleset.living {
                                /* nothing */
                            } else if ncnt >= ruleset.smothered {
                                self.grid.set_dead(x, y);
                            }
                        } else  if ncnt >= ruleset.born && ncnt < ruleset.smothered {
                            self.grid.set_alive(x, y);
                        }
                    }
                }
                // The cell actions above are not recorded until an update is called
                self.grid.update();
                gen_lbl.set_text(format!("Generation: {:>7}", gen));
                aliv_lbl.set_text(format!("Living Cells: {:>5}", alive));
                gen += 1;
            }

            // Display the grid and ui to the screen
            gen_lbl.draw(self.stat_ui.frame_mut());
            aliv_lbl.draw(self.stat_ui.frame_mut());
            self.ui.draw(&mut self.term);
            self.stat_ui.draw(&mut self.term);
            self.grid.draw(&mut self.term);
            self.term.swap_buffers().unwrap();
        }
    }

    pub fn randomize_grid(&mut self) {
        let (cols, rows) = self.grid.playable_size();

        // Start by clearing screen
        for y in 1..rows {
            for x in 1..cols {
                    self.grid.set_dead(x, y);
            }
        }

        // Update grid with blank sheet
        self.grid.update();

        // Designate amount of cells that are alive
        let total_cells = cols * rows;
        let alive_cells = total_cells as f32  * 
            (self.ruleset.distribution as f32 / 100.0);

        let col_rnder = Range::new(2,cols);
        let row_rnder = Range::new(2,rows);
        let mut rng = rand::thread_rng();

        // Worse case O(n^2) , if distribution is 100%. Possibly
        // find a better algorithm?
        for _i in 0..(alive_cells as usize) {
            self.grid.set_alive(
                col_rnder.ind_sample(&mut rng),
                row_rnder.ind_sample(&mut rng))
        }

        self.grid.update();
    
    }

    fn create_stats(width: usize, height: usize) -> Dialog {
        let mut dlg = Dialog::new(width, height);
        dlg.draw_box();

        let mut title = Label::new(width-width/8, 3); 
        title.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
        title.set_text("Stats");
        title.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2, 1));
        dlg.add_label(title);

        dlg
    }

    fn create_ui(width: usize, height: usize) -> Dialog {
        let mut dlg = Dialog::new(width, height);
        dlg.draw_box();

        const COLUMN_SEP : usize = 16;

        let mut title = Label::from_str("Welcome to the console based game of life!");
        title.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,1));
        dlg.add_label(title);
        
        let mut play = StdButton::new("Play", 'p', ButtonResult::Custom(1));
        play.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2,5));
        dlg.add_button(play);

        let mut stop = StdButton::new("Stop", 'o', ButtonResult::Custom(2));
        stop.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2,4));
        dlg.add_button(stop);
        
        let mut settings = StdButton::new("Settings", 's', ButtonResult::Custom(3));
        settings.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2,3));
        dlg.add_button(settings);

        let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
        quit.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2+COLUMN_SEP,5));
        dlg.add_button(quit);

        let mut help = StdButton::new("Help", 'h', ButtonResult::Custom(4));
        help.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2+COLUMN_SEP, 4));
        dlg.add_button(help);

        let mut about = StdButton::new("About", 'a', ButtonResult::Custom(5));
        about.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2+COLUMN_SEP, 3));
        dlg.add_button(about);

        let mut editor = StdButton::new("Editor", 'e', ButtonResult::Custom(6));
        editor.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2+2*COLUMN_SEP, 5));
        dlg.add_button(editor);

        let mut rnd = StdButton::new("Randomize", 'r', ButtonResult::Custom(7));
        rnd.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2+2*COLUMN_SEP, 4));
        dlg.add_button(rnd);

        let mut preset = StdButton::new("Preset", 't', ButtonResult::Custom(8));
        preset.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2+2*COLUMN_SEP, 3));
        dlg.add_button(preset);

        dlg
    }
}

