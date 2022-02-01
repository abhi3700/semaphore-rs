mod hash;
mod identity;
mod merkle_tree;
mod poseidon_tree;
mod protocol;
mod util;

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

#[no_mangle]
pub extern "C" fn generate_identity_commitment(seed: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(seed) };
    let seed = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    let id = identity::Identity::new(seed.as_bytes());

    CString::new(id.commitment().to_str_radix(10))
        .unwrap()
        .into_raw()
}
