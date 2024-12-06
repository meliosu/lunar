#![allow(unused)]

use std::ffi::c_long;
use std::ffi::c_ulong;
use std::ffi::c_void;

use super::imp::RUNTIME;

pub type Block = unsafe extern "C" fn(*mut c_void) -> Action;

#[repr(C)]
pub(super) enum Action {
    Wait,
    Exit,
}

#[repr(C)]
#[derive(Clone)]
pub(super) struct Df {
    pub id: c_ulong,
    pub ctx: usize,
}

#[no_mangle]
extern "C" fn request(df: &mut Df) -> i32 {
    if let Some(fragment) = RUNTIME.request(df.id) {
        *df = fragment;
        0
    } else {
        -1
    }
}

#[no_mangle]
extern "C" fn submit(df: Df) {
    RUNTIME.submit(df);
}

#[no_mangle]
extern "C" fn spawn(block: Block, ctx: usize) {
    RUNTIME.spawn(block, ctx);
}

#[no_mangle]
extern "C" fn wait(df: Df) -> Df {
    RUNTIME.wait(df.id)
}

#[no_mangle]
extern "C" fn df_create() -> Df {
    Df {
        id: RUNTIME.alloc_dfid(),
        ctx: 0,
    }
}
