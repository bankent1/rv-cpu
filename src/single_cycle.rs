/*
 * single_cycle.rs
 * 
 * Author: Travis Banken
 * 
 * Simulates a single cycle cpu instruction cycle
 */
#![allow(dead_code)]

use super::hardware::*;
use crate::instruction::Instruction;

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

/*
 * Decode the given instruction and store the result into the Instruction struct
 * passed in.
 */
pub fn instr_decode(instr_raw: u32, instr_struct: &mut Instruction) {
    instr_struct.opcode = ((instr_raw >> 26) & 0x3f) as u8;
    instr_struct.rs = ((instr_raw >> 21) & 0x1f) as u8;
    instr_struct.rt = ((instr_raw >> 16) & 0x1f) as u8;

    // r-format
    instr_struct.rd = ((instr_raw >> 11) & 0x1f) as u8;
    instr_struct.shamt = ((instr_raw >> 6) & 0x1f) as u8;
    instr_struct.funct = ((instr_raw >> 0) & 0x3f) as u8;

    // i-format
    instr_struct.imm16 = ((instr_raw >> 0) & 0xffff) as u16;

    // j-format
    instr_struct.addr = (instr_raw >> 0) & 0x3f_ffff;
}

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

    #[test]
    fn test_instr_decode() {
        let mut instr = Instruction::default();
        instr_decode(0xffffffff, &mut instr);

        assert_eq!(instr.opcode, 0x3f);
        assert_eq!(instr.rs, 0x1f);
        assert_eq!(instr.rt, 0x1f);
        assert_eq!(instr.rd, 0x1f);
        assert_eq!(instr.shamt, 0x1f);
        assert_eq!(instr.funct, 0x3f);
        assert_eq!(instr.imm16, 0xffff);
        assert_eq!(instr.addr, 0x3f_ffff);
    }
}