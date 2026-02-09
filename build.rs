use cc::Build;
use std::fs::read_dir;
use std::process::exit;

fn main() {
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
}

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