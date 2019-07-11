/*
 * not.rs
 * 
 * Author: Travis Banken
 * 
 * Simulates a logical NOT-gate
 */

pub struct Not(u8);

impl Not {
    pub fn execute(self) -> u8 {
        !self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte() {
        let not1 = Not(0b1);
        assert_eq!(not1.execute(), !0b1);

        let not2 = Not(0b0);
        assert_eq!(not2.execute(), !0b0);
    }
}