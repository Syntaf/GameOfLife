extern crate rustty; 

use ruleset::Ruleset;

use self::rustty::{
    Terminal,
    Event,
    HasSize
};

use self::rustty::ui::core::{
    Widget,
    HorizontalAlign,
    VerticalAlign,
    ButtonResult,
};

use self::rustty::ui::{
    Dialog,
    Canvas,
    Label,
    StdButton
};

pub struct Game {
    term: Terminal,
    ruleset: Ruleset,
    ui: Dialog,
    canvas: Canvas
}

impl Game {
    pub fn new(t_term: Terminal, t_ruleset: Ruleset) -> Game {
        let (t_width, t_height) = t_term.size();

        // Do not start game is the terminal is not large enough for UI
        if t_width < 50 || t_height < 30 {
            let msg = format!("Terminal must be larger than 50 columns and \
            30 rows. {}x{} detected", t_width, t_height);
            panic!(msg);
        }

        let mut ui_ = Game::create_ui(t_width, t_height/5);
        ui_.pack(&t_term, HorizontalAlign::Middle, VerticalAlign::Bottom, (0,0));

        let mut canvas_ = Canvas::new(t_width, t_height - t_height/5);
        canvas_.draw_box();
        canvas_.pack(&t_term, HorizontalAlign::Middle, VerticalAlign::Top, (0,0));

        Game { 
            term: t_term, 
            ruleset: t_ruleset, 
            ui: ui_, 
            canvas: canvas_ 
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

            }

            self.ui.draw(&mut self.term);
            self.canvas.draw(&mut self.term);
            self.term.swap_buffers().unwrap();
        }
    }
}

