use crate::prelude::*;
use bootloader::boot_info::{MemoryRegion,MemoryRegionKind};
use core::{cmp, num};
use x86_64::{structures::paging::{frame::PhysFrameRange, FrameAllocator,Size4KiB ,PhysFrame},PhysAddr};

const fn kib(kib: u64)-> u64 {
    kib * 1024
}
const fn mib(mib: u64)-> u64 {
    mib * kib(1024)
}
const fn gib(gib: u64)-> u64 {
    gib * mib(1024)
}

pub(crate) const BYTES_PER_FRAME: u64 = kib(4);
const MAX_PHYDICAL_MEMORY_BYTE: u64 = gib(128);
const FRAME_COUNT: u64 = MAX_PHYDICAL_MEMORY_BYTE / BYTES_PER_FRAME; //frameの数

type Mapline = u64; //mapline?
const BITS_PER_MAP_LINE:u64 = Mapline::BITS as u64;
const ALLOC_MAP_LEN : usize = (FRAME_COUNT / (BYTES_PER_FRAME as u64 )) as usize;



static  MEMORY_MANAGER: spin::Mutex<BitmapMemoryManager> = spin::Mutex::new(BitmapMemoryManager {
    alloc_map: [0;ALLOC_MAP_LEN],
    range: PhysFrameRange { start: unsafe {
        PhysFrame::from_start_address_unchecked(PhysAddr::new_truncate(0))
    }, end: unsafe {
        PhysFrame::from_start_address_unchecked(PhysAddr::new_truncate(0))
    }},
});


pub(crate) fn lock_memory_manager() -> Result<spin::MutexGuard<'static, BitmapMemoryManager>> {
    MEMORY_MANAGER
        .try_lock()
        .ok_or_else(|| make_error!(ErrorKind::WouldBlock("MEMORY_MANAGER")))
}

pub(crate) struct BitmapMemoryManager {
    alloc_map : [Mapline; ALLOC_MAP_LEN],
    range : PhysFrameRange,
}
impl BitmapMemoryManager {
    pub(crate) fn init (&mut self, regions: impl IntoIterator<Item = MemoryRegion>)->Result<()> 
    {
        let mut available_start = self.range.start;
        let mut available_end = self.range.end;
        for region in regions {
            let start = PhysFrame::from_start_address(PhysAddr::new(region.start))?;
            let end = PhysFrame::from_start_address(PhysAddr::new(region.end))?;
            if available_end < start {
                self.mark_allocated(PhysFrame::range(available_end,start)); //?
            }

            if region.kind == MemoryRegionKind::Usable {
                available_start = cmp::min(available_start, start);
                available_end = cmp::max(available_end, end);
            }else {
                self.mark_allocated(PhysFrame::range(start, end));
            }
        }

        self.range = PhysFrame::range(available_start, available_end);
        Ok(())

    }


    pub(crate) fn mark_allocated(&mut self, range:PhysFrameRange) {
        for frame in range { self.set_bit(frame, true); }
    }

    pub(crate) fn allocate (&mut self, num_frames: usize) -> Result<PhysFrameRange> {
        let mut start_frame = self.range.start;
        
        loop{
            let endframe = start_frame +num_frames as u64;
            if endframe > self.range.end {
                bail!(ErrorKind::NoEnoughMemory);
            }

            let range = PhysFrame::range(start_frame,endframe );
            if let Some(allocated) = range.clone().find(|frame| self.get_bit(*frame))
            {
                start_frame = allocated + 1;
                continue;
            }

            self.mark_allocated(range);
            return Ok(range);
        }
    }

    pub(crate) fn free(&mut self, range: PhysFrameRange) {
        for frame in range {
            self.set_bit(frame, false)
        }
    }

    fn get_bit(&self, frame: PhysFrame) -> bool 
    {
        let frame_index = frame.start_address().as_u64()/ BYTES_PER_FRAME;
        let line_index = (frame_index/ BITS_PER_MAP_LINE) as usize;
        let bit_index = frame_index % BITS_PER_MAP_LINE;

        (self.alloc_map[line_index] & (1<< bit_index)) != 0
    }


    fn set_bit(&mut self, frame: PhysFrame, already_allocated: bool)
    {
        let frame_index = frame.start_address().as_u64() / BYTES_PER_FRAME;
        let line_index = (frame_index/ BITS_PER_MAP_LINE ) as usize;
        let bit_index = frame_index % BITS_PER_MAP_LINE;

        if already_allocated 
        {
            self.alloc_map[line_index] |= 1 << bit_index;
        }else {
            self.alloc_map[line_index] &= !(1 << bit_index);
        }
    }
} 

unsafe impl FrameAllocator<Size4KiB> for BitmapMemoryManager {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        self.allocate(1).map(|range| range.start).ok()
    }

}