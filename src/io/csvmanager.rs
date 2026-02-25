use crate::c::ctime::get_c_time;
use crate::io::csv::csv;
use std::fs::{read_to_string, write};

const SECONDS_IN_A_WEEK: u32 = 604800; // 60*60*24*7
const HOME_DIRECTORY: &str = "data/";

#[allow(non_camel_case_types)]
pub struct csvmanager {
    // Manages adding times and creating weekly csvs, as well as file paths
    currentcsv: csv,     // Holds the current csv
    rowque: Vec<String>, // Holds info to be put into the row
    // The next two are possibly not needed
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
        let days_since_monday = today.weekday().num_days_from_monday();
        let csvname = format!("{}", today - Duration::days(days_since_monday as i64));
        match result {
            Ok(v) => contents = v,
            Err(_) => {
                write(
                    HOME_DIRECTORY.to_owned() + "csvmanager.txt",
                    csvname).expect("Error writing to file");
            }
        }
        if contents != "" {
            if contents == csvname {
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
        let (value, position) = data.match_into_type();
        // Make sure position isn't out of the Vec or the last element (reserved for dates)
        if position >= self.rowque.len() - 1 {
            // Check this
            return;
        }
        if self.rowque[position] == "" {
            self.rowque[position] = value;
            self.try_to_write_row();
        }
    }
    fn try_to_write_row(&mut self) {
        for string in &self.rowque {
            if string == "" {
                return;
            }
        }
        // Make the last element the time and date
        self.rowque[self.rowque.len() - 1] = get_c_time();
        // Write the row
        self.write_row();
    }
    fn write_row(&mut self) {
        let length = self.rowque.len();
        self.currentcsv.write_new_row(get_csv_name(), self.rowque);
        self.rowque = Vec::new();
        self.rowque.resize(length, String::from(""));
    }
    fn get_csv_name(&mut self) -> &str {
        let today = Local::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday();
        let csvname = format!("{}.csv", today - Duration::days(days_since_monday as i64));
        if self.prevcsvname == "" {
            // A file with the name of the date of this week's monday 
            self.prevcsvname = csvname;
            self.prevcsvname.as_str()
        } else if self.prevcsvname != csvname {
            self.prevcsvname = csvname;
            self.change_csv();
            self.prevcsvname.as_str()
        }
        else {
            self.prevcsvname.as_str()
        }
    }
    fn change_csv(&mut self) {
        let headers = self.currentcsv.get_headers().clone();
        self.currentcsv = csv::new_default();
        self.currentcsv.give_headers(headers);
    }
}

pub trait MatchIntoType {
    fn match_into_type(self) -> (String, usize);
}
