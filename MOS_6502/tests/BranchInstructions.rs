use crate::tests::*;
// all branching instructions are tested here

#[test]
fn test_BCC() {
    let mut _cpu = cpu_prep(0x0100,"BCC",RELATIVE);
    let rel: i16 = -4;
    let nobranch: i16 = 0x0102;
    let branch: i16 = 0x0102 + rel;
    _cpu.setmem(0x0101, rel as u8);
    _cpu.step();
    t(&_cpu, _cpu.pc == branch as u16, format!("branch, PC: {:X} != {:X}", _cpu.pc, branch).as_ref());
    _cpu.reset();
    _cpu.ps |= Mos6502Flag::C as u8;
    _cpu.step();
    t(&_cpu, _cpu.pc == 0x0102, format!("no branch, PC: {:X} != {:X}", _cpu.pc, nobranch).as_ref());
}
#[test]
fn test_BCS() {
    let mut _cpu = cpu_prep(0x0100,"BCS",RELATIVE);
    let rel: i16 = -4;
    let nobranch: i16 = 0x0102;
    let branch: i16 = 0x0102 + rel;
    _cpu.setmem(0x0101, rel as u8);
    _cpu.ps |= Mos6502Flag::C as u8;
    _cpu.step();
    t(&_cpu, _cpu.pc == branch as u16, format!("branch, PC: {:X} != {:X}", _cpu.pc, branch).as_ref());
    _cpu.reset();
    _cpu.step();
    t(&_cpu, _cpu.pc == 0x0102, format!("no branch, PC: {:X} != {:X}", _cpu.pc, nobranch).as_ref());
}
#[test]
fn test_BEQ() {
    let mut _cpu = cpu_prep(0x0100,"BEQ",RELATIVE);
    let rel: i16 = -4;
    let nobranch: i16 = 0x0102;
    let branch: i16 = 0x0102 + rel;
    _cpu.setmem(0x0101, rel as u8);
    _cpu.ps |= Mos6502Flag::Z as u8;
    _cpu.step();
    t(&_cpu, _cpu.pc == branch as u16, format!("branch, PC: {:X} != {:X}", _cpu.pc, branch).as_ref());
    _cpu.reset();
    _cpu.step();
    t(&_cpu, _cpu.pc == 0x0102, format!("no branch, PC: {:X} != {:X}", _cpu.pc, nobranch).as_ref());
}
#[test]
fn test_BMI() {
    let mut _cpu = cpu_prep(0x0100,"BMI",RELATIVE);
    let rel: i16 = -4;
    let nobranch: i16 = 0x0102;
    let branch: i16 = 0x0102 + rel;
    _cpu.setmem(0x0101, rel as u8);
    _cpu.ps |= Mos6502Flag::N as u8;
    _cpu.step();
    t(&_cpu, _cpu.pc == branch as u16, format!("branch, PC: {:X} != {:X}", _cpu.pc, branch).as_ref());
    _cpu.reset();
    _cpu.step();
    t(&_cpu, _cpu.pc == 0x0102, format!("no branch, PC: {:X} != {:X}", _cpu.pc, nobranch).as_ref());
}
#[test]
fn test_BNE() {
    let mut _cpu = cpu_prep(0x0100,"BNE",RELATIVE);
    let rel: i16 = -4;
    let nobranch: i16 = 0x0102;
    let branch: i16 = 0x0102 + rel;
    _cpu.setmem(0x0101, rel as u8);
    _cpu.step();
    t(&_cpu, _cpu.pc == branch as u16, format!("branch, PC: {:X} != {:X}", _cpu.pc, branch).as_ref());
    _cpu.reset();
    _cpu.ps |= Mos6502Flag::Z as u8;
    _cpu.step();
    t(&_cpu, _cpu.pc == 0x0102, format!("no branch, PC: {:X} != {:X}", _cpu.pc, nobranch).as_ref());
}
#[test]
fn test_BPL() {
    let mut _cpu = cpu_prep(0x0100,"BPL",RELATIVE);
    let rel: i16 = -4;
    let nobranch: i16 = 0x0102;
    let branch: i16 = 0x0102 + rel;
    _cpu.setmem(0x0101, rel as u8);
    _cpu.step();
    t(&_cpu, _cpu.pc == branch as u16, format!("branch, PC: {:X} != {:X}", _cpu.pc, branch).as_ref());
    _cpu.reset();
    _cpu.ps |= Mos6502Flag::N as u8;
    _cpu.step();
    t(&_cpu, _cpu.pc == 0x0102, format!("no branch, PC: {:X} != {:X}", _cpu.pc, nobranch).as_ref());
}
#[test]
fn test_BVC() {
    let mut _cpu = cpu_prep(0x0100,"BVC",RELATIVE);
    let rel: i16 = -4;
    let nobranch: i16 = 0x0102;
    let branch: i16 = 0x0102 + rel;
    _cpu.setmem(0x0101, rel as u8);
    _cpu.step();
    t(&_cpu, _cpu.pc == branch as u16, format!("branch, PC: {:X} != {:X}", _cpu.pc, branch).as_ref());
    _cpu.reset();
    _cpu.ps |= Mos6502Flag::V as u8;
    _cpu.step();
    t(&_cpu, _cpu.pc == 0x0102, format!("no branch, PC: {:X} != {:X}", _cpu.pc, nobranch).as_ref());
}
#[test]
fn test_BVS() {
    let mut _cpu = cpu_prep(0x0100,"BVS",RELATIVE);
    let rel: i16 = -4;
    let nobranch: i16 = 0x0102;
    let branch: i16 = 0x0102 + rel;
    _cpu.setmem(0x0101,0xFC);
    _cpu.ps |= Mos6502Flag::V as u8;
    _cpu.step();
    t(&_cpu, _cpu.pc == branch as u16, format!("branch, PC: {:X} != {:X}", _cpu.pc, branch).as_ref());
    _cpu.reset();
    _cpu.step();
    t(&_cpu, _cpu.pc == 0x0102, format!("no branch, PC: {:X} != {:X}", _cpu.pc, nobranch).as_ref());
}