/*
 * dump_instr_mem.rs
 * 
 * Author: Travis Banken
 * 
 * Dump the contents of instr memory
 */
#![allow(dead_code)]

use crate::hardware::instr_mem::Memory;

pub fn dump_as_txt(mem: &Memory) {
    let size = Memory::get_size();

    println!("======================================");
    println!("          Instruction Memory          ");
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