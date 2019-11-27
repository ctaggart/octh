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

pub fn value_list_new(n: i32) -> value_list {
    unsafe {
        let mut list = MaybeUninit::<value_list>::uninit();
        root::octave_value_list_new(list.as_mut_ptr(), n);
        list.assume_init()
    }
}

// https://octave.org/doxygen/5.1/d3/d35/classoctave__value.html
pub struct OctaveValue(pub root::octave_value);

impl OctaveValue {

    pub fn new_bool(b: bool) -> Self {
        unsafe {
            let mut value = MaybeUninit::<root::octave_value>::uninit();
            root::octave_value_new_bool(value.as_mut_ptr(), b);
            OctaveValue(value.assume_init())
        }
    }

    // is_bool_type is deprecated
    pub fn islogical(&self) -> bool {
        unsafe {
            root::octave_value_islogical(&self.0)
        }
    }

    pub fn is_matrix_type(&self) -> bool {
        unsafe {
            root::octave_value_is_matrix_type(&self.0)
        }
    }

    pub fn is_scalar_type(&self) -> bool {
        unsafe {
            root::octave_value_is_scalar_type(&self.0)
        }
    }

    pub fn is_string(&self) -> bool {
        unsafe {
            root::octave_value_is_string(&self.0)
        }
    }

    pub fn is_true(&self) -> bool {
        unsafe {
            root::octave_value_is_true(&self.0)
        }
    }

    pub fn is_uint16_type(&self) -> bool {
        unsafe {
            root::octave_value_is_uint16_type(&self.0)
        }
    }

    pub fn is_uint32_type(&self) -> bool {
        unsafe {
            root::octave_value_is_uint32_type(&self.0)
        }
    }

    pub fn is_uint64_type(&self) -> bool {
        unsafe {
            root::octave_value_is_uint64_type(&self.0)
        }
    }

    pub fn is_uint8_type(&self) -> bool {
        unsafe {
            root::octave_value_is_uint8_type(&self.0)
        }
    }

    pub fn isinteger(&self) -> bool {
        unsafe {
            root::octave_value_isinteger(&self.0)
        }
    }

    pub fn isnull(&self) -> bool {
        unsafe {
            root::octave_value_isnull(&self.0)
        }
    }

    pub fn isinumeric(&self) -> bool {
        unsafe {
            root::octave_value_isnumeric(&self.0)
        }
    }

    pub fn isreal(&self) -> bool {
        unsafe {
            root::octave_value_isreal(&self.0)
        }
    }

}


// https://octave.org/doxygen/5.1/d3/d35/classoctave__value.html
pub struct Matrix(pub root::Matrix);

impl Matrix {

    pub fn new() -> Self {
        unsafe {
            let mut matrix = MaybeUninit::<root::Matrix>::uninit();
            root::Matrix_new(matrix.as_mut_ptr());
            Matrix(matrix.assume_init())
        }
    }

    // is_empty is deprecated
    pub fn isempty(&self) -> bool {
        unsafe {
            root::Matrix_isempty(&self.0)
        }
    }

    fn to_value(self) -> OctaveValue {
        unsafe {
            let mut value = MaybeUninit::<root::octave_value>::uninit();
            root::octave_value_new_Matrix(value.as_mut_ptr(), &self.0);
            OctaveValue(value.assume_init())
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
        let argout = value_list_new(0);
        assert_eq!(value_list_length(&argout),0);
        let argout = value_list_new(1);
        assert_eq!(value_list_length(&argout),1);
        let argout = value_list_new(2);
        assert_eq!(value_list_length(&argout),2);
        let argout = value_list_new(3);
        assert_eq!(value_list_length(&argout),3);
        let argout = value_list_new(5);
        assert_eq!(value_list_length(&argout),5);
        let argout = value_list_new(8);
        assert_eq!(value_list_length(&argout),8);
        let argout = value_list_new(13);
        assert_eq!(value_list_length(&argout),13);
        let argout = value_list_new(21);
        assert_eq!(value_list_length(&argout),21);
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