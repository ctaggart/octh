#![allow(non_snake_case)]
#![allow(unused_variables)]

extern crate octh;

// https://thefullsnack.com/en/string-ffi-rust.html
use std::ffi::CString;

#[no_mangle]
pub unsafe extern "C"  fn Ghelloworld (shl: *const octh::root::octave::dynamic_library, relative: bool) -> *mut octh::root::octave_dld_function {
    let name = CString::new("helloworld").unwrap();
    let pname = name.as_ptr() as *const octh::root::std::string;
    std::mem::forget(pname);

    let doc = CString::new("Hello World Help String").unwrap();
    let pdoc = doc.as_ptr() as *const octh::root::std::string;
    std::mem::forget(pdoc);

    return octh::root::octave_dld_function_create(Some(Fhelloworld), shl, pname, pdoc);
}    

pub unsafe extern "C"  fn Fhelloworld (args: *const octh::root::octave_value_list, nargout: ::std::os::raw::c_int) -> octh::root::octave_value_list {
    let list: *mut octh::root::octave_value_list = ::std::ptr::null_mut();
    octh::root::octave_value_list_new(list);
    std::mem::forget(list);
    return *list;
}

// 27 |     return *list;
//    |            ^^^^^ cannot move out of dereference of raw pointer