use std::ffi::c_void as void;

#[repr(C)]
pub struct Id {
    pub value: u64,
}

#[repr(C)]
pub struct Df {
    pub id: Id,
    pub value: *mut void,
}

#[repr(C)]
pub struct Cf {
    pub id: Id,
    pub block: *mut void,
    pub context: *mut void,
}

#[repr(C)]
pub enum Ret {
    Continue,
    Wait,
    Exit,
}

#[no_mangle]
extern "C" fn request(_this: &mut Cf, _df: &mut Df) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn submit(_this: &mut Cf, _df: &mut Df) -> i32 {
    todo!()
}

#[no_mangle]
extern "C" fn spawn(_this: &mut Cf, _block: *mut void, _context: *mut void) {
    todo!()
}

#[no_mangle]
extern "C" fn destroy(_this: &mut Cf, _df: &mut Df) {
    todo!()
}
