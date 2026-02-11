use std::fs::read_to_string;
use std::fs::write;

#[allow(non_camel_case_types)]
pub struct csv {
    pub headers: Vec<String>,   // Vec of strings is kind of sad
    pub body: Vec<Vec<String>>, // Vec of Vec of strings is even sadder
}

// We also want to use multiple files in case one gets overwritten
// Maybe a weelky new file?
// Need to also get the date and time
impl csv {
    pub fn new() -> csv {
        csv {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }
    pub fn save_to_file(&self, file_path: &str) -> Result<(), IoError> {
        let mut contents: String = String::from("");
        for header in &self.headers {
            // The reason I don't add the commas after is because it will mess it up at the end of
            // a row
            if !(contents == "") {
                contents = contents + "," + header.as_str();
            } else {
                contents = contents + header.as_str();
            }
        }
        contents = contents + "\n";
        for i in &self.body {
            let mut is_first_iter = true;
            for v in i {
                if !is_first_iter {
                    contents = contents + "," + v.as_str();
                } else {
                    contents = contents + v.as_str();
                    is_first_iter = false;
                }
            }
            contents = contents + "\n";
        }
        match write(file_path, contents) {
            Ok(()) => return Ok(()),
            Err(_e) => {
                return Err(IoError {});
            }
        }
    }
    pub fn write_new_row(&mut self, file_path: &String, row: Vec<String>) -> std::io::Result<()> {
        let file_contents = read_to_string(file_path)?;
        let mut contents = file_contents;
        let mut is_first_iter = true;
        self.body.push(row.clone());
        for string in row {
            if !is_first_iter {
                contents = contents + "," + string.as_str();
            } else {
                contents = contents + string.as_str();
                is_first_iter = false;
            }
        }
        write(file_path, contents)?;
        Ok(())
    }
}

// Rust says that the struct is never constructed even though it is
#[allow(unused)]
pub struct IoError;

