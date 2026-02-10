use std::fs::read_to_string;
use std::fs::write;

#[allow(non_camel_case_types)]
pub struct csv {
    headers: Vec<String>, // Vec of strings is kind of sad
    body: Vec<Vec<String>>, // Vec of Vec of strings is even sadder
}

// We also want to use multiple files in case one gets overwritten
// Maybe a weelky new file?
// Need to also get the date and time
impl csv {
    pub fn new () -> csv {
        csv {headers:Vec::new(), body:Vec::new()}
    }
    pub fn save_to_file (&self, file_path: &str) -> Result<(),IoError> {
        let mut contents: String = String::from("");
        for header in &self.headers {
            if !(contents=="") {
                contents = contents + "," + header.as_str();
            }
            else {
                contents = contents + header.as_str();
            }
        }
        contents = contents + "\n";
        for i in &self.body {
            for v in i {
                contents = contents + "," + v.as_str();
            }
            contents = contents + "\n";
        }
        match write(file_path, contents) {
            Ok(()) => return Ok(()),
            Err(_e) => {return Err(IoError{});},
        }
    }
    pub fn write_new_row (&mut self, file_path: &String, row: Vec<String>) -> std::io::Result<()> {
        let file_contents = read_to_string(file_path)?;
        let mut contents = file_contents;
        let mut is_first_iter = true;
        self.body.push(row.clone());
        for string in row {
            if is_first_iter {
                contents = contents + string.as_str();
                is_first_iter = false;
            }
            else {
                contents = contents + "," + string.as_str();
            }
        }
        write(file_path, contents)?;
        Ok(())
    }
}

// Rust says that the struct is never constructed even though it is
#[allow(unused)]
pub struct IoError;