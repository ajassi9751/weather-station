mod c;
mod io;

// This program is optimized for battery usage and will poll instruments every hour and should take
// Battery life into account
// Writing to files will not use buffered writes as every hour makes them worthless
fn main() {
    let headers: Vec<String> = vec![String::from("Temp"), String::from("Coolness")];
    let body: Vec<Vec<String>> = vec![vec![String::from("Nice"),String::from("and Cool")], vec![String::from("Really"), String::from("Cool")]];
    let mycsv = io::csv::csv {headers: headers, body: body};
    let _ = mycsv.save_to_file("my.csv");
    println!("Hello, world!");
    println!("{}", c::add::add(1,2));
}