use rustty::{
    Terminal, 
    Event
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

pub fn open(term: &mut Terminal) {
    let mut ui = create_ui(40, 15);
    ui.pack(term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));

    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            if let Some(ButtonResult::Ok) = ui.result_for_key(ch) { 
                break 'main
            }
        }

        ui.draw(term);
        term.swap_buffers().unwrap();
    }
}

fn create_ui(width: usize, height: usize) -> Dialog {
    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();

    let mut author = Label::from_str("Author - Grant Mercer");
    author.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,1));
    dlg.add_label(author);
   
    let mut about = Label::new(35, 7);
    about.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    about.set_text("This program uses Rust and my custom fork of rustty, \
                   a terminal UI library, to create a fun and unique \
                   version of Conways game of Life. Feel free to contribute \
                   or send suggestions!");
    about.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2, 3));
    dlg.add_label(about);

    let mut email = Label::from_str("Email - gmercer015@gmail.com");
    email.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2,2));
    dlg.add_label(email);

    let mut github = Label::from_str("Github - github.com/Syntaf");
    github.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, (2,3));
    dlg.add_label(github);

    let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (1,1));
    dlg.add_button(quit);
    dlg
}
