// rustc -C prefer-dynamic --crate-type dylib test_module.rs
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn setup(arg: *const i8) -> *const i8 {
    unsafe {
        let rust_str = CStr::from_ptr(arg).to_str().expect("Invalid UTF-8 string");

        println!("{}", rust_str);
    }

    return arg;
}
