use crate::tests::*;

fn assert_ASL(_cpu: &Mos6502, val: u8, orig: u8) {
    let expected = (((orig as u16)<<1)%256) as u8;
    t(&_cpu, expected == val,format!("expected {}, got {}",expected,val).as_ref());
    cf(&_cpu, (orig as u16)<<1 );
    nzf(&_cpu, val);
}

#[test]
fn test_ASL_ACC() {
    let mut _cpu = cpu_prep(0x0800,"ASL",ACCUMULATOR);
    _cpu.a = 0x81;
    _cpu.step();
    assert_ASL(&_cpu,_cpu.a,0x81);
}
#[test]
fn test_ASL_ZP() {
    let mut _cpu = cpu_prep(0x0800,"ASL",ZERO_PAGE);
    _cpu.setmem(0x0801,0x55);
    _cpu.setmem(0x55,0x77);
    _cpu.step();
    let res = _cpu.getmem(0x55);
    assert_ASL(&_cpu,res,0x77);   
}
#[test]
fn test_ASL_ZP_X() {
    let mut _cpu = cpu_prep(0x0800,"ASL",ZERO_PAGE_X);
    _cpu.setmem(0x0801,0x55);
    _cpu.x=0xFF;
    _cpu.setmem(0x54,0x77);
    _cpu.step();
    let res = _cpu.getmem(0x54);
    assert_ASL(&_cpu,res,0x77);
}
#[test]
fn test_ASL_ABS() {
    let mut _cpu = cpu_prep(0x0800,"ASL",ABSOLUTE);
    _cpu.setmem(0x0801,0x55);
    _cpu.setmem(0x0802,0x66);
    _cpu.setmem(0x6655,0x77);
    _cpu.step();
    let res = _cpu.getmem(0x6655);
    assert_ASL(&_cpu,res,0x77);
}
#[test]
fn test_ASL_ABS_X() {
    let mut _cpu = cpu_prep(0x0800,"ASL",ABSOLUTE_X);
    _cpu.setmem(0x0801,0x55);
    _cpu.setmem(0x0802,0x66);
    _cpu.x=0xFF;
    _cpu.setmem(0x6754,0x77);
    _cpu.step();
    let res = _cpu.getmem(0x6754);
    assert_ASL(&_cpu,res,0x77);
}