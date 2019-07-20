/*
 * control_bits.rs
 * 
 * Author: Travis Banken
 * 
 * Structure for the control bits of the cpu
 */
#[allow(dead_code)]

pub struct ControlBits {
    pub alu_op: u8,
    pub alu_bnegate: u8,

    pub mem_read: u8,
    pub mem_write: u8,
    pub mem_to_reg: u8,

    pub reg_dst: u8,
    pub reg_write: u8,

    pub branch: u8,
    pub jump: u8,

    // extra
    pub not_res: u8,
    pub mem_by_byte: u8,
}

impl Default for ControlBits {
    fn default() -> ControlBits {
        ControlBits {
            alu_op: 0,
            alu_bnegate: 0,

            mem_read: 0,
            mem_write: 0,
            mem_to_reg: 0,

            reg_dst: 0,
            reg_write: 0,

            branch: 0,
            jump: 0,

            not_res: 0,
            mem_by_byte: 0,
        }
    }
}