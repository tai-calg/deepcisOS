use bootloader::boot_info::{FrameBuffer, FrameBufferInfo, PixelFormat};
use spin::Mutex;
use core::{convert::TryFrom, ops::Range};
use conquer_once::spin::OnceCell;


/// Public Fields and Functions  ///////////////////////////////////////////////
pub(crate) type Point<T> = Vector2d<T>;
static INFO : OnceCell<FrameBufferInfo> = OnceCell::uninit();
static DRAWER: OnceCell<Mutex<Drawer>> = OnceCell::uninit();


static RGB_PIXEL_DRAWER: RgbPixelDrawer= RgbPixelDrawer;
static BGR_PIXEL_DRAWER: BgrPixelDrawer= BgrPixelDrawer;
static U8_PIXEL_DRAWER: U8PixelDrawer= U8PixelDrawer;
static UNSUPPORTED_PIXEL_DRAWER: UnsupportedPixelDrawer= UnsupportedPixelDrawer;

pub(crate) fn init(frameBuffer : FrameBuffer)
{
    INFO.try_init_once(|| frameBuffer.info()).expect("failed to initialize INFO.");
    DRAWER.try_init_once(|| Mutex::new(Drawer::new(frameBuffer))).expect("failed to initialize DRAWER.");
}

pub(crate) fn info() -> &'static FrameBufferInfo {
    INFO.try_get().expect("INFO is not initialized.")
}

pub(crate) fn lock_drawer () -> spin::MutexGuard<'static, Drawer> {
    // TO DO : consider interrupt
    DRAWER.try_get().expect("DRAWER is not initialized.").lock()
}

/// Private Fields and Functions  ///

fn select_pixel_drawer(pixel_format: PixelFormat) -> &'static (dyn PixelDraw + Send + Sync) {
    match pixel_format {
        PixelFormat::RGB => &RGB_PIXEL_DRAWER as _,
        PixelFormat::BGR => &BGR_PIXEL_DRAWER as _,
        PixelFormat::U8 => &U8_PIXEL_DRAWER as _,
        _ => &UNSUPPORTED_PIXEL_DRAWER as _,
    }
}



///        //////////////////////////////////////////////////////////////////////
/// Struct /////////////////////////////////////////////////////////////////////
///        /////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Color {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
}

pub(crate) struct Vector2d<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

pub(crate) struct Drawer {
    inner : FrameBuffer,
    pixel_drawer: &'static (dyn PixelDraw + Send + Sync),
}
#[derive(Debug, Clone, Copy)]
struct RgbPixelDrawer;
struct BgrPixelDrawer;
struct U8PixelDrawer;
struct UnsupportedPixelDrawer;



/// Trait //////////////////////////////////////////////////////////////////////
trait PixelDraw {
    fn pixel_draw(&self, buffer: &mut [u8], pixel_index: usize, c: Color) -> bool;
}


/// Implimentation /////////////////////////////////////////////////////////////

#[allow(dead_code)]
impl Color {
    pub(crate) const BLACK: Self = Color::new(0, 0, 0);
    pub(crate) const WHITE: Self = Color::new(255, 255, 255);
    pub(crate) const RED : Self = Color::new(255, 0, 0);
    pub(crate) const GREEN: Self = Color::new(0, 255, 0);
    pub(crate) const BLUE: Self = Color::new(0, 0, 255);
    
}

impl Color {
    pub(crate) const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub(crate) fn to_grayscale(self) -> u8 {
        u8::try_from((u16::from(self.r) + u16::from(self.g) + u16::from(self.b)) / 3).unwrap()
    }
}

impl<T> Vector2d<T> {
    pub(crate) const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

}


impl Drawer {
    pub(crate) fn new (inner : FrameBuffer) -> Self {
        let pixel_drawer = select_pixel_drawer(inner.info().pixel_format);
        Self {
            inner,
            pixel_drawer,
        }
    }
    
    pub(crate) fn info(&self) -> FrameBufferInfo {
        self.inner.info()
    }

    pub(crate) fn x_range(&self) -> Range<usize> {
        0..self.info().horizontal_resolution 
    }

    pub(crate) fn y_range(&self) -> Range<usize> {
        0..self.info().vertical_resolution 
    }

    pub(crate) fn draw<T>(&mut self, point: Point<T>, color: Color)-> bool
    where
        usize: TryFrom<T>, 
    {
        let pixel_index = match self.pixel_index(point) {
            Some(point) => point,
            None => return false,
        };

        self.pixel_drawer.pixel_draw(self.inner.buffer_mut(), pixel_index, color)

    }


    fn pixel_index<T> (&self, point: Point<T>) -> Option<usize>
    where
        usize: TryFrom<T>,
        {
            let FrameBufferInfo {
                bytes_per_pixel,
                stride,
                ..
            } = self.info();

            let x = usize::try_from(point.x).ok()?;
            let y = usize::try_from(point.y).ok()?;
            if !self.x_range().contains(&x) || !self.y_range().contains(&y) {
                return None;
            }

            Some((y * stride) + (x * bytes_per_pixel))
        }
}

impl PixelDraw for RgbPixelDrawer {
    fn pixel_draw(&self, buffer: &mut [u8], pixel_index: usize, c: Color) -> bool {
        buffer[pixel_index] = c.r;
        buffer[pixel_index + 1] = c.g;
        buffer[pixel_index + 2] = c.b;
        true
    }
}
impl PixelDraw for BgrPixelDrawer {
    fn pixel_draw(&self, buffer: &mut [u8], pixel_index: usize, c: Color) -> bool {
        buffer[pixel_index] = c.b;
        buffer[pixel_index + 1] = c.g;
        buffer[pixel_index + 2] = c.r;
        true
    }
}
impl PixelDraw for U8PixelDrawer {
    fn pixel_draw(&self, buffer: &mut [u8], pixel_index: usize, c: Color) -> bool {
        buffer[pixel_index] = c.to_grayscale();
        true
    }
}
impl PixelDraw for UnsupportedPixelDrawer {
    fn pixel_draw(&self, _buffer: &mut [u8], _pixel_index: usize, _c: Color) -> bool {
        false
    }
}



