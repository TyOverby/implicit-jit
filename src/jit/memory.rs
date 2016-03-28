use libc;
use std::{mem, ptr};

const PAGE_SIZE: usize = 4096;

pub struct JitMemory {
     pub bytes: Vec<u8>
}

impl JitMemory {
    pub fn new() -> JitMemory {
        JitMemory {
            bytes: vec![]
        }
    }

    unsafe fn get_mem(size: usize) -> *mut libc::c_void {
        let mut page : *mut libc::c_void = mem::uninitialized();
        libc::posix_memalign(&mut page, PAGE_SIZE, size);
        page
    }

    pub unsafe fn as_function(&self) -> (extern "C" fn(f32, f32) -> f32) {
        let size = self.bytes.len();
        let page = JitMemory::get_mem(size);

        libc::mprotect(page, size, libc::PROT_READ | libc::PROT_WRITE);
        let page: *mut u8 = mem::transmute(page);
        ptr::copy::<u8>(&self.bytes[0], page, size);
        let page: *mut libc::c_void = mem::transmute(page);
        libc::mprotect(page, size, libc::PROT_EXEC);

        mem::transmute(page)
    }
}
