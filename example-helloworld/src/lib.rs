use std::ffi::CStr;
use std::mem::MaybeUninit;

#[no_mangle]
pub unsafe extern "C" fn Ghelloworld (shl: *const octh::root::octave::dynamic_library, _relative: bool) -> *mut octh::root::octave_dld_function {
    let name = CStr::from_bytes_with_nul(b"helloworld\0").unwrap();
    let pmname = octh::root::stdstring_new(name.as_ptr());
    let sname = pmname as *const octh::root::std::string;

    let doc = CStr::from_bytes_with_nul(b"Hello World Help String\0").unwrap();
    let pmname = octh::root::stdstring_new(doc.as_ptr());
    let sdoc = pmname as *const octh::root::std::string;

    let fcn = octh::root::octave_dld_function_create(Some(Fhelloworld), shl, sname, sdoc);
    // if relative {
    //     fcn.mark_relative();
    // }
    return fcn;
}

#[allow(non_snake_case)]
unsafe extern "C" fn Fhelloworld (args: *const octh::root::octave_value_list, nargout: i32) -> octh::root::octave_value_list {
    let nargin = octh::root::octave_value_list_length(args);
    println!("Hello World has {} input arguments and {} output arguments.", nargin, nargout);

    let mut mlist = MaybeUninit::<octh::root::octave_value_list>::uninit();
    let plist = mlist.as_mut_ptr();
    octh::root::octave_value_list_new(plist, 0);
    return mlist.assume_init();
}