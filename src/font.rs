use crate::{graphics::{Color,Draw,  Point, Rectangle, Size}};

use core::{convert::TryFrom, fmt};

pub(crate) const FONT_PIXEL_SIZE: Size<i32> = Size::new(8, 16);

include!(concat!(env!("OUT_DIR"), "/ascii_font.rs"));

type Font = [u8; 16];

fn get_ascii_font(ch: u8) -> &'static Font{
    static_assertions::const_assert_eq!(ASCII_FONT.len(),256);
    &ASCII_FONT[usize::from(ch)]
}


pub(crate) fn draw_char<D>(drawer: &mut D, pos : Point<i32>, ch: char,color : Color)
where D : Draw,
{
    let byte = char_to_byte(ch);
    draw_byte(drawer, pos, byte, color)

}

//1文字かく     
pub(crate) fn draw_byte<D>(drawer: &mut D, pos : Point<i32>, byte:u8, color : Color,)
where D :Draw,
{
    let font = get_ascii_font(byte);
    let draw_rect = Rectangle {pos , size: FONT_PIXEL_SIZE};

    for(fonty, drawy) in draw_rect.y_range().enumerate() {
        for (fontx, drawx) in draw_rect.x_range().enumerate() {
            if (font[fonty] << fontx) & 0x80 != 0 {
                drawer.draw(Point::new(drawx, drawy), color);
            }
        }
    }
}

pub(crate) fn draw_byte_string<D> (drawer: &mut D, pos : Point<i32>, bytes:&[u8], color : Color)
where D :Draw
{
    let mut pos = pos;
    for byte in bytes {
        draw_byte(drawer, pos, *byte, color);
        pos.x  += FONT_PIXEL_SIZE.x;
    }
}

pub(crate) fn char_to_byte (ch: char) -> u8 {
    let codepoint = u32::from(ch);
    u8::try_from(codepoint).unwrap_or(b'#')
}

//StringDrawと同じ機能だが一応作っておく
pub(crate) fn draw_str <D> (
    drawer: &mut D,
    pos : Point<i32>,
    string : &str,
    color : Color,
)
where D : Draw,
{
    let mut pos = pos; //why?
    for ch in string.chars() {
        draw_char(drawer, pos, ch, color);
        pos.x += FONT_PIXEL_SIZE.x;
    }
}


//write!を使えるようにするためのstruct,これをnewして、write!にぶち込む。かなり冗長だけどね
#[derive(Debug)]
pub(crate) struct StringDrawer<'d , D> {
    drawer: &'d mut D,
    pos: Point<i32>,
    color: Color,
}

impl<'d, D> StringDrawer<'d, D>{
    pub(crate) fn new(
        drawer: &'d mut D,
        start_pos: Point<i32>,
        color: Color,
    ) -> Self{
        Self{
            drawer,
            pos: start_pos,
            color,
        }
    }
}
/// `Point` のインスタンス化
/// let point: Point = Point { x: 10.3, y: 0.4 };

impl<'d, D> fmt::Write for StringDrawer<'d, D>
where D : Draw,
{
    fn write_str (&mut self, s: &str) -> fmt::Result {
        for ch in s.chars() {
            draw_char(self.drawer, self.pos, ch, self.color);
            self.pos.x += FONT_PIXEL_SIZE.x;
        }

        Ok(())
    }
}
