// C functions
#[cfg(not(feature = "rust_only"))]
mod c;
// Stuff for csvs
mod io;
// Used for multithreading
use std::{thread, time};

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
    let headers: Vec<String> = vec![String::from("Temp"), String::from("Coolness")];
    let body: Vec<Vec<String>> = vec![
        vec![String::from("Nice"), String::from("and Cool")],
        vec![String::from("Really"), String::from("Cool")],
    ];
    let mut mycsv = crate::io::csv::csv::new(headers, body);
    let _ = mycsv.save_to_file("data/my.csv");
    let _ = mycsv.write_new_row(
        "data/my.csv",
        vec![String::from("really"), String::from("really")],
    );
    let headers: Vec<String> = vec![
        String::from("Temp"),
        String::from("Hum"),
        String::from("Speed"),
    ];
    let mut csvman = csvmanager::new(headers);
    csvman.give_data(Data::Tempurature(100.1));
    csvman.give_data(Data::Humidity(100.1));
    csvman.give_data(Data::WindSpeed(100.1));
    #[cfg(not(feature = "rust_only"))]
    {
        c_tests()
    }
    #[cfg(not(feature = "no_pi"))]
    {
        wiring_pi();
    }
}

#[cfg(not(feature = "no_pi"))]
fn wiring_pi() {
    c::dht11::setup_wiring_pi();
    loop {
        thread::sleep(time::Duration::from_millis(2000));
        c::dht11::read_dht11_dat();
        c::py::print_temp_humidity();
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
