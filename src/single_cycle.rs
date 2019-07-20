/*
 * single_cycle.rs
 * 
 * Author: Travis Banken
 * 
 * Completes a single cycle on the cpu
 */
#![allow(dead_code)]

use crate::hardware::*;
use crate::phases::*;
use crate::instruction::Instruction;
use crate::control_bits::ControlBits;

pub fn start(debug: bool) {
    if debug {
        println!("Debug Mode: ON");
    }

    let instr_mem = instr_mem::Memory::new();
    let regfile = reg_file::Registers::new();
    let data_mem = data_mem::Memory::new();

    let mem_size = instr_mem::Memory::get_size();
    let ip = 0;
    while ip < mem_size {
        // TODO: Execute instructions

        // TODO: Fetch instruction
        let instr_raw = instr_fetch(&instr_mem, ip);

        // TODO: decode instruction
        //  - decode instruction
        //  - fill control bits
        let mut instr_struct = Instruction::default();
        instr_decode(instr_raw, &mut instr_struct);
        
        let mut ctrl_bits = ControlBits::default();
        fill_control_bits(&mut ctrl_bits, &instr_struct);


        // TODO: execute alu
        //  - get alu input 1
        //  - get alu input 2
        //  - get alu op
        //  - execute
        //  - not result?
        let alu_in1 = get_alu_in1(&regfile, &instr_struct);
        let alu_in2 = get_alu_in2(&regfile, &instr_struct, &ctrl_bits);
        
        let alu_res = execute_alu(ctrl_bits.alu_op, alu_in1, alu_in2, ctrl_bits.alu_bnegate);

        // not the res of alu if ctrl bit is on
        let alu_res = if ctrl_bits.not_res == 1 {!alu_res & 0x1} else {alu_res};


        // TODO: mem phase
        let write_val = regfile.load(instr_struct.rt as usize);
        let wbval = match mem_phase(&ctrl_bits, &data_mem, alu_res as usize, write_val) {
            Some(res) => res,
            None => 0
        };

        // TODO: write back phase
        //  - determine wbval
        //  - determine reg num
        let wbval = if ctrl_bits.mem_to_reg == 1 {wbval} else {alu_res};
        let reg_num = if ctrl_bits.reg_dst == 1 {instr_struct.rd} else {instr_struct.rt};
        write_back(&regfile, reg_num as usize, &ctrl_bits, wbval);
    }
}

fn get_alu_in1(regfile: &reg_file::Registers, instr: &Instruction) -> u32 {
    let reg_num = instr.rs as usize;
    return regfile.load(reg_num);
}

fn get_alu_in2(regfile: &reg_file::Registers, instr: &Instruction, ctrl: &ControlBits) -> u32 {
    // TODO: support unsigned imm
    if ctrl.reg_dst == 1 {
        return instr.imm16 as u32;
    } else {
        return regfile.load(instr.rt as usize);
    }
}

fn fill_control_bits(ctrl: &mut ControlBits, instr: &Instruction) {
    match instr.opcode {
        0x00 => { // r-format
            ctrl.reg_dst = 1;
            ctrl.reg_write = 1;
            ctrl.branch = 0;
            ctrl.jump = 0;
            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;
            match instr.funct {
                0x00 => panic!("Error: Unsupported FUNCT [0x00 (sll)"), // sll
                0x02 => panic!("Error: Unsupported FUNCT [0x02 (srl)"), // srl
                0x03 => panic!("Error: Unsupported FUNCT [0x03 (sra)"), // sra
                0x08 => panic!("Error: Unsupported FUNCT [0x08 (jr)]"), // jr
                0x10 => panic!("Error: Unsupported FUNCT [0x10 (mfhi)"), // mfhi
                0x11 => panic!("Error: Unsupported FUNCT [0x11 (mthi)"), // mthi
                0x12 => panic!("Error: Unsupported FUNCT [0x12 (mflo)"), // mflo
                0x13 => panic!("Error: Unsupported FUNCT [0x13 (mtlo)"), // mtlo
                0x18 => panic!("Error: Unsupported FUNCT [0x18 (mult)"), // mult
                0x19 => panic!("Error: Unsupported FUNCT [0x19 (multu)"), // multu
                0x1A => panic!("Error: Unsupported FUNCT [0x1A (div)]"), // div
                0x1B => panic!("Error: Unsupported FUNCT [0x1B (divu)]"), // divu
                0x20 => { // add
                    ctrl.alu_op = 2;
                    ctrl.alu_bnegate = 0;
                    ctrl.not_res = 0;
                },
                0x21 => { // addu
                    ctrl.alu_op = 2;
                    ctrl.alu_bnegate = 0;
                    ctrl.not_res = 0;
                },
                0x22 => { // sub
                    ctrl.alu_op = 2;
                    ctrl.alu_bnegate = 1;
                    ctrl.not_res = 0;
                },
                0x23 => { // subu
                    ctrl.alu_op = 2;
                    ctrl.alu_bnegate = 1;
                    ctrl.not_res = 0;
                },
                0x24 => { // and
                    ctrl.alu_op = 0;
                    ctrl.alu_bnegate = 0;
                    ctrl.not_res = 0;
                },
                0x25 => { // or
                    ctrl.alu_op = 1;
                    ctrl.alu_bnegate = 0;
                    ctrl.not_res = 0;
                },
                0x26 => { // xor
                    ctrl.alu_op = 4;
                    ctrl.alu_bnegate = 0;
                    ctrl.not_res = 0;
                },
                0x27 => { // nor
                    ctrl.alu_op = 1;
                    ctrl.alu_bnegate = 0;
                    ctrl.not_res = 1;
                },
                0x2A => { // slt
                    ctrl.alu_op = 3;
                    ctrl.alu_bnegate = 1;
                    ctrl.not_res = 0;
                },
                0x2B => { // sltu
                    ctrl.alu_op = 3;
                    ctrl.alu_bnegate = 1;
                    ctrl.not_res = 0;
                },
                funct => panic!("Error: Unsupported FUNCT [{:X}]", funct),
            };
        },
        0x02 => { // j
            ctrl.reg_dst = 0;
            ctrl.reg_write = 0;

            ctrl.branch = 0;
            ctrl.jump = 1;

            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 0; // TODO: alu op for jump
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        0x03 => panic!("Error: Unsupported OPCODE [0x03 (jal)]"),
        0x04 => { // beq
            ctrl.reg_dst = 0;
            ctrl.reg_write = 0;

            ctrl.branch = 1;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 4;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 1;
        },
        0x05 => { // bne
            ctrl.reg_dst = 0;
            ctrl.reg_write = 0;

            ctrl.branch = 1;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 4;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        0x08 => { // addi
            ctrl.reg_dst = 0;
            ctrl.reg_write = 1;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 2;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        0x09 => { // addiu
            ctrl.reg_dst = 0;
            ctrl.reg_write = 1;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 2;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        0x0A => { // slti
            ctrl.reg_dst = 1;
            ctrl.reg_write = 1;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 3;
            ctrl.alu_bnegate = 1;
            ctrl.not_res = 0;
        },
        0x0B => { // sltiu
            ctrl.reg_dst = 1;
            ctrl.reg_write = 1;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 3;
            ctrl.alu_bnegate = 1;
            ctrl.not_res = 0;
        },
        0x0C => { // andi
            ctrl.reg_dst = 0;
            ctrl.reg_write = 1;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 0;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        0x0D => { // ori
            ctrl.reg_dst = 0;
            ctrl.reg_write = 1;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 1;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        0x0F => panic!("Error: Unsupported OPCODE [0x0F (lui)]"),
        0x20 => { // lb
            ctrl.reg_dst = 0;
            ctrl.reg_write = 1;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 1;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 1;
            ctrl.mem_by_byte = 1;

            ctrl.alu_op = 2;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        0x23 => { // lw
            ctrl.reg_dst = 0;
            ctrl.reg_write = 1;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 1;
            ctrl.mem_write = 0;
            ctrl.mem_to_reg = 1;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 2;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        0x28 => { // sb
            ctrl.reg_dst = 0;
            ctrl.reg_write = 0;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 1;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 1;

            ctrl.alu_op = 2;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        0x2B => { // sw
            ctrl.reg_dst = 0;
            ctrl.reg_write = 0;

            ctrl.branch = 0;
            ctrl.jump = 0;

            ctrl.mem_read = 0;
            ctrl.mem_write = 1;
            ctrl.mem_to_reg = 0;
            ctrl.mem_by_byte = 0;

            ctrl.alu_op = 2;
            ctrl.alu_bnegate = 0;
            ctrl.not_res = 0;
        },
        opcode => panic!("Error: Unsupported OPCODE [{:X}]", opcode)
    };
}
