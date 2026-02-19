use std::fs::read_to_string;
use std::fs::write;

#[allow(non_camel_case_types)]
pub struct csv {
    headers: Vec<String>,   // Vec of strings is kind of sad
    body: Vec<Vec<String>>, // Vec of Vec of strings is even sadder
}

// We also want to use multiple files in case one gets overwritten
// Maybe a weekly new file?
impl csv {
    pub fn new_default() -> csv {
        csv {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }
    pub fn new(headers: Vec<String>, body: Vec<Vec<String>>) -> csv {
         csv {
            headers: headers,
            body: body,
        }       
    }
    pub fn give_headers(&mut self, headers: Vec<String>) {
        self.headers = headers;
    }
    pub fn give_body(&mut self, body: Vec<Vec<String>>) {
        self.body = body;
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
    pub fn write_new_row(&mut self, file_path: &str, row: Vec<String>) -> std::io::Result<()> {
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
    pub fn new_row(&mut self, row: Vec<String>) {
        self.body.push(row);
    }
}

// Rust says that the struct is never constructed even though it is
#[allow(unused)]
pub struct IoError;

