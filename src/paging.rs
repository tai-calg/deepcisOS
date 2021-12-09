use x86_64::{
    structures::paging::{OffsetPageTable,PageTable, page_table},
    VirtAddr
};

/// initialize a new offsetPageTable
/// 
/// 
//physical_memory_offsetもvartual addr型（ただ役割はただのオフセット）
pub unsafe fn init (physical_memory_offset:VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = unsafe {
        active_level_4_table(physical_memory_offset)
    };

    unsafe {OffsetPageTable::new(level_4_table, physical_memory_offset)}
}

/// returns a mutable reference to the active level 4 table
pub unsafe fn active_level_4_table(physical_memory_offset:VirtAddr)-> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _ ) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = physical_memory_offset + phys.as_u64(); //offset と物理addrを加算したのをviraddrとする
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();
    //virtual addrのアドレスをページングテーブルのアドレスに割り当てる

    unsafe{&mut *page_table_ptr}
}