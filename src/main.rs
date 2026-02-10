mod c;
mod io;

// This program is optimized for battery usage and will poll instruments every hour and should take
// Battery life into account
// Writing to files will not use buffered writes as every hour makes them worthless
fn main() {
    println!("Hello, world!");
    println!("{}", c::add::add(1,2));
}
