use octh::*;

#[no_mangle]
pub extern "C" fn Ghelloworld (library: *const dynamic_library, relative: bool) -> *mut dld_function {
    let help = "Hello World Help String";
    dld_function_create(helloworld, library, "helloworld".into(), help.into(), relative)
}

extern "C" fn helloworld (argin: *const value_list, nargout: i32) -> value_list {
    let nargin = value_list_length(argin);
    println!("Hello World has {} input arguments and {} output arguments.", nargin, nargout);
    let mut argout = OctaveValueList::new(nargout);
    for i in 0..nargout {
        argout.set_value(i, Matrix::new().to_value());
    }
    argout.unwrap()
}