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

/*
 * Executes the given operation on the given inputs.
 */
pub fn execute_alu(alu_op: u8, alu_in1: u32, alu_in2: u32) -> u32 {
    let alu = alu::Alu::new(alu_in1, alu_in2);
    match alu_op {
        0 => alu.and(),
        1 => alu.or(),
        2 => alu.add(),
        3 => alu.sub(),
        4 => alu.less(),
        5 => alu.xor(),
        _ => 0 // NOP
    }
}

/*
 * Writes the given 32-bit val into main mem
 * 
 * TODO: Revisit for mem-read
 */
pub fn mem_phase(mem: &mut data_mem::Memory, addr: usize, alu_res: u32) {
    let write0 = (alu_res >> 24) as u8;
    let write1 = (alu_res >> 16) as u8;
    let write2 = (alu_res >> 8) as u8;
    let write3 = (alu_res >> 0) as u8;

    mem.write(write0, addr + 0);
    mem.write(write1, addr + 1);
    mem.write(write2, addr + 2);
    mem.write(write3, addr + 3);
}

/*
 * Write the given data into the given register
 * 
 * TODO: Revisit
 */
pub fn write_back(regfile: &mut reg_file::Registers, value: u32, reg: usize) {
    regfile.write(value, reg);
}

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

    #[test]
    fn test_execute_alu() {
        // and
        let mut res = execute_alu(0, 1, 0);
        assert_eq!(res, 0);

        // or
        res = execute_alu(1, 1, 0);
        assert_eq!(res, 1);

        // add
        res = execute_alu(2, 1, 1);
        assert_eq!(res, 2);

        // sub
        res = execute_alu(3, 3, 2);
        assert_eq!(res, 1);

        // less
        res = execute_alu(4, 3, 1);
        assert_eq!(res, 0);

        // xor
        res = execute_alu(5, 1, 1);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_mem_phase() {
        let mut mem = data_mem::Memory::new();
        mem_phase(&mut mem, 0, 0x12345678);

        assert_eq!(mem.read(0), 0x12);
        assert_eq!(mem.read(1), 0x34);
        assert_eq!(mem.read(2), 0x56);
        assert_eq!(mem.read(3), 0x78);
    }

    #[test]
    fn test_write_back() {
        let mut regfile = reg_file::Registers::new();
        write_back(&mut regfile, 1234, 23);

        assert_eq!(regfile.load(23), 1234);
    }
}