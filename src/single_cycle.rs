/*
 * single_cycle.rs
 * 
 * Author: Travis Banken
 * 
 * Simulates a single cycle cpu instruction cycle
 */
#![allow(dead_code)]

use super::hardware::*;

/*
 * Fetch the next instruction from memory
 */
pub fn instr_fetch(mem: &instr_mem::Memory, addr: usize) -> u32 {
    let read0 = mem.read(addr + 0) as u32;
    let read1 = mem.read(addr + 1) as u32;
    let read2 = mem.read(addr + 2) as u32;
    let read3 = mem.read(addr + 3) as u32;

    return read0 << 24 | read1 << 16 | read2 << 8 | read3 << 0;
}

// TODO: Instruction decode
// pub fn instr_decode(instr: u32, )

// TODO: Execute

// TODO: Memory

// TODO: Write Back

#[cfg(test)]
mod tests {
    use super::*;

    fn test_instr_fetch() {
        let mut mem = instr_mem::Memory::new();

        mem.write(0x12, 0);
        mem.write(0x34, 1);
        mem.write(0x56, 2);
        mem.write(0x78, 3);

        assert_eq!(instr_fetch(&mem, 0), 0x12345678);
    }
}