use crate::alloc::{GlobalAlloc, Layout, System};


extern "C" {
    fn aligned_alloc(alignment: usize, size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[stable(feature = "alloc_system_type", since = "1.28.0")]
unsafe impl GlobalAlloc for System {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        aligned_alloc(layout.align(), layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }
}
