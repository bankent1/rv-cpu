/*
 * or.rs
 *
 * Author: Travis Banken
 *
 * Simulates a single byte OR-gate
 */

pub struct Or(pub u8, pub u8);

impl Or {
    pub fn execute(self) -> u8 {
        self.0 | self.1
    }
}

// Unit Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_eq() {
        let or1 = Or(0b1, 0b1);
        assert_eq!(or1.execute(), 0b1 | 0b1);

        let or2 = Or(0b0, 0b1);
        assert_eq!(or2.execute(), 0b0 | 0b1);

        let or3 = Or(0b0, 0b0);
        assert_eq!(or3.execute(), 0b0 | 0b0);

    }
}
