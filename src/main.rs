#![no_std]
#![no_main]
#![feature(asm)]
#![feature(start)]




use core::panic::PanicInfo;
use core;



//no-mangleはRustにはないシステムプログラミングに必要な
//Cライブラリと同等の機能を使えるようにする。e.g. memcpyなど
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
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
