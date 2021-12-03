#![warn(unsafe_op_in_unsafe_fn)]
#![no_std]
#![no_main]

use crate::graphics::{Color, Point,Size};
use bootloader::{boot_info::Optional, entry_point, BootInfo};
use console::Console;
use graphics::{Draw, Rectangle};
use core::{fmt::{Write}, mem};
use font::StringDrawer;

mod font;
mod framebuffer;
mod graphics;
mod console;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = mem::replace(&mut boot_info.framebuffer, Optional::None)
        .into_option()
        .expect("framebuffer not supported");
    framebuffer::init(framebuffer).expect("failed to initialize framebuffer");

    //Drawer をMutexにしていじれるのはただ一つにしてからDrawerを生成
    let mut drawer = framebuffer::lock_drawer().expect("failed to get framebuffer");


    for p in drawer.area().points(){
        drawer.draw(p, Color::WHITE).expect("fail to draw");
    }

    //これが長方形を生成
    let rect = Rectangle::new(Point::new(300,200), Size::new(200,200));
    for p in rect.points() {
        drawer.draw(p, Color::RED).expect("failed to draw Red");
    }


    //文字描画
    let mut console = Console::new(&mut *drawer, Color::BLACK, Color::WHITE);

    for i in 0..80 {
        writeln!(&mut console, "line {}",i).expect("failed to draw");
    }


    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}