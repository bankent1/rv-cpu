/*
 * demo2.rs
 * 
 * Author: Travis Banken
 * 
 * writes 42 in the first 256 bytes in memory
 */
#![allow(dead_code)]
#![allow(non_snake_case)]

use crate::demos::assembler::reg_macros::*;
use crate::demos::assembler::instr_macros::*;
use crate::demos::assembler::mem_loader::MemLoader;
use crate::hardware::instr_mem;
use crate::hardware::data_mem;
use crate::single_cycle;
use crate::tools::dump_instr_mem;
use crate::tools::dump_data_mem;

pub fn start(debug: bool, mem_dump: bool) {
    println!("Runnning Demo 2...");
    let mut instr_mem = instr_mem::Memory::new();
    let mut data_mem = data_mem::Memory::new();

    instr_mem = load_instr(instr_mem);
    if mem_dump {
        dump_instr_mem::dump_as_txt(&instr_mem);
    }

    single_cycle::start(&instr_mem, &mut data_mem, debug);
    if mem_dump {
        dump_data_mem::dump_as_txt(&data_mem);
    }
    println!("Done!");
}

fn load_instr(mem: instr_mem::Memory) -> instr_mem::Memory {
    let mut loader = MemLoader::new(mem);

    // s0 = *mem
    loader.load_instr( ADD (S0(), ZERO(), ZERO()) );
    loader.load_instr( ADDI(S7(), ZERO(), 256)    );
    loader.load_instr( ADDI(T1(), ZERO(), 0x42)   );

    let LOOP = loader.get_ip() as u32;
// LOOP:
    loader.load_instr( SLT (T0(), S0()  , S7())   );
    loader.load_instr( BEQ (T0(), ZERO(), (LOOP + 5*4) as u16)); // j END_LOOP

    loader.load_instr( SB  (T1(), 0     , S0())   );
    loader.load_instr( ADDI(S0(), S0()  , 1)      );

    loader.load_instr( J   (LOOP)                  ); // j LOOP
// END_LOOP:

    return loader.return_mem();
}