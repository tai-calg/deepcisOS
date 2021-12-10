#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use crate::{
    font,framebuffer,
    graphics::{Color, Draw, Point, Rectangle, Size}, desktop,
};
use core::{convert::TryFrom, fmt};


const ROWS: usize = 25;
const COLUMNS: usize = 80;



//マクロの記法わからん
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write as _; //?
    if let Ok(Some(mut framebuffer)) = framebuffer::try_lock_drawer() {
        if let Some(mut console) = CONSOLE.try_lock() {
            #[allow(clippy::unwrap_used)]
            console.create_writer(&mut *framebuffer).write_fmt(args).unwrap();
        }
    }
}

                        /// Console /// 

static CONSOLE: spin::Mutex<Console> = spin::Mutex::new(Console {
    buffer: [[0; COLUMNS];ROWS],
    fg_color: desktop::FG_COLOR,
    bg_color: desktop::BG_COLOR,
    cursor: Point::new(0,0),
});

//drawerを保持しない, console をstatic変数にして mutex にする
pub(crate) struct Console 
{
    fg_color: Color,
    bg_color: Color,
    buffer: [[u8; COLUMNS]; ROWS],
    cursor: Point<usize>,
}



impl Console  {

    pub(crate) fn create_writer<'d, 'c, D>(&'c mut self, drawer:&'d mut D)-> ConsoleWriter<'d, 'c,D> {
        ConsoleWriter {
            drawer,
            console:self,
        }
    }
    fn write_str(&mut self, s : &str) -> RedrawArea {
        let mut redraw = RedrawArea::new();
        for ch in s.chars() {
            let byte = font::char_to_byte(ch);
            if byte == b'\n' {
                self.newline(&mut redraw);
                continue;
            }

            if self.cursor.x < COLUMNS -1 {
                redraw.add(self.cursor);
                self.buffer[self.cursor.y][self.cursor.x] = byte;
                self.cursor.x += 1;
            }
        }

        redraw
    }

    fn newline (&mut self, redraw: &mut RedrawArea) {
        self.cursor.x = 0;
        if self.cursor.y < ROWS -1 {
            self.cursor.y += 1;
            return;
        }

        //update buffer 
        for (src, dst ) in (1..).zip(0..(ROWS -1)) {
            self.buffer[dst] = self.buffer[src];
            //下の行の内容を一行上の配列に代入する。０行目は代入先がないがそれが通常処理。
        }
        self.buffer[ROWS-1].fill(0b0); //最終行に0...0をパディング


        //redraw whole console
        redraw.fill_bg = true;
        redraw.area = Rectangle{
            pos: Point::new(0,0),
            size : Size::new(COLUMNS, ROWS),
        };
    }    
    
}


//ここでdrawerを持つ（分離してそれらを持つようにラップする）
pub(crate) struct  ConsoleWriter<'d, 'c, D> {
    drawer: &'d mut D,
    console:&'c mut Console,
}

impl<'d, 'c, D> ConsoleWriter<'d, 'c, D>
where D: Draw, 
{
    fn to_draw_point(&self,p:Point<usize>)-> Point<i32> {
        let fontsize = font::FONT_PIXEL_SIZE;
        #[allow(clippy::unwrap_used)]
        Point {
            x : i32::try_from(p.x).unwrap() * fontsize.x,
            y : i32::try_from(p.y).unwrap() * fontsize.y,
        }
    }

    fn to_draw_rect (&self,rect: Rectangle<usize>) ->Rectangle<i32> {
        Rectangle {
            pos: self.to_draw_point(rect.pos),
            size: self.to_draw_point(rect.size),
        }
    }
}

struct RedrawArea {
    area: Rectangle<usize>,
    fill_bg: bool,
}

impl RedrawArea {
    fn new() -> Self {
        Self {
            area : Rectangle {
                pos : Point::new(0,0),
                size : Point::new(0, 0),},
            fill_bg: false,    
        }
    }

    fn add(&mut self, p : Point<usize>) {
        if self.area.size.x == 0 || self.area.size.y == 0 { // ?
            self.area.pos = p;
            self.area.size = Size::new(1,1);
            return;
        }
        self.area = self.area.extend_to_contain(p);
    }
}


//impl trait for のなかにはtrait の抽象関数の実装しかできない。
impl<'d, 'c, D> fmt::Write for ConsoleWriter<'d, 'c, D>
where D:Draw
{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let redraw = self.console.write_str(s);
        if redraw.fill_bg {
        let rect = self.to_draw_rect(redraw.area);
        self.drawer.fill_rect(rect, self.console.bg_color);
        }

        for console_y in redraw.area.y_range() {
            let xrange = redraw.area.x_range();
            let console_p = Point::new(redraw.area.x_start(), console_y);

            let bytes = &self.console.buffer[console_y][xrange]; //&はアドレスを渡してると思えばいい
            let draw_p = self.to_draw_point(console_p);
            font::draw_byte_string(self.drawer, draw_p, bytes, self.console.fg_color);//一行書く（関数内でｘのloopは回してる）
        }



        
        Ok(())
    }
}
