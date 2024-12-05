#![allow(unused)]

use std::ffi::c_long;
use std::ffi::c_void;

#[repr(C)]
pub(super) enum Action {
    Wait,
    Exit,
}

#[repr(C)]
pub(super) struct Df {
    id: c_long,
    data: *mut c_void,
}

#[no_mangle]
extern "C" fn request(df: &mut Df) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn submit(df: Df) {
    todo!()
}

#[no_mangle]
extern "C" fn spawn(block: unsafe extern "C" fn(*mut c_void) -> Action, ctx: *mut c_void) {
    todo!()
}

#[no_mangle]
extern "C" fn wait(df: Df) -> Df {
    todo!()
}

#[no_mangle]
extern "C" fn df_create() -> Df {
    todo!()
}

extern "C" {
    #[no_mangle]
    pub(super) static entry: unsafe extern "C" fn(*mut c_void) -> Action;
}
