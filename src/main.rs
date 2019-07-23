mod hardware;
mod instruction;
mod phases;
mod single_cycle;
mod control_bits;
mod demos;
mod tools;

use demos::demo1;
use demos::demo2;
use std::env;
use std::process::exit;

fn main() {
    let argv: Vec<String> = env::args().collect();

    let mut debug: bool = false;
    let mut dump: bool = false;
    let mut demo: i32 = -1;
    for arg in argv.iter() {
        if arg == "--debug" {
            debug = true;
        } else if arg == "--dump" {
            dump = true;
        } else if arg == "--help" || arg == "-h" {
            show_help(0);
        } else if arg == "demo1" {
            println!("Demo:      demo1");
            demo = 1;
        } else if arg == "demo2" {
            println!("Demo:      demo2");
            demo = 2;
        }
    }

    match demo {
        1 => {
            demo1::start(debug, dump);
        },
        2 => {
            demo2::start(debug, dump);
        },
        _ => {
            eprintln!("Invalid demo name!");
            show_help(1);
        }
    };
    exit(0);
}

fn show_help(retval: i32) {
    println!("Usage: ");
    println!("      rvp [OPTIONS] <demo-name>");
    println!("OPTIONS:");
    println!("      --debug  Prints out debug information while the processor");
    println!("               runs");
    println!("      --dump   Dumps out the contents of instr mem and data mem");
    println!("               to stdout");
    println!("Demo Names:");
    println!("      demo1    Writes the value 0xcafebabe into address 0x42");
    println!("      demo2    Writes the value 0x42 into every address in mem");
    exit(retval);
}
