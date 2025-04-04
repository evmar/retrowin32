use windows_sys::Win32::{Foundation::GlobalFree, System::Memory::GlobalAlloc};

struct Alloc {}

#[global_allocator]
static ALLOCATOR: Alloc = Alloc {};

unsafe impl Sync for Alloc {}

unsafe impl core::alloc::GlobalAlloc for Alloc {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        assert!(layout.align() <= 8); // GlobalAlloc only supports 8-byte alignment
        GlobalAlloc(0, layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        GlobalFree(ptr as *mut _);
    }
}
