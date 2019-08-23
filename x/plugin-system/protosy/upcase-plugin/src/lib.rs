/**
 * C-style
 */

use std::ffi::{CString};
use std::os::raw::{c_void, c_char, c_int};

#[repr(C)]
pub struct PluginData {
}

#[no_mangle]
pub unsafe extern "C" fn initialize() -> *mut c_void {
    Box::into_raw(Box::new(PluginData {
    })) as *mut c_void
}

static NAME: &[u8] = b"upcase\0";

#[no_mangle]
pub unsafe extern "C" fn name(_self: *mut c_void) -> *const c_char {
    NAME.as_ptr() as *const c_char
}

#[no_mangle]
pub unsafe extern "C" fn on_load(_self: *mut c_void) -> c_int {
    0
}

#[no_mangle]
pub unsafe extern "C" fn on_unload(_self: *mut c_void) -> c_int {
    0
}
