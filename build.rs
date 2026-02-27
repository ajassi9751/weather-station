#[cfg(not(feature = "rust_only"))]
use cc::Build;

use std::fs::read_dir;
use std::process::exit;

// I wonder if there is a way to auto install wiringPi, that would be great

// Only compiles if c code is allowed
#[cfg(not(feature = "rust_only"))]
fn main() {
    // unsafe {
    //     std::env::set_var("CFLAGS", "-l wiringPi");
    // }
    let mut filenames: Vec<String> = Vec::new();
    get_file_paths(&mut filenames, "c/");
    let mut compilenames: Vec<String> = Vec::new();
    get_file_names(&mut compilenames, "c/");
    for name in compilenames.iter_mut() {
        // Removes .c file extension
        name.truncate(name.len() - 2);
    }
    for (src, libname) in filenames.iter().zip(compilenames.iter()) {
        Build::new().file(src).compile(libname.as_str());
    }
    // Build::new().file("c/main.c").compile("main");
    // Only links to wiringPi if we are allowed to use a pi
    #[cfg(not(feature = "no_pi"))]
    {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-lib=wiringPi");
    }
}

#[cfg(not(feature = "rust_only"))]
fn get_file_paths(filenames: &mut Vec<String>, path: &str) {
    match read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(v) => {
                        if v.file_type().expect("Error getting file type").is_dir() {
                            get_file_names(filenames, v.path().display().to_string().as_str());
                        } else {
                            let filename =
                                v.file_name().into_string().expect("Issue reading filename");
                            // Don't compile header files
                            if filename.ends_with(".c") {
                                // Double check this
                                filenames.push(format!("{}/{}", path, filename));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Illegal file/directory access error: {}", e);
                        exit(1);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Illegal file/directory access error: {}", e);
            exit(1);
        }
    }
}

#[cfg(not(feature = "rust_only"))]
fn get_file_names(filenames: &mut Vec<String>, path: &str) {
    match read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(v) => {
                        if v.file_type().expect("Error getting file type").is_dir() {
                            get_file_names(filenames, v.path().display().to_string().as_str());
                        } else {
                            let filename =
                                v.file_name().into_string().expect("Issue reading filename");
                            // Don't compile header files
                            if filename.ends_with(".c") {
                                // Double check this
                                filenames.push(filename);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Illegal file/directory access error: {}", e);
                        exit(1);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Illegal file/directory access error: {}", e);
            exit(1);
        }
    }
}
