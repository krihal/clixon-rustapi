extern crate glob;

use std::ffi::CStr;
use glob::glob;
use libloading::{Library, library_filename, Symbol};
use log::info;

pub fn modules_find(path: &str) -> Vec<String> {
    let mut modules = Vec::new();

    for file in glob(path).expect("Failed to find files") {
        if let Ok(file) = file {
        	info!("Found module: {}", file.display());
            modules.push(file.display().to_string());
        }
    }

    modules
}

pub fn module_call(module: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib =
            libloading::Library::new(module)?;
        let func: libloading::Symbol<unsafe extern "C" fn(arg: *const u8) -> *const i8> =
            lib.get(b"setup")?;

        let my_str = String::from("Hello, world!!!");
        let res = func(my_str.as_ptr());

        let rust_str = CStr::from_ptr(res).to_str().expect("Invalid UTF-8 string");

        println!("Result: {:?}", rust_str);

        Ok(())
    }
}
