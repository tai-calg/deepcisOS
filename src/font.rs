use crate::{graphics::{Color,Draw, DrawError, Point, Rectangle, Size, DrawErrorExt}, framebuffer::Drawer};

use core::{convert::TryFrom};

const FONT_SIZE_I32: Size<i32> = Size::new(8,16);
const FONT_SIZE: Size<usize> = Size::new(8,16);
const FONT_RECT : Rectangle<usize>  = Rectangle::new(Point::new(0,0), FONT_SIZE);


include!(concat!(env!("OUT_DIR"), "/ascii_font.rs"));

type Font = [u8; 16];

fn get_ascii_font(ch: u8) -> Font{
    static_assertions::const_assert_eq!(ASCII_FONT.len(),256);
    ASCII_FONT[usize::from(ch)]
}

pub(crate) fn draw_char<D>(
    drawer: &mut D,
    pos : Point<i32>,
    ch: char,
    color : Color,
    ignore_out_of_range : bool,

) -> Result<(), DrawError> 
where D : Draw,
{
    let codepoint = u32::from(ch);
    let ch  = u8::try_from(codepoint).unwrap_or(b'?');
    let font = get_ascii_font(ch);

    let draw_rect = Rectangle{
        pos,
        size: FONT_SIZE_I32,
    };

    for (drawPos , fontPos) in draw_rect.points().zip(FONT_RECT.points()) {
        if ((font[fontPos.y]<< fontPos.x) & 0x80) != 0 {
            drawer
                .draw(drawPos, color)
                .ignore_out_of_range(ignore_out_of_range)?;
        }
    }

    Ok(())
}