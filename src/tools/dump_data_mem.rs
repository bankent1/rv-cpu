/*
 * dump_data_mem.rs
 * 
 * Author: Travis Banken
 * 
 * Dump the contents of data memory
 */
#![allow(dead_code)]

use crate::hardware::data_mem::Memory;

pub fn dump_as_txt(mem: &Memory) {
    let size = Memory::get_size();

    println!("======================================");
    println!("             Data Memory              ");
    println!("======================================");
    for addr in 0..size {
        if addr % 4 == 0 {
            println!();
        }
        print!("0x{:08x}: 0x{:02x}", addr, mem.read(addr));
        print!("  |  ");
    }
    println!("\n======================================\n");
}