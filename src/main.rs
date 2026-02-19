mod c;
mod io;
// use dht_mmap_rust::{Dht, DhtType};
use std::{thread, time};
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
    let mut mycsv = io::csv::csv::new(headers, body);
    let _ = mycsv.save_to_file("data/my.csv");
    let _ = mycsv.write_new_row(
        "data/my.csv",
        vec![String::from("really"), String::from("really")],
    );
    // let pi = wiringpi::setup();
    // let pin = pi.input_pin(6);
    // loop {
    //     thread::sleep(time::Duration::from_millis(2000));
    //     let value = pin.digital_read();
    //     match value {
    //         High => {println!("High");},
    //         Low => {println!("Low");},
    //     }
    // }
    // The sensor is a DHT11 connected on pin 23
    // let mut dht = get_dht(11);

    // Important: DHT sensor reads fail sometimes. In an actual program, if a read fails you should retry multiple times until
    // the read succeeds.
    // For more information, see documentation on `read()`
    // loop {
        // let reading = dht.read().expect("Failed to read dht");

        // println!(
        //     "Temperature {} °C, Humidity {}%RH",
        //     reading.temperature(),
        //     reading.humidity()
        // );
    // }
    // println!("Hello, world!");
    // println!("{}", c::add::add(1,2));
    c::dht11::setup_wiring_pi();
    loop {
        thread::sleep(time::Duration::from_millis(2000));
        c::dht11::read_dht11_dat();
    }
}

// fn get_dht(pin: usize) -> Dht {
//     match Dht::new(DhtType::Dht11, pin) {
//         Ok(v) => v,
//         Err(_e) => {
//             println!("Error accessing dht");
//             thread::sleep(time::Duration::from_millis(1000));
//             get_dht(pin)
//         }
//     }
// }
