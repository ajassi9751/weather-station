#[cfg(not(feature = "rust_only"))]
use crate::c::ctime::get_c_time;
use crate::io::csv::csv;
use chrono::{Duration, Local, Datelike};
use std::fs::{read_to_string, write};

const HOME_DIRECTORY: &str = "data/";

#[cfg(feature = "rust_only")]
fn get_c_time () -> String {
    String::from("Disable the rust_only feature for dates to work")
}

#[allow(non_camel_case_types)]
pub struct csvmanager {
    // Manages adding times and creating weekly csvs, as well as file paths
    currentcsv: csv,     // Holds the current csv
    rowque: Vec<String>, // Holds info to be put into the row
    prevcsvname: String,
}

impl csvmanager {
    pub fn new(mut headers: Vec<String>) -> csvmanager {
        headers.push(String::from("Date and Time"));
        let mut temp_csv = csv::new_default();
        let vecsize = headers.len();
        temp_csv.give_headers(headers);
        let result = read_to_string(HOME_DIRECTORY.to_owned() + "csvmanager.txt");
        let mut contents = String::from("");
        let today = Local::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday() as i64;
        let monday = today - Duration::days(days_since_monday);
        let monday_str = monday.format("%Y-%m-%d").to_string();
        let csvname = format!("{}.csv", monday_str); 
        match result {
            Ok(v) => contents = v,
            Err(_) => {
                write(HOME_DIRECTORY.to_owned() + "csvmanager.txt", &csvname)
                    .expect("Error writing to file");
            }
        }
        if contents != "" {
            if contents == csvname {
                // If it cant read the file, just give it a default body to be safe
                match temp_csv.parse_into_body(csvname.as_str()) {
                    // Could use the given result here but idk how
                    Ok(_) => {},
                    Err(_) => temp_csv.give_body(Vec::new()),
                }
                // Read the contents and parse them
            }
        }
        let mut rowque: Vec<String> = Vec::new();
        rowque.resize(vecsize, String::from(""));
        csvmanager {
            currentcsv: temp_csv,
            rowque: rowque,
            prevcsvname: String::from(""),
        }
    }
    // You can give data of any enum that implements the give_data trait
    // This is potentially needed for multithreading
    // Ensures data arrives in correct order with type saftey
    pub fn give_data<T: MatchIntoType>(&mut self, data: T) {
        // I love this syntax
        let (value, position) = data.match_into_type();
        // Make sure position isn't out of the Vec or the last element (reserved for dates)
        if position >= self.rowque.len() - 1 {
            // Check this
            return;
        }
        if self.rowque[position] == "" {
            self.rowque[position] = value;
            println!("{:?}", self.rowque);
            self.try_to_write_row();
        }
    }
    fn try_to_write_row(&mut self) {
        for (i, string) in self.rowque.iter().enumerate() {
            // Last element should be ""
            if i == self.rowque.len()-1 {
                break;
            }
            if string == "" {
                return;
            }
        }
        // Make the last element the time and date
        let rowque_index: usize = self.rowque.len() - 1;
        self.rowque[rowque_index] = get_c_time();
        // Write the row
        self.write_row();
    }
    fn write_row(&mut self) {
        let length = self.rowque.len();
        // Dumb heap allocation for no reason
        let csvname = self.get_csv_name().to_owned();
        let rowque = self.rowque.clone();
        // Should use the result here
        let _ = self.currentcsv.write_new_row(csvname.as_str(), rowque);
        self.rowque = Vec::new();
        self.rowque.resize(length, String::from(""));
    }
    fn get_csv_name(&mut self) -> &str {
        let today = Local::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday() as i64;
        let monday = today - Duration::days(days_since_monday);
        let csvname = format!("{}.csv", monday.format("%Y-%m-%d").to_string());
        if self.prevcsvname == "" {
            // A file with the name of the date of this week's monday
            self.prevcsvname = csvname;
            self.prevcsvname.as_str()
        } else if self.prevcsvname != csvname {
            self.prevcsvname = csvname;
            self.change_csv();
            self.prevcsvname.as_str()
        } else {
            self.prevcsvname.as_str()
        }
    }
    fn change_csv(&mut self) {
        self.currentcsv.remove_body()
    }
}

pub trait MatchIntoType {
    fn match_into_type(self) -> (String, usize);
}
