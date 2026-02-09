mod c;

fn main() {
    println!("Hello, world!");
    unsafe {
        println!("{}", c::add::add(1,2));
    }
}
