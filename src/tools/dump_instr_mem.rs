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
        println!("0x{:08x}:\t0x{:02x}", addr, mem.read(addr));
    }
    println!("======================================\n");
}