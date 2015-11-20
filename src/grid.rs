use rand::distributions::{IndependentSample, Range};
use rand;

use rustty::{
    Size, 
    HasSize,
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

const ADJ: [(i8, i8); 8] = 
    [(-1, -1), (-1, 0), (-1, 1), 
     (-1, 0), (1, 0), (1, 1), 
     (1, 0), (1, -1)];

pub struct Grid {
    canvas: Canvas,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Grid {
        // Create a canvas of rows by cols. The border of the canvas
        // will take up 1x1, so the actual size of raw is one less
        let mut canvas_ = Canvas::new(rows, cols);
        canvas_.draw_box();

        let (x, y) = (rows/2, cols/2);

        let color = Range::new(0, 7);
        let mut rng = rand::thread_rng();

        /*
        canvas_.get_mut(x, y).unwrap().set_bg(Color::Byte(0x6));
        canvas_.get_mut(x+1, y).unwrap().set_bg(Color::Byte(0x7));
        canvas_.get_mut(x+2, y).unwrap().set_bg(Color::Byte(0x8));
        */
        canvas_.get_mut(x, y).unwrap().set_bg(Grid::rand_color());
        canvas_.get_mut(x+1, y).unwrap().set_bg(Grid::rand_color());
        canvas_.get_mut(x+2, y).unwrap().set_bg(Grid::rand_color());
        

        Grid {
            canvas: canvas_,
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

    pub fn neighbors(&self, r: usize, j: usize) -> u8{
        let mut cnt = 0u8;
        for k in ADJ.into_iter() {
            let (i, c) = (k.0, k.1);
            let (r, j) = (r as i8 + i, j as i8 + c);
            if r > 0 && j > 0 {
                let (r, j) = (r as usize, j as usize);
                if let Some(a) = self.canvas.get(r, j) {
                    if a.bg() != Color::Default {
                        cnt += 1;
                    }
                }
            }
        }
        cnt
    }

    pub fn set_alive(&mut self, r: usize, j: usize) {
        self.canvas.get_mut(r, j).unwrap().set_bg(Grid::rand_color()); 
    }

    pub fn set_dead(&mut self, r: usize, j: usize) {
        self.canvas.get_mut(r, j).unwrap().set_bg(Color::Default);
    }

    pub fn is_alive(&self, r: usize, j: usize) -> bool {
        if self.canvas.get(r, j).unwrap().bg() != Color::Default {
            return true
        }
        false
    }

    pub fn playable_size(&self) -> Size {
        (self.canvas.size().0 - 1, self.canvas.size().1 - 1)
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
