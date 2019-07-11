/*
 * alu.rs
 * 
 * Author: Travis Banken
 * 
 * Simulates an ALU.
 * 
 * Supported Operations:
 *  0) AND
 *  1) OR
 *  2) ADD
 *  3) SUB
 *  4) LESS
 *  5) XOR
 */

use u32;

struct Alu(u32, u32);

impl Alu {
    pub fn new(in1: u32, in2: u32) -> Alu {
        Alu(in1, in2)
    }

    pub fn and(self) -> u32 {
        self.0 & self.1
    }

    pub fn or(self) -> u32 {
        self.0 | self.1
    }

    pub fn add(self) -> u32 {
        self.0.overflowing_add(self.1).0
    }
    
    pub fn sub(self) -> u32 {
        self.0.overflowing_sub(self.1).0
    }

    pub fn less(self) -> u32 {
        self.sub() >> 31 
    }

    pub fn xor(self) -> u32 {
        self.0 ^ self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::u32;

    #[test]
    fn test_and() {
        let alu1 = Alu::new(0b1, 0b1);
        assert_eq!(alu1.and(), 0b1 & 0b1);

        let alu2 = Alu::new(0b0, 0b1);
        assert_eq!(alu2.and(), 0b0 & 0b1);

        let alu3 = Alu::new(0b0, 0b0);
        assert_eq!(alu3.and(), 0b0 & 0b0);

        let alu_big = Alu::new(3452, 23555);
        assert_eq!(alu_big.and(), 3452 & 23555);
    }

    #[test]
    fn test_or() {
        let alu1 = Alu::new(0b1, 0b1);
        assert_eq!(alu1.or(), 0b1 | 0b1);

        let alu2 = Alu::new(0b0, 0b1);
        assert_eq!(alu2.or(), 0b0 | 0b1);

        let alu3 = Alu::new(0b0, 0b0);
        assert_eq!(alu3.or(), 0b0 | 0b0);

        let alu_big = Alu::new(3452, 23555);
        assert_eq!(alu_big.or(), 3452 | 23555);
    }

    #[test]
    fn test_add() {
        let alu1 = Alu::new(0b1, 0b1);
        assert_eq!(alu1.add(), 0b1 + 0b1);

        let alu2 = Alu::new(0b0, 0b1);
        assert_eq!(alu2.add(), 0b0 + 0b1);

        let alu3 = Alu::new(0b0, 0b0);
        assert_eq!(alu3.add(), 0b0 + 0b0);

        let alu_big = Alu::new(3452, 23555);
        assert_eq!(alu_big.add(), 3452 + 23555);

        //overflow
        let alu_ovrflw = Alu::new(u32::MAX, u32::MAX);
        assert_eq!(alu_ovrflw.add(), u32::MAX.overflowing_add(u32::MAX).0)
    }

    #[test]
    fn test_sub() {
        let alu1 = Alu::new(0b1, 0b1);
        assert_eq!(alu1.sub(), 0b1 - 0b1);

        let alu2 = Alu::new(0b1, 0b0);
        assert_eq!(alu2.sub(), 0b1 - 0b0);

        let alu3 = Alu::new(0b0, 0b1);
        assert_eq!(alu3.sub(), (0b0 - 0b1) as u32);

        let alu4 = Alu::new(0b0, 0b0);
        assert_eq!(alu4.sub(), 0b0 - 0b0);

        let alu_big = Alu::new(3452, 23555);
        assert_eq!(alu_big.sub(), (3452 - 23555) as u32);
    }

    #[test]
    fn test_less() {
        let alu1 = Alu::new(0b1, 0b1);
        assert_eq!(alu1.less(), 0);

        let alu2 = Alu::new(0b1, 0b0);
        assert_eq!(alu2.less(), 0);

        let alu3 = Alu::new(0b0, 0b1);
        assert_eq!(alu3.less(), 1);

        let alu4 = Alu::new(0b0, 0b0);
        assert_eq!(alu4.less(), 0);

        let alu_big1 = Alu::new(1234, 23);
        assert_eq!(alu_big1.less(), 0);

        let alu_big2 = Alu::new(1234, 12223);
        assert_eq!(alu_big2.less(), 1);
    }

    #[test]
    fn test_xor() {
        let alu1 = Alu::new(0b1, 0b1);
        assert_eq!(alu1.xor(), 0b1 ^ 0b1);

        let alu2 = Alu::new(0b0, 0b1);
        assert_eq!(alu2.xor(), 0b0 ^ 0b1);

        let alu3 = Alu::new(0b0, 0b0);
        assert_eq!(alu3.xor(), 0b0 ^ 0b0);

        let alu_big = Alu::new(3452, 23555);
        assert_eq!(alu_big.xor(), 3452 ^ 23555);
    }
}