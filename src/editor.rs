use grid::Grid;

use rustty::{
    Terminal, 
    Event,
    HasSize,
    CellAccessor,
    Cell,
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
    lpos: Position,
    color: Color,
    lcolor: Color,
}

#[derive(Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

pub fn open(grid: &mut Grid, term: &mut Terminal) {
    let mut cursor = Cursor {
        pos: Position { x: 2, y: 2 },
        lpos: Position { x: 2, y: 2},
        color: Color::Default,
        lcolor: Color::Default,
    };

    let (cols, rows) = grid.playable_size();
    term.swap_buffers().unwrap();
    'main: loop {
        while let Some(Event::Key(ch)) = term.get_event(0).unwrap() {
            let canvas = grid.canvas_mut();
            match ch {
                'q' => { break 'main; },
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
                '\r' => {
                    cursor.color = Grid::rand_color();
                    canvas.get_mut(cursor.pos.x, cursor.pos.y).unwrap().set_bg(
                        cursor.color);
                },
                _ => {}
            }
        }
        if cursor.pos.x > cols {
            cursor.pos.x -= 1;
        }
        if cursor.pos.y > rows {
            cursor.pos.y -= 1;
        }
        //grid.canvas_mut().get_mut(cursor.lpos.x, cursor.lpos.y).unwrap().set_bg(
        //    cursor.lcolor);
        grid.canvas_mut().get_mut(cursor.pos.x, cursor.pos.y).unwrap().set_bg(
            Color::Red);

        //cells[(cursor.lpos.x, cursor.lpos.y)].set_bg(cursor.lcolor);
        //cells[(cursor.pos.x, cursor.pos.y)].set_bg(cursor.color);
        grid.draw(term);
        term.swap_buffers().unwrap();
    }
}
