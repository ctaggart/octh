use octh::*;

#[no_mangle]
pub extern "C" fn Ghelloworld (library: *const dynamic_library, relative: bool) -> *mut dld_function {
    let name = StdString::from_bytes_with_nul(b"helloworld\0");
    let help = StdString::from_bytes_with_nul(b"Hello World Help String\0");
    dld_function_create(helloworld, library, name, help, relative)
}

extern "C" fn helloworld (args: *const value_list, nargout: i32) -> value_list {
    let nargin = value_list_length(args);
    println!("Hello World has {} input arguments and {} output arguments.", nargin, nargout);
    value_list_new(0)
}