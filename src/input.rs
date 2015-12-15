use std::u32;

use rustty::{
    Terminal,
    Event
};

use rustty::ui::core::{
    Widget,
    HorizontalAlign,
    VerticalAlign,
};

use rustty::ui::{
    Dialog,
    Label
};

pub fn integer_prompt(term: &mut Terminal) -> Option<u32> {
    let (width, height) = (30, 7);

    let mut prompt = create_prompt(width, height);
    prompt.pack(term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));

    let mut value = "_".to_string();
    let mut input = Label::new(15,1);
    input.pack(&prompt, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ch {
                'q'     => break 'main,
                '\r'    => { 
                    if !value.is_empty() {
                        value.remove(0);
                        return u32::from_str_radix(&value, 10).ok();
                    }
                },
                _       => {
                    if ch.is_digit(10) {
                        value.push(ch);
                    }
                }
            }
        }

        input.set_text(value.clone() + "_");
        input.draw(prompt.frame_mut());
        prompt.draw(term);
        term.swap_buffers().unwrap();
    }
    None
}

fn create_prompt(width: usize, height: usize) -> Dialog {
    let mut prompt = Dialog::new(width, height);
    prompt.draw_box();

    let mut title = Label::from_str("Enter to finish");
    title.pack(&prompt, HorizontalAlign::Left, VerticalAlign::Top, (2,1));
    prompt.add_label(title);

    let mut quit = Label::from_str("q to cancel");
    quit.pack(&prompt, HorizontalAlign::Right, VerticalAlign::Bottom, (2,1));
    prompt.add_label(quit);
    prompt
}
