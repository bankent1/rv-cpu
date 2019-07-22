mod hardware;
mod instruction;
mod phases;
mod single_cycle;
mod control_bits;
mod demos;
mod tools;

use demos::demo1;

fn main() {
    println!("Hello, cpu!");

    demo1::start(false);
}
