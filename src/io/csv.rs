use std::fs::read_to_string;
use std::fs::write;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct csv <const N: usize> {
    headers: [Box<str>; N],   // Vec of strings is kind of sad
    body: Vec<[Box<str>; N]> // Vec of Vec of strings is even sadder
}

#[allow(dead_code)]
impl <const N: usize> csv <N> {
    pub fn new_default() -> csv<N> {
        csv {
            headers: std::array::from_fn(|_i| { "".into()}),
            body: Vec::new(),
        }
    }
    pub fn new(headers: [Box<str>; N], body: Vec<[Box<str>; N]>) -> csv<N> {
        csv {
            headers: headers,
            body: body,
        }
    }
    pub fn give_headers(&mut self, headers: [Box<str>; N]) {
        self.headers = headers;
    }
    pub fn give_body(&mut self, body: Vec<[Box<str>; N]>) {
        self.body = body;
    }
    pub fn save_to_file(&self, file_path: &str) -> std::io::Result<()> {
        let mut contents: String = String::from("");
        for header in &self.headers {
            // The reason I don't add the commas after is because it will mess it up at the end of
            // a row
            if !contents.is_empty() {
                contents = contents + "," + header;
            } else {
                contents += header;
            }
        }
        contents += "\n";
        for i in &self.body {
            let mut is_first_iter = true;
            for v in i {
                if !is_first_iter {
                    contents = contents + "," + v;
                } else {
                    contents += v;
                    is_first_iter = false;
                }
            }
            contents += "\n";
        }
        write(file_path, contents)?;
        Ok(())
    }
    pub fn write_new_row(&mut self, file_path: &str, row: [Box<str>; N]) -> std::io::Result<()> {
        #[cfg(debug_assertions)]
        {
            println!("File path: {}", file_path);
        }
        let file_contents = read_to_string(file_path)?;
        #[cfg(debug_assertions)]
        {
            println!("Completed read");
        }
        let mut contents = file_contents;
        let mut is_first_iter = true;
        self.body.push(row.clone());
        for string in row {
            if !is_first_iter {
                contents = contents + "," + &string;
            } else {
                contents += &string;
                is_first_iter = false;
            }
        }
        contents += "\n";
        write(file_path, contents)?;
        Ok(())
    }
    pub fn new_row(&mut self, row: [Box<str>; N]) {
        self.body.push(row);
    }
    pub fn get_headers(&self) -> &[Box<str>] {
        &self.headers
    }
    pub fn get_body(&self) -> &Vec<[Box<str>; N]> {
        &self.body
    }
    // Add a function to parse a csv ino the body, like parse_into_body
    pub fn parse_into_body(&mut self, file_path: &str) -> std::io::Result<i32> {
        #[cfg(debug_assertions)]
        {
            println!("parse_into_body called!");
            println!("file_path: {}", file_path);
        }
        let file_contents = read_to_string(file_path)?;
        #[cfg(debug_assertions)]
        {
            println!("file contents:\n{}", file_contents);
        }
        let mut current_element: String = String::from("");
        let mut current_vec: [Option<Box<str>>; N] = [const{None}; N];
        let mut body: Vec<[Box<str>; N]> = Vec::new();
        let mut headers: i32 = 0;
        let mut is_first_row: bool = true;
        for (i, char) in file_contents.chars().into_iter().enumerate() {
            // println!("{}", char);
            if is_first_row {
                if char == ',' {
                    headers += 1;
                } else if char == '\n' {
                    is_first_row = false;
                    headers += 1;
                } else {
                    continue;
                }
            } else {
                if char == ',' {
                    current_vec[i] = Some(current_element.into_boxed_str());
                    current_element = String::from("");
                } else if char == '\n' {
                    body.push(current_vec.map(|element| element.unwrap()));
                    current_vec = [const{None}; N];
                } else {
                    // Idk if this works
                    let mut buf = [0u8; 4];
                    current_element += char.encode_utf8(&mut buf);
                }
            }
        }
        self.body = body;
        Ok(headers)
    }
    pub fn remove_body(&mut self) {
        self.body = Vec::new()
    }
}
