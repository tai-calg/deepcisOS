#![no_main]
#![no_std]
#![feature(asm)]
#![feature(start)]




use core::panic::PanicInfo;
use bootloader::{BootInfo, boot_info, entry_point};


//entry point!のおかげでもうno_mangleやextern C の必要がなくなった
entry_point!(kernel_main);


//no-mangleは名前マングリング（コンパイル時に関数名に付加情報を付け足してユニークにすること）を無効にする

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {

        let mut value = 0x90;
        for byte in framebuffer.buffer_mut() {
            *byte = value;
            value = value.wrapping_add(1);
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
