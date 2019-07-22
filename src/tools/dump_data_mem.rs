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
        println!("0x{:08x}:\t0x{:02x}", addr, mem.read(addr));
    }
    println!("======================================\n");
}