/*
 * nor.rs
 * 
 * Author: Travis Banken
 * 
 * Simulates a NOR logic gate
 */

pub struct Nor(u8, u8);

impl Nor {
    pub fn execute(self) -> u8 {
        !(self.0 | self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nor_byte() {
        let nor1 = Nor(0b1, 0b1);
        assert_eq!(nor1.execute(), !(0b1 | 0b1));

        let nor2 = Nor(0b0, 0b1);
        assert_eq!(nor2.execute(), !(0b0 | 0b1));

        let nor3 = Nor(0b0, 0b0);
        assert_eq!(nor3.execute(), !(0b0 | 0b0));
    }
}