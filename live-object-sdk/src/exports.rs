use std::alloc::{alloc, dealloc, Layout};
use std::slice;

/// A block of memory in the WASM (guest) memory.
#[repr(C)]
pub struct MemorySlice {
    /// A pointer to the start of this memory slice,
    /// measured in bytes from the beginning of the WASM (guest) memory.
    pub ptr: u32,
    /// The number of bytes in this memory slice.
    pub len: u32,
}

/// Allocate `MemorySlice` of the given length and returns a pointer to it.
#[no_mangle]
pub extern "C" fn allocate(len: u32) -> u32 {
    MemorySlice::new(len).into_raw_ptr() as u32
}

/// Deallocate `MemorySlice` referenced by the given pointer.
#[no_mangle]
pub extern "C" fn deallocate(memory_slice_ptr: u32) {
    MemorySlice::from_raw_ptr(memory_slice_ptr as *mut MemorySlice).release();
}

impl MemorySlice {
    /// Create a new `MemorySlice` and allocate the underlying memory slice.
    pub fn new(len: u32) -> Self {
        Self {
            ptr: unsafe { alloc(MemorySlice::layout(len)) } as u32,
            len,
        }
    }
    /// Create a new `MemorySlice` from the given pointer.
    pub fn from_raw_ptr(memory_slice_ptr: *mut MemorySlice) -> Self {
        unsafe { *Box::from_raw(memory_slice_ptr) }
    }

    /// Consume the `MemorySlice` and returns a pointer to it.
    pub fn into_raw_ptr(self) -> *mut MemorySlice {
        Box::into_raw(Box::new(self))
    }

    /// Read the underlying memory slice.
    pub fn read(&self) -> Vec<u8> {
        unsafe { slice::from_raw_parts(self.ptr as *const u8, self.len as usize).to_vec() }
    }

    /// Write the given data to the underlying memory slice.
    pub fn write(&self, data: &[u8]) {
        let slice = unsafe { slice::from_raw_parts_mut(self.ptr as *mut u8, self.len as usize) };
        slice.copy_from_slice(data);
    }

    /// Consume and deallocate the underlying memory slice.
    pub fn release(self) {
        unsafe {
            dealloc(self.ptr as *mut u8, MemorySlice::layout(self.len));
        }
    }

    /// Return the layout for the given length.
    fn layout(len: u32) -> Layout {
        Layout::array::<u8>(len as usize).expect("MemorySlice: failed to create layout")
    }
}
