/*
 * reg_file.rs
 * 
 * Author: Travis Banken
 * 
 * Simulated Register file, which allows for loading and storing of data into
 * registers.
 * 
 * Supported Registers:
 *  $0 - $zero      $10 - $t2       $20 - $s4       $30 - $fp
 *  $1 - $at        $11 - $t3       $21 - $s5       $31 - $ra
 *  $2 - $v0        $12 - $t4       $22 - $s6
 *  $3 - $v1        $13 - $t5       $23 - $s7
 *  $4 - $a0        $14 - $t6       $24 - $t8
 *  $5 - $a1        $15 - $t7       $25 - $t9
 *  $6 - $a2        $16 - $s0       $26 - $k0
 *  $7 - $a3        $17 - $s1       $27 - $k1
 *  $8 - $t0        $18 - $s2       $28 - $gp
 *  $9 - $t1        $19 - $s3       $29 - $sp
 */

#![allow(dead_code)]

struct Registers {
    registers: [u32; 32]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: [0; 32]
        }
    }

    /*
     * Loads value from register.
     */
    pub fn load(&mut self, reg_num: usize) -> u32 {
        self.registers[reg_num]
    }

    /*
     * Writes value into register.
     */
    pub fn write(&mut self, value: u32, reg_num: usize) {
        self.registers[reg_num] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_and_load() {
        let mut regfile = Registers::new();
        regfile.write(2, 3);
        let mut r3 = regfile.load(3);
        assert_eq!(r3, 2);
        regfile.write(1, 3);
        r3 = regfile.load(3);
        assert_ne!(r3, 2);
    }
}