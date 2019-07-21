/*
 * reg_macros.rs
 * 
 * Author: Travis Banken
 * 
 * Holds macros and functions for refering to registers
 */
#![allow(dead_code)]
#![allow(non_snake_case)]

// Registers
#[inline]
pub fn ZERO() -> u8 {0}

#[inline]
pub fn AT() -> u8 {1}

#[inline]
pub fn V0() -> u8 {2}

#[inline]
pub fn V1() -> u8 {3}

#[inline]
pub fn A0() -> u8 {4}

#[inline]
pub fn A1() -> u8 {5}

#[inline]
pub fn A2() -> u8 {6}

#[inline]
pub fn A3() -> u8 {7}

#[inline]
pub fn T0() -> u8 {8}

#[inline]
pub fn T1() -> u8 {9}

#[inline]
pub fn T2() -> u8 {10}

#[inline]
pub fn T3() -> u8 {11}

#[inline]
pub fn T4() -> u8 {12}

#[inline]
pub fn T5() -> u8 {13}

#[inline]
pub fn T6() -> u8 {14}

#[inline]
pub fn T7() -> u8 {15}

#[inline]
pub fn S0() -> u8 {16}

#[inline]
pub fn S1() -> u8 {17}

#[inline]
pub fn S2() -> u8 {18}

#[inline]
pub fn S3() -> u8 {19}

#[inline]
pub fn S4() -> u8 {20}

#[inline]
pub fn S5() -> u8 {21}

#[inline]
pub fn S6() -> u8 {22}

#[inline]
pub fn S7() -> u8 {23}

#[inline]
pub fn T8() -> u8 {24}

#[inline]
pub fn T9() -> u8 {25}

#[inline]
pub fn K0() -> u8 {26}

#[inline]
pub fn K1() -> u8 {27}

#[inline]
pub fn GP() -> u8 {28}

#[inline]
pub fn SP() -> u8 {29}

#[inline]
pub fn FP() -> u8 {30}

#[inline]
pub fn RA() -> u8 {31}