#![no_main]
#![no_std]
#![feature(asm)]
#![feature(start)]




use core::panic::PanicInfo;
use bootloader::{BootInfo, boot_info, entry_point};


//entry point!のおかげでもうno_mangleやextern C の必要がなくなった
entry_point!(kernel_main);


//no-mangleはRustにはないシステムプログラミングに必要な
//Cライブラリと同等の機能を使えるようにする。e.g. memcpyなど

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
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
