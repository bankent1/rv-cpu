/*
 * xor.rs
 *
 * Author: Travis Banken
 * 
 * Simulates a single byte XOR gate 
 */

pub struct Xor(u8, u8);

impl Xor {
    pub fn execute(self) -> u8 {
        self.0 ^ self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xor_byte() {
        let xor1 = Xor(0b1, 0b1);
        assert_eq!(xor1.execute(), 0b1 ^ 0b1);
        
        let xor2 = Xor(0b0, 0b1);
        assert_eq!(xor2.execute(), 0b0 ^ 0b1);

        let xor3 = Xor(0b0, 0b0);
        assert_eq!(xor3.execute(), 0b0 ^ 0b0);
    }
}