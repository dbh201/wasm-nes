use crate::tests::*;

fn assert_LSR(_cpu: &Mos6502, val: u8, orig: u8) {
    let expected = orig>>1 as u8;
    let carry = orig & 0x01;
    let cflag = _cpu.flag(Mos6502Flag::C);
    t(_cpu, expected == val,format!("expected {}, got {}",expected,val).as_ref());
    t(_cpu, cflag as u8 == carry, format!("carry was {}, but bit 0 was {}", cflag as u8, carry).as_ref());
    nzf(_cpu, val);
}

#[test]
fn test_LSR_ACC() {
    let mut _cpu = cpu_prep(0x0800,"LSR",ACCUMULATOR);
    _cpu.a = 0x81;
    _cpu.step();
    assert_LSR(&_cpu,_cpu.a,0x81);
}
#[test]
fn test_LSR_ZP() {
    let mut _cpu = cpu_prep(0x0800,"LSR",ZERO_PAGE);
    _cpu.setmem(0x0801,0x55);
    _cpu.setmem(0x55,0x77);
    _cpu.step();
    let res = _cpu.getmem(0x55);
    assert_LSR(&_cpu,res,0x77);   
}
#[test]
fn test_LSR_ZP_X() {
    let mut _cpu = cpu_prep(0x0800,"LSR",ZERO_PAGE_X);
    _cpu.setmem(0x0801,0x55);
    _cpu.x=0xFF;
    _cpu.setmem(0x54,0x77);
    _cpu.step();
    let res = _cpu.getmem(0x54);
    assert_LSR(&_cpu,res,0x77);
}
#[test]
fn test_LSR_ABS() {
    let mut _cpu = cpu_prep(0x0800,"LSR",ABSOLUTE);
    _cpu.setmem(0x0801,0x55);
    _cpu.setmem(0x0802,0x66);
    _cpu.setmem(0x6655,0x77);
    _cpu.step();
    let res = _cpu.getmem(0x6655);
    assert_LSR(&_cpu,res,0x77);
}
#[test]
fn test_LSR_ABS_X() {
    let mut _cpu = cpu_prep(0x0800,"LSR",ABSOLUTE_X);
    _cpu.setmem(0x0801,0x55);
    _cpu.setmem(0x0802,0x66);
    _cpu.x=0xFF;
    _cpu.setmem(0x6754,0x77);
    _cpu.step();
    let res = _cpu.getmem(0x6754);
    assert_LSR(&_cpu,res,0x77);
}