/*
 * data_mem.rs
 * 
 * Author: Travis Banken
 * 
 * Memory unit for data
 */
#![allow(dead_code)]

const MEM_SIZE: usize = 256;

struct Memory {
    mem: [u8; MEM_SIZE]
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            mem: [0; MEM_SIZE]
        }
    }

    /*
     * Reads a value from the given address.
     * 
     * Will panic if address does not exist.
     */
    pub fn read(&mut self, addr: usize) -> u8 {
        if addr > MEM_SIZE {
            let addr_hex = format!("{:x}", addr);
            panic!("Mem-Error: Tried to access non-existant address: 0x{}", addr_hex);
        }
        self.mem[addr]
    }

    /*
     * Writes value into address. 
     * 
     * Will panic if address does not exist.
     */
    pub fn write(&mut self, value: u8, addr: usize) {
        if addr > MEM_SIZE {
            let addr_hex = format!("{:x}", addr);
            panic!("Mem-Error: Tried to access non-existant address: 0x{}", addr_hex);
        }
        self.mem[addr] = value;
    }

    /*
     * Return size of memory
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
        let mut mem = Memory::new();
        mem.read(0xdeadbeef);
    }

    #[test]
    #[should_panic]
    fn test_write_panic() {
        let mut mem = Memory::new();
        mem.write(34, 0xdeadbeef);
    }
}

