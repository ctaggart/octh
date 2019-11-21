#![allow(non_camel_case_types)]
include!("bindings.rs");

use std::ffi::CStr;
use std::mem::MaybeUninit;

pub struct StdString(pub *mut root::std::string);

impl StdString {
    pub fn from_bytes_with_nul(bytes: &[u8]) -> Self {
        let name = CStr::from_bytes_with_nul(bytes).unwrap();
        let pmname = unsafe { root::stdstring_new(name.as_ptr()) };
        Self(pmname as *mut root::std::string)
    }
}

pub type dynamic_library = root::octave::dynamic_library;
pub type value_list = root::octave_value_list;
pub type builtin_fcn = extern "C" fn(list: *const value_list, nargout: i32) -> value_list;

pub type dld_function = root::octave_dld_function;

pub fn dld_function_create(function: builtin_fcn, library: *const dynamic_library, name: StdString, help: StdString, relative: bool) -> *mut dld_function {
    unsafe {
        let f = root::octave_dld_function_create(Some(function), library, name.0, help.0);
        if relative {
            root::octave_dld_function_mark_relative(f);
        }
        f
    }
}

pub fn value_list_length(list: *const value_list) -> i32 {
    unsafe {
        root::octave_value_list_length(list)
    }
}

pub fn value_list_new(n: i32) -> value_list {
    unsafe {
        let mut list = MaybeUninit::<value_list>::uninit();
        root::octave_value_list_new(list.as_mut_ptr(), n);
        list.assume_init()
    }
}