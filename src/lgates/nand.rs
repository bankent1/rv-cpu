/*
 * nand.rs
 * 
 * Author: Travis Banken
 * 
 * Simulates a logical NAND-gate
 */

pub struct Nand(u8, u8);

impl Nand {
    pub fn execute(self) -> u8 {
        !(self.0 & self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte() {
        let nand1 = Nand(0b1, 0b1);
        assert_eq!(nand1.execute(), !(0b1 & 0b1));

        let nand2 = Nand(0b0, 0b1);
        assert_eq!(nand2.execute(), !(0b0 & 0b1));

        let nand3 = Nand(0b0, 0b0);
        assert_eq!(nand3.execute(), !(0b0 & 0b0));
    }
}