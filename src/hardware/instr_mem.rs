/*
 * instr_mem.rs
 * 
 * Author: Travis Banken
 * 
 * Instruction memory for the processor.
 * 
 */
#![allow(dead_code)]

const MEM_SIZE: usize = 256;

pub struct Memory {
    mem: [u8; MEM_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; MEM_SIZE]
        }
    }

    /*
     * Read a byte from memory. 
     * 
     * Will panic if address is outside size of memory.
     */
    pub fn read(&self, addr: usize) -> u8 {
        if addr > MEM_SIZE {
            panic!("Mem-Error (Instruction): Tried to access
                     non-existant memory: 0x{:x}", addr);
        }
        self.mem[addr]
    }

    /*
     * Write a byte into memory. 
     * 
     * Will panic if address is outside size of memory.
     */
    pub fn write(&mut self, value: u8, addr: usize) {
        if addr > MEM_SIZE {
            panic!("Mem-Error (Instruction): Tried to access
                     non-existant memory: 0x{:x}", addr);
        }
        self.mem[addr] = value;
    }

    /*
     * get size of memory
     */
    pub fn get_size() -> usize {
        MEM_SIZE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let mut mem = Memory::new();
        mem.write(23, 0x12);
        let mut val = mem.read(0x12);
        assert_eq!(val, 23);

        mem.write(43, 0x12);
        val = mem.read(0x12);
        assert_ne!(val, 23);
    }

    #[test]
    fn test_get_size() {
        assert_eq!(Memory::get_size(), MEM_SIZE);
    }

    #[test]
    #[should_panic]
    fn test_read_panic() {
        let mem = Memory::new();
        mem.read(0xdeadbeef);
    }

    #[test]
    #[should_panic]
    fn test_write_panic() {
        let mut mem = Memory::new();
        mem.write(34, 0xdeadbeef);
    }
}