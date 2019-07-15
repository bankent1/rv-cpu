/*
 * instruction.rs
 * 
 * Author: Travis Banken
 * 
 * Structure for holding decoded instruction
 */
#![allow(dead_code)]

#[derive(Debug)]
pub struct Instruction {
    pub opcode: u8,
    pub rs: u8,
    pub rt: u8,

    // r-format
    pub rd: u8,
    pub shamt: u8,
    pub funct: u8,

    // i-format
    pub imm16: u16,

    // j-format
    pub addr: u32
}

impl Default for Instruction {
    fn default() -> Instruction {
        Instruction {
            opcode: 0,
            rs: 0,
            rt: 0,
            rd: 0,
            shamt: 0,
            funct: 0,
            imm16: 0,
            addr: 0 
        }
    }
}