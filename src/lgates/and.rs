/*
 * and.rs
 *
 * Author: Travis Banken
 *
 * Simulates an AND-gate
 */

pub struct And(pub u8, pub u8);

impl And {
    pub fn execute(self) -> u8 {
        self.0 & self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte() {
        let and1 = And(0b1, 0b1);
        assert_eq!(and1.execute(), 0b1 & 0b1);

        let and2 = And(0b0, 0b1);
        assert_eq!(and2.execute(), 0b0 & 0b1);

        let and3 = And(0b0, 0b0);
        assert_eq!(and3.execute(), 0b0 & 0b0);
    }
}
