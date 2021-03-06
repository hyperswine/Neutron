use core::alloc::Layout;
use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

// TO USE: in common(), call init_heap()

// FOR NOW!
pub fn init_heap() {
    // heap grows up
    let heap_start = 0xFFFF_FFFF_0000_0000;
    let heap_end = 0xFFFF_FFFF_FFFF_0000;
    let heap_size = heap_end - heap_start;
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
}

// ALLOC HANDLER
#[alloc_error_handler]
pub fn handle_alloc_error(layout: Layout) -> ! {
    loop {}
}
