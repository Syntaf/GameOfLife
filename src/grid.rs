use rand::distributions::{IndependentSample, Range};
use rand;

use rustty::{
    Size, 
    HasSize,
    Cell,
    CellAccessor,
    Color
};

use rustty::ui::core::{
    Widget,
    Frame,
    Painter,
    HorizontalAlign,
    VerticalAlign
};

use rustty::ui::Canvas;

const ADJ: [(i32, i32); 8] = 
    [(-1, -1), (-1, 0), (-1, 1), 
     (0, -1), (0, 1), (1, 1), 
     (1, 0), (1, -1)];

pub enum Action {
   Create,
   Kill
}

pub struct Grid {
    canvas: Canvas,
    actions: Vec<((usize, usize), Action)>
}

impl Grid {
    pub fn new(cols: usize, rows: usize) -> Grid {
        // Create a canvas of rows by cols. The border of the canvas
        // will take up 1x1, so the actual size of raw is one less
        let mut canvas_ = Canvas::new(cols, rows);
        canvas_.draw_box();

        let (x, y) = (cols/2, rows/2);

        let color = Range::new(0, 7);
        let mut rng = rand::thread_rng();

        canvas_.get_mut(x, y).unwrap().set_bg(Grid::rand_color());
        canvas_.get_mut(x+1, y).unwrap().set_bg(Grid::rand_color());
        canvas_.get_mut(x+2, y).unwrap().set_bg(Grid::rand_color());
        

        Grid {
            canvas: canvas_,
            actions: Vec::new()
        }
    }

    fn rand_color() -> Color {
        let color = Range::new(0, 6);
        let mut rng = rand::thread_rng();

        let value = color.ind_sample(&mut rng);
        Color::Byte(
            if value >= 3 {
                value + 1
            } else {
                value
            })
    }

    pub fn neighbors(&self, x: usize, y: usize) -> u8{
        let mut cnt = 0u8;
        for &(r, c) in &ADJ {
            let (x1, y1) = (x as i32 + r, y as i32 + c);
            if x1 > 1 && y1 > 1 {
                let (x2, y2) = (x1 as usize, y1 as usize);
                if let Some(a) = self.canvas.get(x2, y2) {
                    if a.bg() != Color::Default {
                        cnt += 1;
                    }
                }
            }
        }
        cnt
    }

    pub fn set_alive(&mut self, x: usize, y: usize) {
        self.actions.push(
            ((x, y),
             Action::Create)
            );
        //self.canvas.get_mut(r, j).unwrap().set_bg(Grid::rand_color()); 
    }

    pub fn set_dead(&mut self, x: usize, y: usize) {
        self.actions.push(
            ((x, y),
             Action::Kill)
            );
        //self.canvas.get_mut(r, j).unwrap().set_bg(Color::Default);
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        if self.canvas.get(x, y) == None { 
            panic!(format!("{}, {}", x, y));
        }
        if self.canvas.get(x, y).unwrap().bg() != Color::Default {
            return true
        }
        false
    }

    pub fn playable_size(&self) -> Size {
        (self.canvas.size().0 - 1, self.canvas.size().1 - 1)
    }

    pub fn update(&mut self) {
        while let Some(((x, y), act)) = self.actions.pop() {
            match act { 
               Action::Create   => { self.canvas.get_mut(x, y).unwrap()
                                         .set_bg(Grid::rand_color()); },
               Action::Kill     => { self.canvas.get_mut(x, y).unwrap()
                                         .set_bg(Color::Default); }
            }
        }
    }
}

impl Widget for Grid {
    fn draw(&mut self, parent: &mut CellAccessor) {
        self.canvas.draw(parent);
    }

    fn pack(&mut self, parent: &HasSize, halign: HorizontalAlign, valign: VerticalAlign,
            margin: (usize, usize)) {
        self.canvas.pack(parent, halign, valign, margin);
    }

    fn resize(&mut self, new_size: Size) {
        self.canvas.resize(new_size);
    }

    fn draw_box(&mut self) {
        self.canvas.draw_box();
    }

    fn frame(&self) -> &Frame {
        self.canvas.frame()
    }

    fn frame_mut(&mut self) -> &mut Frame {
        self.canvas.frame_mut()
    }
}
