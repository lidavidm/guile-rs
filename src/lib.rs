#![feature(braced_empty_structs)]
#![feature(libc)]
extern crate guile_sys;
extern crate libc;

use std::ffi;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;

// use libc::c_void;

pub struct GuileVM {
}

pub fn init<F>(func: F)
    where F: Fn(GuileVM) {
    unsafe {
        guile_sys::scm_with_guile(Some(with_guile_callback::<F>), &func as *const _ as *mut c_void);
    }
}

impl GuileVM {
    pub fn shell(&self, args: Vec<String>) {
        unsafe {
            let mut argv: Vec<*mut c_char> = args.into_iter().map(|arg| {
                ffi::CString::new(arg).unwrap().into_raw()
            }).collect();
            let argv_ptr = argv.as_mut_ptr();
            guile_sys::scm_shell(argv.len() as i32, argv_ptr);
        }
    }
}

unsafe extern fn with_guile_callback<F>(data: *mut c_void) -> *mut c_void where F: Fn(GuileVM) {
    let callback = data as *mut F;

    let vm = GuileVM {};

    (*callback)(vm);

    std::ptr::null_mut()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        init(|_| {
            println!("Hello guile!");
        });
    }
}
