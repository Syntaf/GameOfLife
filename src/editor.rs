use grid::Grid;

use rustty::{
    Terminal, 
    Event,
    HasSize,
    CellAccessor,
    Color,
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

struct Cursor {
    pos: Position,
    color: Color,
}

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

pub fn open(grid: &mut Grid, term: &mut Terminal) {
    let mut cursor = Cursor {
        pos: Position { x: 1, y: 1 },
        color: Color::Default,
    };
    let (t_width, t_height) = term.size();

    let mut ui = create_ui(2 * t_width/3, t_height/5);
    ui.pack(term, HorizontalAlign::Left, VerticalAlign::Bottom, (0,0));

    let (cols, rows) = grid.playable_size();
    term.swap_buffers().unwrap();
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            let canvas = grid.canvas_mut();
            match ch {
                'q' => { 
                    canvas.get_mut(cursor.pos.x, cursor.pos.y).unwrap().
                        set_bg(cursor.color);
                    break 'main; 
                },
                'w' => { 
                    canvas.get_mut(cursor.pos.x, cursor.pos.y).unwrap().
                        set_bg(cursor.color);
                    cursor.pos.y = cursor.pos.y.saturating_sub(1);
                    cursor.color = canvas.get_mut(cursor.pos.x, cursor.pos.y).
                        unwrap().bg();
                },
                's' => {
                    canvas.get_mut(cursor.pos.x, cursor.pos.y).unwrap().
                        set_bg(cursor.color);
                    cursor.pos.y += 1;
                    cursor.color = canvas.get_mut(cursor.pos.x, cursor.pos.y).
                        unwrap().bg();
                },
                'a' => {
                    canvas.get_mut(cursor.pos.x, cursor.pos.y).unwrap().
                        set_bg(cursor.color);
                    cursor.pos.x = cursor.pos.x.saturating_sub(1);
                    cursor.color = canvas.get_mut(cursor.pos.x, cursor.pos.y).
                        unwrap().bg();
                },
                'd' => {
                    canvas.get_mut(cursor.pos.x, cursor.pos.y).unwrap().
                        set_bg(cursor.color);
                    cursor.pos.x += 1;
                    cursor.color = canvas.get_mut(cursor.pos.x, cursor.pos.y).
                        unwrap().bg();
                },
                '\x7f' => {
                    cursor.color = Color::Default;
                    canvas.get_mut(cursor.pos.x, cursor.pos.y).unwrap().set_bg(
                        cursor.color);
                }
                '\r' => {
                    cursor.color = Grid::rand_color();
                    canvas.get_mut(cursor.pos.x, cursor.pos.y).unwrap().set_bg(
                        cursor.color);
                },
                'c' => {
                    for y in 1..rows {
                        for x in 1..cols {
                            canvas.get_mut(x, y).unwrap().
                                set_bg(Color::Default);
                        }
                    }
                }
                _ => {}
            }
        }
        if cursor.pos.x > cols {
            cursor.pos.x -= 1;
        } else if cursor.pos.x < 1 {
            cursor.pos.x += 1;
        }
        if cursor.pos.y > rows {
            cursor.pos.y -= 1;
        } else if cursor.pos.y < 1 { 
            cursor.pos.y += 1;
        }
        grid.canvas_mut().get_mut(cursor.pos.x, cursor.pos.y).unwrap().set_bg(
            Color::Red);

        grid.draw(term);
        ui.draw(term);
        term.swap_buffers().unwrap();
    }
}

fn create_ui(width: usize, height: usize) -> Dialog {
    let mut dlg = Dialog::new(width, height);
    dlg.draw_box();

    let mut title = Label::from_str("Editor Mode Controls");
    title.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,1));
    dlg.add_label(title);
   
    let mut movement = Label::from_str(
        format!("{:<10} {}","w,a,s,d", "-> Move about the board"));
    movement.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,3));
    dlg.add_label(movement);

    let mut create = Label::from_str(
        format!("{:<10} {}","Enter", "-> Set cell to random color"));
    create.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,4));
    dlg.add_label(create);

    let mut del = Label::from_str(
        format!("{:<10} {}","Backspace", "-> Set cell to default color"));
    del.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,5));
    dlg.add_label(del);

    let mut clear = Label::from_str(
        format!("{:<10} {}","c", "-> Clear screen"));
    clear.pack(&dlg, HorizontalAlign::Left, VerticalAlign::Top, (2,6));
    dlg.add_label(clear);

    let mut quit = StdButton::new("Quit", 'q', ButtonResult::Ok);
    quit.pack(&dlg, HorizontalAlign::Right, VerticalAlign::Bottom, (2,1));
    dlg.add_button(quit);

    dlg
}
