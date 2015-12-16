use std::error::Error;
use std::io::prelude::*;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::BufReader;

use grid::Grid;
use rustty::{
    Terminal, 
    Event,
    HasSize,
};

use rustty::ui::core::{
    Widget,
    HorizontalAlign,
    VerticalAlign,
    ButtonResult
};

use rustty::ui::{
    Dialog,
    StdButton
};

pub fn open(grid: &mut Grid, term: &mut Terminal) {
    let (_, t_height) = term.size();
    let mut presets: Vec<String> = Vec::new();
    let mut ui = create_ui(50, t_height - t_height/3 + 2, &mut presets);
    ui.pack(term, HorizontalAlign::Middle, VerticalAlign::Middle, (0,0));

    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            match ui.result_for_key(ch) {
                Some(ButtonResult::Ok) => break 'main,
                Some(ButtonResult::Custom(i)) => {
                    load_preset(&presets[i as usize -1], grid);
                    break 'main;
                },
                _                      => {}
            }
        }

        ui.draw(term);
        term.swap_buffers().unwrap();
    }
}

fn load_preset(p: &str, grid: &mut Grid) {
    let file = match File::open(Path::new(&p)) {
        Err(why) => panic!("Error loading preset {}: {}", &p,
                           Error::description(&why)),
        Ok(f) => BufReader::new(f),
    };

    for line in file.lines() {
        let l = line.unwrap();
        let cords = l.split(",").map(
            |x| usize::from_str_radix(x, 10).unwrap())
            .collect::<Vec<usize>>();
        grid.canvas_mut().get_mut(cords[0], cords[1]).unwrap().
            set_bg(Grid::rand_color());
    }

}

fn create_ui(width: usize, height: usize, presets: &mut Vec<String>) 
    -> Dialog {

    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();

    let paths = fs::read_dir(Path::new("./presets")).unwrap();

    let mut i: u32 = 1;
    for path in paths {
        if i as usize >= height-1 || i > 9 {
            panic!(format!("Sorry! Only maximum 10 presets allowed. Either \
                            increase height of terminal or delete some presets
                            within the 'preset' folder"));
        }
        let path = path.unwrap().path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            let mut btn = StdButton::new(&format!("{}: {}", i, name), 
                                         (i + 48) as u8 as char, 
                                         ButtonResult::Custom(i as i32));
            btn.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top,
                     (2,i as usize));
            dlg.add_button(btn);
            presets.push(path.to_str().unwrap().to_string());
            i += 1;
        }
    }
    let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (1,1));
    dlg.add_button(quit);
    dlg
}
