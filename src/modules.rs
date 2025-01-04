extern crate glob;
use glob::glob;
use libloading::{Library, library_filename, Symbol};

pub fn modules_find(path: &str) -> Vec<String> {
    let mut modules = Vec::new();

    for file in glob(path).expect("Failed to find files") {
        if let Ok(file) = file {
            modules.push(file.display().to_string());
        }
    }

    modules
}

pub fn modules_load(modules: &Vec<String>) {
    for module in modules {
        println!("Loading module: {}", module);

        unsafe {
            let lib = Library::new(library_filename(module)).unwrap();
            let func = lib.get(b"setup");

            match func {
                Ok(func) => {
                    let func: Symbol<unsafe extern "C" fn() -> ()> = func;
                    func();
                },
                Err(e) => {
                    println!("Error, failed to load module: {}", e);
                    continue;
                }
            }
        }
    }
}
