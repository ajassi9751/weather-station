use crate::io::csv::csv;
use std::fs::write;
use std::time::{SystemTime, UNIX_EPOCH};

const SECONDS_IN_A_WEEK: u32 = 604800; // 60*60*24*7
const HOME_DIRECTORY: &str = "data/";

#[allow(non_camel_case_types)]
pub struct csvmanager {
    // Manages adding times and creating weekly csvs, as well as file paths
    currentcsv: csv,     // Holds the current csv
    rowque: Vec<String>, // Holds info to be put into the row
    date_of_last_made_csv: SystemTime, // Holds info about the system time to know when to change csvs
                                       // Should use a u64 for ease of use
}

impl csvmanager {
    pub fn new(mut headers: Vec<String>, vecsize: usize) -> csvmanager {
        headers.push(String::from("Date and Time"));
        let mut temp_csv = csv::new_default();
        temp_csv.give_headers(headers);
        let time = SystemTime::now();
        write(
            HOME_DIRECTORY.to_owned() + "csvmanager.txt",
            format!(
                "{}",
                time.duration_since(UNIX_EPOCH)
                    .expect("Error getting time")
                    .as_secs()
            ),
        )
        .expect("Error writing to file");
        // Maybe don't use expect but this is at the beggining of the program anyway
        let mut rowque: Vec<String> = Vec::new();
        rowque.resize(vecsize + 1, String::from(""));
        csvmanager {
            currentcsv: temp_csv,
            rowque: rowque,
            date_of_last_made_csv: time,
        }
    }
    pub fn give_data<T: MatchIntoType>(&mut self, data: T) {
        let (value, position) = data.match_into_type();
        if position > self.rowque.len() - 1 {
            // Check this
            return;
        }
        if self.rowque[position] != "" {
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
        let length = self.rowque.len();
        self.write_row();
        self.rowque = Vec::new();
        self.rowque.resize(length, String::from(""));
    }
    fn write_row(&mut self) {}
    fn try_change_date(&mut self) {}
}

pub trait MatchIntoType {
    fn match_into_type(self) -> (String, usize);
}
