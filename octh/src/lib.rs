#![allow(non_camel_case_types)]
include!("bindings.rs");

use std::ffi::CStr;
use std::ffi::CString;
use std::mem::MaybeUninit;

pub struct StdString(pub *mut root::std::string);

impl StdString {
    /// bytes nust be terminated with the NUL character `\0`
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let name = CStr::from_bytes_with_nul(bytes).unwrap();
        let pmname = unsafe { root::stdstring_new(name.as_ptr()) };
        Self(pmname as *mut root::std::string)
    }
    
    pub fn from_string(s: &str) -> Self {
        unsafe {
            let c = CString::new(s).unwrap();
            let s = root::stdstring_new(c.as_ptr());
            Self(s as *mut root::std::string)
        }
    }
}

impl From<&str> for StdString {
    fn from(s: &str) -> Self {
        StdString::from_string(s)
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

// https://octave.org/doxygen/5.1/d6/db6/classoctave__value__list.html
pub struct OctaveValueList(pub MaybeUninit<root::octave_value_list>);

impl OctaveValueList {

    pub fn new(n: i32) -> Self {
        unsafe {
            let mut m = MaybeUninit::<value_list>::uninit();
            root::octave_value_list_new(m.as_mut_ptr(), n);
            OctaveValueList(m)
        }
    }

    pub fn unwrap(self) -> root::octave_value_list {
        unsafe {
            self.0.assume_init()
        }
    }

    pub fn length(&self) -> i32 {
        value_list_length(self.0.as_ptr())
    }

    pub fn set_value(&mut self, i: i32, value: OctaveValue) {
        unsafe {
            root::octave_value_list_set_value(self.0.as_mut_ptr(), i, value.0.as_ptr());
        }
    }

}

// https://octave.org/doxygen/5.1/d3/d35/classoctave__value.html
pub struct OctaveValue(pub MaybeUninit<root::octave_value>);

impl OctaveValue {

    pub fn new_bool(b: bool) -> Self {
        unsafe {
            let mut value = MaybeUninit::<root::octave_value>::uninit();
            root::octave_value_new_bool(value.as_mut_ptr(), b);
            OctaveValue(value)
        }
    }

    // is_bool_type is deprecated
    pub fn islogical(&self) -> bool {
        unsafe {
            root::octave_value_islogical(self.0.as_ptr())
        }
    }

    pub fn is_matrix_type(&self) -> bool {
        unsafe {
            root::octave_value_is_matrix_type(self.0.as_ptr())
        }
    }

    pub fn is_scalar_type(&self) -> bool {
        unsafe {
            root::octave_value_is_scalar_type(self.0.as_ptr())
        }
    }

    pub fn is_string(&self) -> bool {
        unsafe {
            root::octave_value_is_string(self.0.as_ptr())
        }
    }

    pub fn is_true(&self) -> bool {
        unsafe {
            root::octave_value_is_true(self.0.as_ptr())
        }
    }

    pub fn is_uint16_type(&self) -> bool {
        unsafe {
            root::octave_value_is_uint16_type(self.0.as_ptr())
        }
    }

    pub fn is_uint32_type(&self) -> bool {
        unsafe {
            root::octave_value_is_uint32_type(self.0.as_ptr())
        }
    }

    pub fn is_uint64_type(&self) -> bool {
        unsafe {
            root::octave_value_is_uint64_type(self.0.as_ptr())
        }
    }

    pub fn is_uint8_type(&self) -> bool {
        unsafe {
            root::octave_value_is_uint8_type(self.0.as_ptr())
        }
    }

    pub fn isinteger(&self) -> bool {
        unsafe {
            root::octave_value_isinteger(self.0.as_ptr())
        }
    }

    pub fn isnull(&self) -> bool {
        unsafe {
            root::octave_value_isnull(self.0.as_ptr())
        }
    }

    pub fn isinumeric(&self) -> bool {
        unsafe {
            root::octave_value_isnumeric(self.0.as_ptr())
        }
    }

    pub fn isreal(&self) -> bool {
        unsafe {
            root::octave_value_isreal(self.0.as_ptr())
        }
    }

}


// https://octave.org/doxygen/5.1/d3/d35/classoctave__value.html
pub struct Matrix(pub MaybeUninit<root::Matrix>);

impl Matrix {

    pub fn new() -> Self {
        unsafe {
            let mut m = MaybeUninit::<root::Matrix>::uninit();
            root::Matrix_new(m.as_mut_ptr());
            Matrix(m)
        }
    }

    // is_empty is deprecated
    pub fn isempty(&self) -> bool {
        unsafe {
            root::Matrix_isempty(self.0.as_ptr())
        }
    }

    pub fn to_value(self) -> OctaveValue {
        unsafe {
            let mut m = MaybeUninit::<root::octave_value>::uninit();
            root::octave_value_new_Matrix(m.as_mut_ptr(), self.0.as_ptr());
            OctaveValue(m)
        }
    }

}

impl Into<OctaveValue> for Matrix {
    fn into(self) -> OctaveValue {
        self.to_value()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_length() {
        let argout = OctaveValueList::new(0);
        assert_eq!(argout.length(),0);
        let argout = OctaveValueList::new(1);
        assert_eq!(argout.length(),1);
        let argout = OctaveValueList::new(2);
        assert_eq!(argout.length(),2);
        let argout = OctaveValueList::new(3);
        assert_eq!(argout.length(),3);
        let argout = OctaveValueList::new(5);
        assert_eq!(argout.length(),5);
        let argout = OctaveValueList::new(8);
        assert_eq!(argout.length(),8);
        let argout = OctaveValueList::new(13);
        assert_eq!(argout.length(),13);
        let argout = OctaveValueList::new(21);
        assert_eq!(argout.length(),21);
    }

    #[test]
    fn test_value_islogical() {
        assert!(OctaveValue::new_bool(true).islogical());
    }

    #[test]
    fn test_matrix_isempty() {
        assert!(Matrix::new().isempty());
    }

    #[test]
    fn test_value_is_matrix_type() {
        assert!(Matrix::new().to_value().is_matrix_type());
    }

}