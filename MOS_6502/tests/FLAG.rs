// set and clear instructions
use crate::tests::*;

fn test_flag(_cpu: &mut Mos6502, f: Mos6502Flag, set: bool) {
    _cpu.ps |= Mos6502Flag::C as u8;
    let orig = _cpu.ps;
    _cpu.step();
    t(&_cpu,_cpu.flag(f) == set,format!("{:08b} -> {:08b} (not {:08b})",orig,_cpu.ps,f as u8).as_ref());
    _cpu.reset();
    let orig = _cpu.ps;
    _cpu.step();
    t(&_cpu,_cpu.flag(f) == set,format!("{:08b} -> {:08b} (not {:08b})",orig,_cpu.ps,f as u8).as_ref());
}
#[test]
fn test_CLC() {
    let mut _cpu = cpu_prep(0x123,"CLC",IMPLIED);
    test_flag(&mut _cpu, Mos6502Flag::C, false);
}
#[test]
fn test_CLD() {
    let mut _cpu = cpu_prep(0x123,"CLD",IMPLIED);
    test_flag(&mut _cpu,Mos6502Flag::D, false);
}
#[test]
fn test_CLI() {
    let mut _cpu = cpu_prep(0x123,"CLI",IMPLIED);
    test_flag(&mut _cpu,Mos6502Flag::I, false);
}
#[test]
fn test_CLV() {
    let mut _cpu = cpu_prep(0x123,"CLV",IMPLIED);
    test_flag(&mut _cpu,Mos6502Flag::V, false);
}
#[test]
fn test_SEC() {
    let mut _cpu = cpu_prep(0x123,"SEC",IMPLIED);
    test_flag(&mut _cpu,Mos6502Flag::C, true);
}
#[test]
fn test_SED() {
    let mut _cpu = cpu_prep(0x123,"SED",IMPLIED);
    test_flag(&mut _cpu,Mos6502Flag::D, true);
}
#[test]
fn test_SEI() {
    let mut _cpu = cpu_prep(0x123,"SEI",IMPLIED);
    test_flag(&mut _cpu,Mos6502Flag::I, true);
}