/*
 * memloader.rs
 * 
 * Author: Travis Banken
 * 
 * Helper functions to write instructions into memory
 */
#![allow(dead_code)]

use crate::hardware::instr_mem::Memory;

pub struct MemLoader {
    mem: Memory,
    addr: usize
}

impl MemLoader {
    pub fn new(memory: Memory) -> MemLoader {
        MemLoader {
            mem: memory,
            addr: 0
        }
    }

    pub fn load_instr(&mut self, instr: u32) {
        let write0 = (instr >> 24) as u8; 
        let write1 = (instr >> 16) as u8; 
        let write2 = (instr >> 8) as u8; 
        let write3 = (instr >> 0) as u8; 

        self.mem.write(write0, self.addr + 0);
        self.mem.write(write1, self.addr + 1);
        self.mem.write(write2, self.addr + 2);
        self.mem.write(write3, self.addr + 3);
        
        self.addr += 4;
    }

    pub fn return_mem(self) -> Memory {
        let mut new_mem = Memory::new();
        let size = Memory::get_size();

        for i in 0..size {
            new_mem.write(self.mem.read(i), i);
        }
        return new_mem;
    }

    pub fn get_ip(&self) -> usize {
        return self.addr;
    }
} 