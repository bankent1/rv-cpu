/*
 * instr_macros.rs
 * 
 * Author: Travis Banken
 * 
 * Macros and functions for writing instructions
 */
#![allow(dead_code)]
#![allow(non_snake_case)]

// r-format

pub fn ADD(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x20;

    return opcode | rs | rt | rd | shamt | funct;
}

pub fn ADDU(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x21;

    return opcode | rs | rt | rd | shamt | funct;
}

pub fn SUB(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x22;

    return opcode | rs | rt | rd | shamt | funct;
}

pub fn SUBU(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x23;

    return opcode | rs | rt | rd | shamt | funct;
}

pub fn AND(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x24;

    return opcode | rs | rt | rd | shamt | funct;
}

pub fn OR(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x25;

    return opcode | rs | rt | rd | shamt | funct;
}

pub fn XOR(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x26;

    return opcode | rs | rt | rd | shamt | funct;
}

pub fn NOR(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x27;

    return opcode | rs | rt | rd | shamt | funct;
}

pub fn SLT(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x2A;

    return opcode | rs | rt | rd | shamt | funct;
}

pub fn SLTU(RD: u8, RS: u8, RT: u8) -> u32 {
    let opcode = 0x00 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let rd = (RD as u32) << 11;
    let shamt = 0 << 6;
    let funct = 0x2B;

    return opcode | rs | rt | rd | shamt | funct;
}

// j-format

pub fn J(address: u32) -> u32 {
    if address > 0x03ff_ffff {
        panic!("Error [J macro]: Address [{:X}] too large!", address);
    }
    let opcode = 0x02 << 26;
    let addr = address & 0x03ff_ffff;

    return opcode | addr;
}

// i-format

pub fn BEQ(RS: u8, RT: u8, address: u16) -> u32 {
    let opcode = 0x04 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let addr = address as u32;

    return opcode | rs | rt | addr;
}

pub fn BNE(RS: u8, RT: u8, address: u16) -> u32 {
    let opcode = 0x05 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let addr = address as u32;

    return opcode | rs | rt | addr;
}

pub fn ADDI(RT: u8, RS: u8, imm16: u16) -> u32 {
    let opcode = 0x08 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let imm32 = imm16 as u32;

    return opcode | rs | rt | imm32;
}

pub fn ADDIU(RT: u8, RS: u8, imm16: u16) -> u32 {
    let opcode = 0x09 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let imm32 = imm16 as u32;

    return opcode | rs | rt | imm32;
}

pub fn SLTI(RT: u8, RS: u8, imm16: u16) -> u32 {
    let opcode = 0x0A << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let imm32 = imm16 as u32;

    return opcode | rs | rt | imm32;
}

pub fn SLTIU(RT: u8, RS: u8, imm16: u16) -> u32 {
    let opcode = 0x0B << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let imm32 = imm16 as u32;

    return opcode | rs | rt | imm32;
}

pub fn ANDI(RT: u8, RS: u8, imm16: u16) -> u32 {
    let opcode = 0x0C << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let imm32 = imm16 as u32;

    return opcode | rs | rt | imm32;
}

pub fn ORI(RT: u8, RS: u8, imm16: u16) -> u32 {
    let opcode = 0x0D << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let imm32 = imm16 as u32;

    return opcode | rs | rt | imm32;
}

pub fn LUI(RT: u8, imm16: u16) -> u32 {
    let opcode = 0x0F << 26;
    let rs = 0 << 21;
    let rt = (RT as u32) << 16;
    let imm32 = imm16 as u32;

    return opcode | rs | rt | imm32;
}

pub fn LB(RT: u8, addr_offset: u16, RS: u8) -> u32 {
    let opcode = 0x20 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let addr = addr_offset as u32;

    return opcode | rs | rt | addr;
}

pub fn LW(RT: u8, addr_offset: u16, RS: u8) -> u32 {
    let opcode = 0x23 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let addr = addr_offset as u32;

    return opcode | rs | rt | addr;
}

pub fn SB(RT: u8, addr_offset: u16, RS: u8) -> u32 {
    let opcode = 0x28 << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let addr = addr_offset as u32;

    return opcode | rs | rt | addr;
}

pub fn SW(RT: u8, addr_offset: u16, RS: u8) -> u32 {
    let opcode = 0x2B << 26;
    let rs = (RS as u32) << 21;
    let rt = (RT as u32) << 16;
    let addr = addr_offset as u32;

    return opcode | rs | rt | addr;
}



#[cfg(test)]
mod tests {
    use super::*;
    use super::super::reg_macros::*;

    #[test]
    fn test_ADD() {
        let instr1 = ADD(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B4820);

        let instr2 = ADD(S0(), A0(), V1());
        assert_eq!(instr2, 0x00838020);
    }

    #[test]
    fn test_ADDU() {
        let instr1 = ADDU(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B4821);
    }

    #[test]
    fn test_SUB() {
        let instr1 = SUB(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B4822);
    }

    #[test]
    fn test_SUBU() {
        let instr1 = SUBU(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B4823);
    }

    #[test]
    fn test_AND() {
        let instr1 = AND(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B4824);
    }

    #[test]
    fn test_OR() {
        let instr1 = OR(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B4825);
    }

    #[test]
    fn test_XOR() {
        let instr1 = XOR(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B4826);
    }

    #[test]
    fn test_NOR() {
        let instr1 = NOR(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B4827);
    }

    #[test]
    fn test_SLT() {
        let instr1 = SLT(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B482A);
    }

    #[test]
    fn test_SLTU() {
        let instr1 = SLTU(T1(), T2(), T3());
        assert_eq!(instr1, 0x014B482B);
    }

    #[test]
    fn test_J() {
        let instr1 = J(0xDEAD);
        assert_eq!(instr1, 0x0800DEAD);
    }

    #[test]
    fn test_BEQ() {
        let instr1 = BEQ(T0(), T1(), 0xBABE);
        assert_eq!(instr1, 0x1109BABE);
    }

    #[test]
    fn test_BNE() {
        let instr1 = BNE(T0(), T1(), 0xCAFE);
        assert_eq!(instr1, 0x1509CAFE);
    }

    #[test]
    fn test_ADDI() {
        let instr1 = ADDI(T0(), T1(), 0x23);
        assert_eq!(instr1, 0x21280023);
    }

    #[test]
    fn test_ADDIU() {
        let instr1 = ADDIU(T0(), T1(), 0x23);
        assert_eq!(instr1, 0x25280023);
    }

    #[test]
    fn test_SLTI() {
        let instr1 = SLTI(T0(), T1(), 0x42);
        assert_eq!(instr1, 0x29280042);
    }

    #[test]
    fn test_SLTIU() {
        let instr1 = SLTIU(T0(), T1(), 0x42);
        assert_eq!(instr1, 0x2D280042);
    }

    #[test]
    fn test_ANDI() {
        let instr1 = ANDI(T0(), T1(), 0x42);
        assert_eq!(instr1, 0x31280042);
    }

    #[test]
    fn test_ORI() {
        let instr1 = ORI(T0(), T1(), 0x42);
        assert_eq!(instr1, 0x35280042);
    }

    #[test]
    fn test_LUI() {
        let instr1 = LUI(T0(), 0xaaa4);
        assert_eq!(instr1, 0x3C08AAA4);
    }

    #[test]
    fn test_LB() {
        let instr1 = LB(T0(), 0x4, T1());
        assert_eq!(instr1, 0x81280004);
    }

    #[test]
    fn test_LW() {
        let instr1 = LW(T0(), 0x4, T1());
        assert_eq!(instr1, 0x8D280004);
    }

    #[test]
    fn test_SB() {
        let instr1 = SB(T0(), 0x4, T1());
        assert_eq!(instr1, 0xA1280004);
    }

    #[test]
    fn test_SW() {
        let instr1 = SW(T0(), 0x4, T1());
        assert_eq!(instr1, 0xAD280004);
    }
}