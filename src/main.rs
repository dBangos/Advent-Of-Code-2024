use std::time::Instant;

mod dayone;
mod daythree;
mod daytwo;
fn main() {
    let start = Instant::now();
    daythree::daythree();
    let duration = start.elapsed();
    println!("Time elapsed {:?}", duration);
}
