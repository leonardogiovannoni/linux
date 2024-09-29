use crate::bindings;

/// # Safety
/// currently deeply unsafe
pub unsafe fn set_bpf_hello_world_func(ptr: *mut core::ffi::c_void) {
    unsafe {
        bindings::set_bpf_hello_world_func(ptr);
    }
}

/// # Safety
/// currently deeply unsafe
pub unsafe fn reset_bpf_hello_world_func() {
    unsafe {
        bindings::reset_bpf_hello_world_func();
    }
}

pub unsafe fn btf_range(start: *mut *mut core::ffi::c_void, end: *mut *mut core::ffi::c_void) {
    unsafe { 
        bindings::get_btf_range(start, end)
    }
}


/* 
pub struct VmallocArray<T> {
    // make it invariant over T for simplicity
    data: *mut T,
    len: usize,
}

impl<T: Default> for VmallocArray<T> {
    pub fn new(len: usize) -> Result<Self, AllocError> {
        let size = core::mem::size_of::<T>() * len;
        // SAFETY: just a ffi call
        let data = unsafe { kvrealloc_noprof(core::ptr::null(), 0, size, GFP_KERNEL); };
        if data.is_null() {
            return Err(AllocError::OutOfMemory);
        }
        let data = data as *mut MaybeUninit<T>;

        for i in 0..len {
            // SAFETY: we just allocated the memory, so it is safe to write to it
            unsafe { data.add(i).write(Default::default()); }
        }
        let data = data as *mut T; 
        Ok(Self { data, len })
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx >= self.len {
            return None;
        }
        // SAFETY: we know that idx < len, so it is safe to access the data
        unsafe { Some(&*self.data.add(idx)) }
    }

    pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
        if idx >= self.len {
            return None;
        }
        // SAFETY: we know that idx < len, so it is safe to access the data
        unsafe { Some(&mut *self.data.add(idx)) }
    }

}

impl Drop for VmallocArray<T> {
    fn drop(&mut self) {
        // SAFETY: just calling vfree on a pointer we own
        let ptr = self.data as *mut core::ffi::c_void as *const _;
        unsafe { kvfree(ptr); }
    }
}



*/


