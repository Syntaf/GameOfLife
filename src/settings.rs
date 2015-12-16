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

use input;

const CATEGORY1_S: usize = 6;
const CATEGORY2_S: usize = CATEGORY1_S + 7;
const CATEGORY3_S: usize = CATEGORY2_S + 6;

pub fn open(_ruleset: &mut Ruleset, term: &mut Terminal) {
    let (t_width, t_height) = term.size();

    let mut ui_ = create_ui(t_width/2, t_height - t_height/3 + 2, &_ruleset);
    ui_.pack(term, HorizontalAlign::Middle, VerticalAlign::Top, (0,2));

    // Rules with a value of -1 will not be updated when Ruleset::update() is
    // called, thus start all rules empty
    let mut new_rules: Vec<i32> = vec![-1,-1,-1,-1,-1,-1];
    // Errors are displayed through error_lbl with the text as errors
    let mut error_lbl = Label::new(23,1);
    error_lbl.pack(&ui_, HorizontalAlign::Left, VerticalAlign::Bottom, (1,1));
    'main: loop {
        let mut errors = String::new();
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ui_.result_for_key(ch) {
                Some(ButtonResult::Ok)          => break 'main,
                Some(ButtonResult::Custom(i))   => {
                    if let Some(res) = input::integer_prompt(term) {
                        if i < 5 {
                            // options 1-5 are rules for cell death/creation, thus cannot exceed 
                            // 8 (the number of neighbords)
                            if res <= 8 {
                                new_rules[(i-1) as usize] = res as i32;
                            } else {
                                errors = "Invalid value(0-8)".to_string();
                            }
                        } else if i == 5 {
                            // options > 5 are rules for randomization, thus cannot exceed
                            // 100% of the board
                            if res <= 100 {
                                new_rules[(i-1) as usize] = res as i32;
                            } else {
                                errors = "Invalid value(0-100)".to_string();
                            }
                        } else {
                            new_rules[(i-1) as usize] = res as i32;
                        }
                    }
                },
                _  => {}
            }
        }

        if !errors.is_empty() {
            error_lbl.set_text(errors.clone());
            error_lbl.draw(ui_.frame_mut());
        }

         _ruleset.update(&new_rules);
        draw_buttons(&mut ui_, &_ruleset);
        ui_.draw(term);
        term.swap_buffers().unwrap();
    }
}

fn create_ui(width: usize, height: usize,rules: &Ruleset) -> Dialog {
    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();

    let mut title = Label::from_str("Settings");
    title.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,1));
    dlg.add_label(title);

    let mut category1 = Label::new(width-3, 3);
    category1.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    category1.set_text("Generational Rules: ".to_string() 
                       + &(0..width-4).map(|_| "─").collect::<String>());
    category1.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2, 3));
    dlg.add_label(category1);

    let mut category2 = Label::new(width-3, 3);
    category2.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    category2.set_text("Randomize ".to_string()
                       + &(0..width-4).map(|_| "─").collect::<String>());
    category2.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, 
                   (2, CATEGORY1_S+5));
    dlg.add_label(category2);

    let mut category3 = Label::new(width-3, 3);
    category3.align_text(HorizontalAlign::Left, VerticalAlign::Top, (0,0));
    category3.set_text("Game Speed ".to_string()
                       + &(0..width-4).map(|_| "─").collect::<String>());
    category3.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, 
                   (2, CATEGORY2_S+3));
    dlg.add_label(category3);

    draw_buttons(&mut dlg, rules);

    dlg
}

fn draw_buttons(dlg: &mut Dialog, rules: &Ruleset) {
    let mut starvation_b = StdButton::new(&format!("{:<15} [{}]", "Starvation", rules.starvation), 's', ButtonResult::Custom(1));
    starvation_b.pack(dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,CATEGORY1_S));
    dlg.add_button(starvation_b);

    let mut living_b = StdButton::new(
        &format!("{:<15} [{}]", "living", rules.living), 
        'l', ButtonResult::Custom(2));
    living_b.pack(dlg, HorizontalAlign::Left, VerticalAlign::Top, 
                  (2,CATEGORY1_S+1));
    dlg.add_button(living_b);

    let mut smothered_b = StdButton::new(
        &format!("{:<15} [{}]", "Smothered", rules.smothered), 
        'm', ButtonResult::Custom(3));
    smothered_b.pack(dlg, HorizontalAlign::Left, VerticalAlign::Top, 
                     (2,CATEGORY1_S+2));
    dlg.add_button(smothered_b);

    let mut born_b = StdButton::new(
        &format!("{:<15} [{}]", "Born", rules.born), 
        'b', ButtonResult::Custom(4));
    born_b.pack(dlg, HorizontalAlign::Left, VerticalAlign::Top, 
                (2,CATEGORY1_S+3));
    dlg.add_button(born_b);

    let mut dist_b = StdButton::new(
        &format!("Percent Alive [{}%]", rules.distribution), 
        'p', ButtonResult::Custom(5));
    dist_b.pack(dlg, HorizontalAlign::Left, VerticalAlign::Top, 
                (2, CATEGORY2_S+1));
    dlg.add_button(dist_b);

    let mut speed_b = StdButton::new(
        &format!("Delay/Iteration [{}]", rules.speed), 
        'd', ButtonResult::Custom(6));
    speed_b.pack(dlg, HorizontalAlign::Left, VerticalAlign::Top, 
                 (2, CATEGORY3_S));
    dlg.add_button(speed_b);

    let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit.pack(dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (2,1));
    dlg.add_button(quit);
}
