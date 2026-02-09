use cc::Build;
use std::fs::read_dir;
use std::process::exit;

fn main () {
    let mut filenames: Vec<String> = Vec::new();
    match read_dir("c/") {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(v) => {
                        let filename = v.file_name().into_string().expect("Issue reading filename");
                        // Don't compile header files
                        if filename.ends_with(".c") {
                            filenames.push(filename);
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
    let mut compilenames: Vec<String> = filenames.clone();
    for name in compilenames.iter_mut() {
        // Removes .c file extension
        name.truncate(name.len() - 2);
    }
    for (src, libname) in filenames.iter().zip(compilenames.iter()) {
        Build::new().file(format!("c/{}", src)).compile(libname.as_str());
    }
    // Build::new().file("c/main.c").compile("main");
}