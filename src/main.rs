// C functions
#[cfg(not(feature = "rust_only"))]
mod c;
// Stuff for csvs
mod io;
// Used for multithreading
#[cfg(not(feature = "no_pi"))]
use std::{thread, time, sync::{Arc, Mutex}, process::Command};

#[cfg(not(feature = "no_pi"))]
use crate::io::csvmanager;
// Manages files and has the Data type
use crate::io::{csvmanager::csvmanager, data::Data};
// We basically have a rust storage backend for managing data but tons of c code for interacting with sensors and wiringPi
// To individually compile a c file that uses wiringPi make sure to use the -l wiringPi flag

// To install wiring pi use wget https://github.com/WiringPi/WiringPi/releases/download/3.18/wiringpi_3.18_arm64.deb
// Then use sudo apt install ./[filename]
// Wiringpi is only build for arm so it will have to be tested on the pi

// This program is optimized for battery usage and will poll instruments every 5 min and should take
// Battery life into account
// Writing to files will not use buffered writes as every 5 min makes them worthless
fn main() {
    // let headers: Vec<String> = vec![String::from("Temp"), String::from("Coolness")];
    // let body: Vec<Vec<String>> = vec![
    //     vec![String::from("Nice"), String::from("and Cool")],
    //     vec![String::from("Really"), String::from("Cool")],
    // ];
    // let mut mycsv = crate::io::csv::csv::new(headers, body);
    // let _ = mycsv.save_to_file("data/my.csv");
    // let _ = mycsv.write_new_row(
    //     "data/my.csv",
    //     vec![String::from("really"), String::from("really")],
    // );
    let headers: Vec<String> = vec![
        String::from("Tempurature"),
        String::from("Humidity"),
        String::from("Air Pressure"),
        String::from("Wind Speed")
    ];
    let mut csvman = csvmanager::new(headers);
    csvman.give_data(Data::Tempurature(100.1));
    csvman.give_data(Data::Humidity(100.1));
    csvman.give_data(Data::AirPressure(100.1));
    csvman.give_data(Data::WindSpeed(100.1));
    #[cfg(not(feature = "rust_only"))]
    {
        c_tests()
    }
    #[cfg(not(feature = "no_pi"))]
    {
        pi_data();
    }
}

#[cfg(not(feature = "no_pi"))]
fn pi_data() {
    let headers: Vec<String> = vec![
        String::from("Tempurature"),
        String::from("Humidity"),
        String::from("Air Pressure"),
        String::from("Wind Speed")
    ];
    // The csv manager instance
    let csvman: Arc<Mutex<csvmanager>> = Arc::new(Mutex::new(csvmanager::new(headers)));
    // Handles for threads
    let mut handles = vec![];
    // Wind speed
    let handle = thread::spawn(
        move || {
            value = c::wind_speed::get_wind_speed();
            // Make sure to only keep the lock for as little as possible
            {
                let mut man = csvman.lock().unwrap();
                *man.give_data(Data::WindSpeed(value));
            }
            // 300 seconds is 5 minutes and it takes 10 seconds to measure so the rate stays consistent
            thread::sleep(time::Duration::from_secs(300-10));
        }
    );
    handles.push(handle);
    // BME280
    // Air Pressure
    // Humidity
    // Tempurature
    let handle = thread::spawn(
        move || {
            let command = Command::new("python ../python/BME280.py")
                .output()
                .expect("Failed to run command");
            let output = String::from_utf8_lossy(&command.stdout);
            {
                let mut man = csvman.lock().unwrap();
                // *man.give_data(Data::AirPressure(0.0));
            }
            // Wait for 5 minutes
            thread::sleep(time::Duration::from_secs(300));
        }
    );
    handles.push(handle);
    for handle in handles {
        handle.join().unwrap();
    }
}

#[cfg(not(feature = "rust_only"))]
fn c_tests() {
    #[cfg(debug_assertions)]
    {
        // println!("{}", c::add::add(1,2));
        println!("C time: {}", c::ctime::get_c_time());
    }
}
