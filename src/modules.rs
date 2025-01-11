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

pub fn module_call(module: &str, service: &str) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let lib =
            libloading::Library::new(module)?;
        let func_setup: libloading::Symbol<unsafe extern "C" fn(arg: *const u8) -> *const i8> =
            lib.get(b"setup")?;
        let func_service: libloading::Symbol<unsafe extern "C" fn() -> *const i8> =
            lib.get(b"service")?;

        let res = func_service();
        let service_name = CStr::from_ptr(res).to_str().expect("Invalid UTF-8 string");

        if service_name != service {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Service name mismatch",
            )));        	
        }

        let my_str = String::from("Hello, world!!!");
        let res = func_setup(my_str.as_ptr());

        let rust_str = CStr::from_ptr(res).to_str().expect("Invalid UTF-8 string");

        println!("Result: {:?}", rust_str);

        Ok(())
    }
}
