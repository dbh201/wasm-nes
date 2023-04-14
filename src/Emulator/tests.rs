/*
    Test every addressing mode with ADC, JMP, and LDX/STX.
    Then, test every other instruction with the simplest mode available.

    It would be ideal to test all ~130 instruction variants, but this will
    take a lot of code.
*/
#![cfg(test)]
#![cfg_attr(test, allow(dead_code))]
#![cfg_attr(test, allow(non_snake_case))]
#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(test, allow(unused_must_use))]
// Test modules
mod ADC;
mod AND;
mod ASL;
mod BIT;
mod EOR;
mod LDX;
mod LDY;
mod LDA;
mod LSR;
mod ORA;
mod ROL;
mod ROR;
mod SBC;
mod STX;
mod STY;
mod STA;
mod BranchInstructions;
mod CompareInstructions;
mod FlagInstructions;
mod IncDecInstructions;
mod JumpInstructions;
mod StackInstructions;
mod SystemInstructions;
mod TransferInstructions;

mod SuperMarioBros;



use std::{cell::RefCell, rc::Rc};





use super::CPU::MOS_6502::Mos6502::{Mos6502,Mos6502Flag};
use super::Common::AddressNode::*;
use super::CPU::MOS_6502::Mos6502Debug::AddrMode::{self,*};
use super::Common::AddressBus::AddressBus;
use super::Common::AddressBus::MemRW;
use super::System::NES::NES;
use super::Hardware::NES::*;
use crate::dummy_console_log as console_log;

// Utility functions
pub fn t(_cpu: &Mos6502, c: bool, msg: &str) {
    assert!(c, "{}\n-----\n{}\n-----\n", msg, _cpu);
}
#[test]
pub fn test_mem(){
    let _cpu = cpu_prep(0x1234,"NOP",IMPLIED);
    let op = _cpu.getmem(0x1234).unwrap();
    assert!(op == 0xEA,"op was {:02X}",op);
}
pub fn cpu_prep(addr: u16, mnem: &str, mode: AddrMode) -> Mos6502 {
    let mut _cpu = Mos6502::new(
        "TESTCPU".to_owned(),
        Rc::new(RefCell::new(AddressBus::new("TESTMMU".to_owned()).unwrap()))
    ).unwrap();
    let mut mem = AddressNode::new("64KB RAM".to_string());
    let op = _cpu.debug.get_opcode(mnem, mode);
    mem.add_addr_range(0,0xFFFF).expect("cpu_prep ERR:");
    mem.make_ram(0xFFFF).expect("cpu_prep ERR:");
    mem.set(addr,op).expect("TestCpu:");
    console_log!("memget: {:02X}", mem.get(addr).unwrap());
    _cpu.bus.borrow_mut().register_AddressNode(mem).expect("TestCpu:");
    console_log!("get: {:02X}", _cpu.getmem(addr).unwrap());
    _cpu._place_u16(0xFFFC, addr);
    assert!(_cpu.reset().is_ok());
    _cpu
}
pub fn nf(_cpu: &Mos6502, val: u8) {
    t(
        _cpu,
        _cpu.flag(Mos6502Flag::N) == ((val as i8) < 0),
        format!("[N flag error] val:{} N:{}",val as i8,_cpu.flag(Mos6502Flag::N)).as_ref()
    );
}
pub fn zf(_cpu: &Mos6502, val: u8) {
    t(
        _cpu,
        _cpu.flag(Mos6502Flag::Z) == (val == 0),
        format!("[Z flag error] val:{}, Z:{}",val,_cpu.flag(Mos6502Flag::Z)).as_ref()
    );
}
pub fn nzf(_cpu: &Mos6502, val: u8) {
    nf(_cpu, val);
    zf(_cpu, val);
}
pub fn cf(_cpu: &Mos6502, val: u16) {
    t(
        _cpu,
        _cpu.flag(Mos6502Flag::C) == (val > 255),
        format!("[C flag error] C:{}, {:X} > 0xFF:{}",_cpu.flag(Mos6502Flag::C),val,(val > 255)).as_ref()
    );
}
