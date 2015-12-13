use ruleset::Ruleset;

use rustty::{
    Terminal, 
    Event,
    HasSize
};

use rustty::ui::core::{
    Widget,
    HorizontalAlign,
    VerticalAlign,
    ButtonResult
};

use rustty::ui::{
    Dialog,
    Label,
    StdButton
};

pub fn open(_ruleset: &Ruleset, term: &mut Terminal) {
    let (t_width, t_height) = term.size();

    let mut ui_ = create_ui(t_width/2, t_height/2);
    ui_.pack(term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ui_.result_for_key(ch) {
                Some(ButtonResult::Ok) => break 'main,
                _                      => {}
            }
        }

        ui_.draw(term);
        term.swap_buffers().unwrap();
    }
}

fn create_ui(width: usize, height:usize) -> Dialog {
    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();

    let mut title = Label::from_str("How-to");
    title.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (1,1));
    dlg.add_label(title);

    let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (1,1));
    dlg.add_button(quit);

    dlg
}
