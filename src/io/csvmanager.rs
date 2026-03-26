#[cfg(not(feature = "rust_only"))]
use crate::c::ctime::get_c_time;
use crate::io::csv::csv;
use chrono::{Datelike, Duration, Local};
use std::fs::{read_to_string, write};

const HOME_DIRECTORY: &str = "data/";

#[cfg(feature = "rust_only")]
fn get_c_time() -> String {
    String::from("Disable the rust_only feature for dates to work")
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq)]
pub struct csvmanager {
    // Manages adding times and creating weekly csvs, as well as file paths
    currentcsv: csv,     // Holds the current csv
    rowque: Vec<String>, // Holds info to be put into the row
    prevcsvname: String,
}

impl csvmanager {
    // Creates a new csvmanager based on the headers you give it
    // It tries to read from a csvmanager.txt which gives it info about the last week the program was active
    // If that week is this week then it recovers the weeks file
    // If that fails it just ignores it and moves on as it will auto create the file later on
    // If the weeks don't match, it writes this week to the manager file
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
        #[cfg(debug_assertions)]
        {
            println!("contents: {}, csvname: {}", contents, csvname);
        }
        if contents == csvname {
            // If it cant read the file, just give it a default body to be safe
            match temp_csv.parse_into_body(format!("{}{}", HOME_DIRECTORY, csvname).as_str()) {
                Ok(_) => {}
                Err(_) => temp_csv.give_body(Vec::new()),
            }
            // Read the contents and parse them
        } else {
            // Write the this week to other file
            write(HOME_DIRECTORY.to_owned() + "csvmanager.txt", &csvname)
                .expect("Error writing to file");
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
    // This is the main function that users will interact with
    // Makes it easy to just shove whatever data in here and it handles the rest
    pub fn give_data<T: MatchIntoType>(&mut self, data: T) {
        // I love this syntax
        let (value, position) = data.match_into_type();
        // Make sure position isn't out of the Vec or the last element (reserved for dates)
        if position >= self.rowque.len() - 1 {
            // Check this
            return;
        }
        if self.rowque[position].is_empty() {
            self.rowque[position] = value;
            #[cfg(debug_assertions)]
            {
                println!("{:?}", self.rowque);
            }
            self.try_to_write_row();
        }
    }
    // Tries to write a row by checking if any spaces are empty
    // If its valid, it adds the data and time and calls the write function
    fn try_to_write_row(&mut self) {
        for (i, string) in self.rowque.iter().enumerate() {
            // Last element should be ""
            if i == self.rowque.len() - 1 {
                break;
            }
            if string.is_empty() {
                return;
            }
        }
        // Make the last element the time and date
        let rowque_index: usize = self.rowque.len() - 1;
        self.rowque[rowque_index] = get_c_time();
        // Write the row
        self.write_row();
    }
    // Does some cloning to write the new row and cleans up rowque
    fn write_row(&mut self) {
        #[cfg(debug_assertions)]
        {
            println!("writing row");
        }
        let length = self.rowque.len();
        // Dumb heap allocation for no reason
        let csvname = self.get_csv_name().to_owned();
        let rowque = self.rowque.clone();
        #[cfg(debug_assertions)]
        {
            println!("Rowque:\n{:?}", rowque);
        }
        // Should use the result here
        let _ = self.currentcsv.write_new_row(csvname.as_str(), rowque);
        self.rowque = Vec::new();
        self.rowque.resize(length, String::from(""));
    }
    // Gets the name of the csv right now
    // Changes the csv if needed but idk if that should be done here
    fn get_csv_name(&mut self) -> &str {
        let today = Local::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday() as i64;
        let monday = today - Duration::days(days_since_monday);
        let csvname = format!("{}{}.csv", HOME_DIRECTORY, monday.format("%Y-%m-%d"));
        if self.prevcsvname.is_empty() {
            // A file with the name of the date of this week's monday
            self.prevcsvname = csvname;
            self.prevcsvname.as_str()
        } else if self.prevcsvname != csvname {
            self.prevcsvname = csvname;
            // Idk if this should be done here
            self.change_csv();
            self.prevcsvname.as_str()
        } else {
            self.prevcsvname.as_str()
        }
    }
    // Changes to a "new" csv by removing th ebody of it and writing to csvmanager.txt
    fn change_csv(&mut self) {
        self.currentcsv.remove_body();
        // Maybe don't use .expect becuase this runs every week and could lead to data loss upon restart
        write(
            HOME_DIRECTORY.to_owned() + "csvmanager.txt",
            self.prevcsvname.as_str(),
        )
        .expect("Error writing to file");
    }
}

pub trait MatchIntoType {
    fn match_into_type(self) -> (String, usize);
}
