#![warn(unsafe_op_in_unsafe_fn)]
#![no_std]
#![no_main]


use self::error::{Error,ErrorKind,Result};
use crate::graphics::{Color, Point,Size};
use bootloader::{boot_info::{Optional,MemoryRegion}, entry_point, BootInfo};
use graphics::{Draw, Rectangle};
use x86_64::{VirtAddr, structures::paging::Translate};
use core::{ mem};


mod prelude;
mod mouse;
mod font;
mod framebuffer;
mod graphics;
mod console;
mod paging;
mod desktop;
mod error;
mod memory;
mod pci;

struct MemoryRegions<'a> {
    regions: core::slice::Iter<'a, MemoryRegion>,
}

impl<'a> MemoryRegions<'a> 
{
    fn new(regions: &'a [MemoryRegion])-> Self {
        Self {
            regions: regions.iter(),
        }
    }
}
impl<'a> Iterator for MemoryRegions<'a> {
    type Item = MemoryRegion;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current = *self.regions.next()?;
        loop {
            #[allow(clippy::suspicious_operation_groupings)]
            match self.regions.as_slice().get(0) {
                Some(next) if current.kind == next.kind && current.end == next.start => {
                    current.end = next.end;
                    let _ = self.regions.next();
                    continue;
                }
                
                _ => return Some(current),
            }
        }
    }
}



entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuffer = mem::replace(&mut boot_info.framebuffer, Optional::None)
        .into_option()
        .expect("framebuffer not supported");
    framebuffer::init(framebuffer).expect("failed to initialize framebuffer");



    // Memory Management
    let physical_memory_offset = boot_info.physical_memory_offset
        .as_ref().copied().expect("physical memory is not mapped");
    let physical_memory_offset = VirtAddr::new(physical_memory_offset);
    let mapper = unsafe { paging::init(physical_memory_offset)};

    
    let addresses = &[
        0xb8000,
        0x201008,
        0x0100_0020_1a10,
        physical_memory_offset.as_u64(),
        ];
        
        for &addr in addresses 
        {
            let virt = VirtAddr::new(addr);
            let phys = mapper.translate(virt);
            println!("{:?} -> {:?}", virt, phys);
        }
        
        
        
        
        
        
        // 描画
        desktop::draw().expect("failed to draw desktop");
        let mut allocator = memory::lock_memory_manager();

    allocator.init(MemoryRegions::new(&* boot_info.memory_regions))
        .expect("failed to init bitmap memory manager");



        println!("welcome to deepcisOS !  ");
    
    mouse::draw_cursor().expect("failed to draw mouse");

    for region in MemoryRegions::new(&* boot_info.memory_regions)
    {


        println!(
            "addr = {:08x}-{:08x}, pages = {:08x}, kind = {:?}  ",
            region.start,
            region.end,
            (region.end - region.start) / 4096,
            region.kind,
        );
    }

    {
        let frames1 = allocator.allocate(3).expect("failed to allocate");
        println!("allocated: {:?}", frames1);
        let frames2 = allocator.allocate(5).expect("failed to allocate");
        println!("allocated: {:?}", frames2);
        let frames3 = allocator.allocate(4).expect("failed to allocate");
        println!("allocated: {:?}", frames3);
        let frames4 = allocator.allocate(3).expect("failed to allocate");
        println!("allocated: {:?}", frames4);
    }
    


    //for p in drawer.area().points(){
    //    drawer.draw(p, Color::WHITE).expect("fail to draw");
    //}

    //これが長方形を生成
    //let rect = Rectangle::new(Point::new(300,200), Size::new(200,200));
    //for p in rect.points() {
    //    drawer.draw(p, Color::RED).expect("failed to draw Red");
    //}


    //文字描画




    hlt_loop();
}

fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}