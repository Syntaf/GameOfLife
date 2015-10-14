extern crate rustty; 

use ruleset::Ruleset;

use self::rustty::{
    Terminal,
    Event,
    HasSize,
    CellAccessor
};

use self::rustty::ui::{
    Painter,
    Dialog,
    DialogResult,
    Alignable,
    HorizontalAlign,
    VerticalAlign,
    Widget
};

pub struct Game<'a> {
    term: Terminal,
    ruleset: &'a Ruleset,
    ui: Dialog,
    canvas: Widget
}

impl<'a> Game<'a> {
    pub fn new(t_term: Terminal, t_ruleset: &'a Ruleset) -> Game {
        let mut ui_dlg = Dialog::new(t_term.cols(), 10);
        let mut canvas = Widget::new(t_term.cols(), t_term.rows() - 10);
        ui_dlg.window_mut().align(&t_term, HorizontalAlign::Left, VerticalAlign::Bottom, 0);
        ui_dlg.window_mut().draw_box();
        canvas.align(&t_term, HorizontalAlign::Left, VerticalAlign::Top, 0);
        canvas.draw_box();
        Game { term: t_term, ruleset: t_ruleset, ui: ui_dlg, canvas: canvas }
    }

    pub fn run(&mut self) {
        'main: loop {
            while let Some(Event::Key(ch)) = self.term.get_event(0).unwrap() {
                match ch {
                    'q' => break 'main,
                     _  => {},
                }
            }

            self.ui.window().draw_into(&mut self.term);
            self.canvas.draw_into(&mut self.term);
            self.term.swap_buffers().unwrap();
        }
    }
}

