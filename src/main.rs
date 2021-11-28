#![no_main]
#![no_std]
#![feature(asm)]
#![feature(start)]


mod framebuffer;
use framebuffer::{Color,Point};

use core::{mem,panic::PanicInfo};
use bootloader::{boot_info::Optional, BootInfo, entry_point};



//entry point!のおかげでもうno_mangleやextern C の必要がなくなった
entry_point!(kernel_main);


//no-mangleは名前マングリング（コンパイル時に関数名に付加情報を付け足してユニークにすること）を無効にする

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {

    //画面描画
    let frameBuffer = mem::replace(&mut boot_info.framebuffer, Optional::None)
    .into_option()
    .expect("frameBuffer not supported.");

    framebuffer::init(frameBuffer);
    {
        let mut drawer = framebuffer::lock_drawer();
        for x in 0..800 {
            for y in drawer.y_range() {
                drawer.draw(Point::new(x, y), Color::WHITE);
            }
        }

    }

    loop {
        hlt();
    }

}
#[no_mangle]
fn hlt() {
    unsafe {
        asm!("hlt");
    }   
}


#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {

    loop {}
}
