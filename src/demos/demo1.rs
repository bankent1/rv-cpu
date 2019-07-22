/*
 * demo1.rs
 * 
 * Author: Travis Banken
 * 
 * Writes 0xcafebabe into address 0x42
 */
#![allow(dead_code)]

use crate::demos::assembler::reg_macros::*;
use crate::demos::assembler::instr_macros::*;
use crate::demos::assembler::mem_loader::MemLoader;
use crate::hardware::instr_mem;
use crate::hardware::data_mem;
use crate::single_cycle;
use crate::tools::dump_instr_mem;
use crate::tools::dump_data_mem;

pub fn start(debug: bool) {

    let mut instr_mem = instr_mem::Memory::new();
    let mut data_mem = data_mem::Memory::new();
    instr_mem = load_instr(instr_mem);
    dump_instr_mem::dump_as_txt(&instr_mem);

    single_cycle::start(&instr_mem, &mut data_mem, debug);
    dump_data_mem::dump_as_txt(&data_mem);
}

fn load_instr(mem: instr_mem::Memory) -> instr_mem::Memory {
    let mut loader = MemLoader::new(mem);

    loader.load_instr( ADDI(T0(), ZERO(), 0xbabe) );
    loader.load_instr( LUI (T1(),         0xcafe) );
    loader.load_instr( OR  (T0(), T1()  , T0()  ) );
    loader.load_instr( ADDI(S0(), ZERO(), 0x42  ) );
    loader.load_instr( SW  (T0(), 0     , S0()  ) );

    return loader.return_mem();
}