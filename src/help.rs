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

const GAME_DESC: &'static str = "The Game of Life is a cellular automaton \
    devised by John Horton Conway in 1970. The rules of the game are very \
    simple: based on a starting configuration, each iteration creates a \
    new generation of cells based off of 4 rules.";

const SETTINGS_DESC: &'static str = "All 4 rules of 'Life' can be modified \
    through the settings window by pressing 's' at the home screen. \
    Generational rules accept 0-8 (number of possible neighbors), Randomize \
    accepts a percentage value 0-100 (percentage cells that will start alive) \
    and speed accepts up to a u16 (ms of delay per generation).";

const CREATION_DESC: &'static str = "There are a couple ways to create a \
    starting configuration. The first is to randomize the selection by \
    pressing 'r', this will distribute a percentage of cells as alive \
    randomly across the board. Next you can hop into the editor mode with \
    'e' and create a custom configuration using your paintbrush. Lastly you \
    can load custom presets provided by pressing 't'";

pub fn open(_ruleset: &Ruleset, term: &mut Terminal) {
    let (t_width, t_height) = term.size();

    let mut page1 = create_page0(t_width - t_width/3, 
                                 t_height - t_height/3 + 2);
    page1.pack(term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));

    let mut page2 = create_page1(t_width - t_width/3, 
                                 t_height - t_height/3 + 2);
    page2.pack(term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));
   
    let mut pages = vec![page1, page2];
    let mut current_page = 0;
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match pages[current_page].result_for_key(ch) {
                Some(ButtonResult::Ok) => break 'main,
                Some(ButtonResult::Custom(i)) => {
                    current_page = i as usize;
                }
                _                      => {}
            }
        }

        pages[current_page].draw(term);
        term.swap_buffers().unwrap();
    }
}

fn create_page0(width: usize, height:usize) -> Dialog {
    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();

    let mut desc = Label::new(width-4, height/4);
    desc.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    desc.set_text(GAME_DESC);
    desc.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,1));
    dlg.add_label(desc);

    let h = height/4;
    let mut rule1 = Label::new(width-6, 3);
    rule1.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    
    rule1.set_text("1. Any live cell with fewer than _ \
                   neighbors dies, as if caused by under population");
    rule1.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (4,h+1));
    dlg.add_label(rule1);

    let mut rule2 = Label::new(width-6, 3);
    rule2.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    rule2.set_text("2. Any live cell with _ to _ alive neighbors \
                   lives on to the next generation");
    rule2.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (4,h+5));
    dlg.add_label(rule2);

    let mut rule3 = Label::new(width-6, 3);
    rule3.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    rule3.set_text("3. Any live cell with more than _ alive neighbors \
                   dies, as if by over population");
    rule3.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (4,h+9));
    dlg.add_label(rule3);

    let mut rule4 = Label::new(width-6, 3);
    rule4.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    rule4.set_text("4. Any dead cell with exactly _ neighbors becomes a \
                   live cell, as if by reproduction");
    rule4.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (4, h+13));
    dlg.add_label(rule4);

    let mut next_page = StdButton::new("Next Page - Usage", 'n',
                                  ButtonResult::Custom(1));
    next_page.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom, 
                   (4, 3));
    dlg.add_button(next_page);

    let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (1,1));
    dlg.add_button(quit);

    dlg
}

fn create_page1(width: usize, height:usize) -> Dialog {
    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();
    
    let mut title = Label::from_str("Modifying Settings");
    title.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,1));
    dlg.add_label(title);

    let mut desc = Label::new(width-6, height/4);
    desc.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    desc.set_text(SETTINGS_DESC);
    desc.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (4,3));
    dlg.add_label(desc);

    let mut desc2 = Label::new(width-6, height/4);
    desc2.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    desc2.set_text(CREATION_DESC);
    desc2.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, 
               (4,5+height/4));
    dlg.add_label(desc2);

    let mut prev_page = StdButton::new("Previous Page - Game of Life", 'p',
                                       ButtonResult::Custom(0));
    prev_page.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Bottom,
                   (4,3));
    dlg.add_button(prev_page);

    let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (1,1));
    dlg.add_button(quit);

    dlg
}
