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

pub struct Game<'a> {
    term: Terminal,
    ruleset: &'a Ruleset,
    ui: Dialog,
    canvas: Canvas
}

impl<'a> Game<'a> {
    pub fn new(t_term: Terminal, t_ruleset: &'a Ruleset) -> Game {
        let (t_width, t_height) = t_term.size();

        let mut dlg = Dialog::new(t_width, t_height/5);
        dlg.draw_box();

        let mut title = Label::from_str("Welcome to the console based game of life!");
        title.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (1,1));
        dlg.add_label(title);
        dlg.pack(&t_term, HorizontalAlign::Middle, VerticalAlign::Bottom, (0,0));

        let mut canvas_ = Canvas::new(t_width - 1, t_height - t_height/5 - 1);
        canvas_.draw_box();
        canvas_.pack(&t_term, HorizontalAlign::Middle, VerticalAlign::Top, (0,1));

        Game { term: t_term, ruleset: t_ruleset, ui: dlg, canvas: canvas_ }
    }

    pub fn run(&mut self) {
        'main: loop {
            while let Some(Event::Key(ch)) = self.term.get_event(0).unwrap() {
                match ch {
                    'q' => break 'main,
                     _  => {},
                }
            }

            self.ui.draw(&mut self.term);
            self.canvas.draw(&mut self.term);
            self.term.swap_buffers().unwrap();
        }
    }
}

