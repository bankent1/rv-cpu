/*
 * phases.rs
 * 
 * Author: Travis Banken
 * 
 * Contains all of the phases of the cpu cycle
 */
#![allow(dead_code)]

use super::hardware::*;
use crate::instruction::Instruction;
use crate::control_bits::ControlBits;

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
    instr_struct.addr = (instr_raw >> 0) & 0x03ff_ffff;
}

/*
 * Executes the given operation on the given inputs.
 */
pub fn execute_alu(alu_op: u8, alu_in1: u32, alu_in2: u32, bnegate: u8) -> u32 {
    let in2 = if bnegate == 1 {(!alu_in2).overflowing_add(1).0} else {alu_in2};
    let alu = alu::Alu::new(alu_in1, in2);
    match alu_op {
        0 => alu.and(),
        1 => alu.or(),
        2 => alu.add(),
        3 => alu.less(),
        4 => alu.xor(),
        op => panic!("Error: Alu op [{}] not supported!", op)
    }
}

/*
 * Based on the given control bits, the mem phase will read/write mem or do 
 * nothing.
 */
pub fn mem_phase(ctrl: &ControlBits, mem: &mut data_mem::Memory, addr: usize, write_val: u32) -> Option<u32> {
    if ctrl.mem_read == 1 {
        // read by byte or word
        if ctrl.mem_by_byte == 1 {
            return Some(mem.read(addr) as u32);
        } else {
            return Some(read_word(&mem, addr));
        }
    } else if ctrl.mem_write == 1 {
        // write by byte or word
        if ctrl.mem_by_byte == 1 {
            mem.write(write_val as u8, addr);
        } else {
            write_word(mem, write_val, addr);
        }
        return None;
    }
    return None;
}

/*
 * Write the given data into the given register
 * 
 * TODO: Revisit
 */
pub fn write_back(regfile: &mut reg_file::Registers, reg_num: usize, ctrl: &ControlBits, wbval: u32) {
    if ctrl.reg_write == 1 {
        regfile.write(wbval, reg_num);
    }
}

// *** PRIVATE FN ***

fn write_word(mem: &mut data_mem::Memory, val: u32, addr: usize) {
    let write0 = (val >> 24) as u8;
    let write1 = (val >> 16) as u8;
    let write2 = (val >> 8) as u8;
    let write3 = (val >> 0) as u8;

    mem.write(write0, addr + 0);
    mem.write(write1, addr + 1);
    mem.write(write2, addr + 2);
    mem.write(write3, addr + 3);
}

fn read_word(mem: &data_mem::Memory, addr: usize) -> u32 {
    let read0 = mem.read(addr + 0) as u32;
    let read1 = mem.read(addr + 1) as u32;
    let read2 = mem.read(addr + 2) as u32;
    let read3 = mem.read(addr + 3) as u32;

    return (read0 << 24) & (read1 << 16) & (read2 << 8) & (read3 << 0);
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
        assert_eq!(instr.addr, 0x03ff_ffff);
    }

    #[test]
    fn test_execute_alu() {
        // and
        let mut res = execute_alu(0, 1, 0, 0);
        assert_eq!(res, 0);

        // or
        res = execute_alu(1, 1, 0, 0);
        assert_eq!(res, 1);

        // add
        res = execute_alu(2, 1, 1, 0);
        assert_eq!(res, 2);

        // sub
        res = execute_alu(2, 3, 2, 1);
        assert_eq!(res, 1);

        // less
        res = execute_alu(3, 3, 1, 1);
        assert_eq!(res, 0);

        // xor
        res = execute_alu(4, 1, 1, 0);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_mem_phase() {
        // TODO: Write tests
    }

    #[test]
    fn test_write_back() {
        // TODO: Write tests
    }
}