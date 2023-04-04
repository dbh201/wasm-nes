use crate::tests::*;

fn assert_inc(_cpu: &Mos6502, res: u8, orig: u8) {
    let expected = ((orig as u16 + 1) % 256) as u8;
    t(&_cpu, res == expected, format!("m{} != {}",res,expected).as_ref());
    nzf(_cpu,res);
}
fn assert_dec(_cpu: &Mos6502, res: u8, orig: u8) {
    let expected = ((orig as u16 + 255) % 256) as u8;
    t(&_cpu, res == expected, format!("m{} != {}",res,expected).as_ref());
    nzf(_cpu,res);
}
#[test]
fn test_INC_ZP() {
    let mut _cpu = cpu_prep(0x0800,"INC",ZERO_PAGE);
    _cpu.setmem(0x0801,0x87);
    _cpu.setmem(0x87,0xED);
    _cpu.step();
    let res = _cpu.getmem(0x87);
    assert_inc(&_cpu,res,0xED);
}
#[test]
fn test_INC_ZP_X() {
    let mut _cpu = cpu_prep(0x0800,"INC",ZERO_PAGE_X);
    _cpu.setmem(0x0801,0x85);
    _cpu.x = 2;
    _cpu.setmem(0x87,0xFF);
    _cpu.step();
    let res = _cpu.getmem(0x87);
    assert_inc(&_cpu,res,0xFF);
}
#[test]
fn test_INC_ABS() {
    let mut _cpu = cpu_prep(0x0800,"INC",ABSOLUTE);
    _cpu.setmem(0x0801,0x87);
    _cpu.setmem(0x0802,0xA9);
    _cpu.setmem(0xA987,0xED);
    _cpu.step();
    let res = _cpu.getmem(0xA987);
    assert_inc(&_cpu,res,0xED);
}
#[test]
fn test_INC_ABS_X() {
    let mut _cpu = cpu_prep(0x0800,"INC",ABSOLUTE_X);
    _cpu.setmem(0x0801,0x83);
    _cpu.setmem(0x0802,0xA9);
    _cpu.x = 4;
    _cpu.setmem(0xA987,0x7F);
    _cpu.step();
    let res = _cpu.getmem(0xA987);
    assert_inc(&_cpu, res, 0x7F);
}
#[test]
fn test_INX() {
    let mut _cpu = cpu_prep(0x0800,"INX",IMPLIED);
    _cpu.x = 0xFF;
    _cpu.step();
    assert_inc(&_cpu, _cpu.x, 0xFF);
}
#[test]
fn test_INY() {
    let mut _cpu = cpu_prep(0x0800,"INY",IMPLIED);
    _cpu.y = 0xFF;
    _cpu.step();
    assert_inc(&_cpu, _cpu.y, 0xFF);
}
#[test]
fn test_DEC_ZP() {
    let mut _cpu = cpu_prep(0x0800,"DEC",ZERO_PAGE);
    _cpu.setmem(0x0801,0x87);
    _cpu.setmem(0x87,0xED);
    _cpu.step();
    let res = _cpu.getmem(0x87);
    assert_dec(&_cpu,res,0xED);
}
#[test]
fn test_DEC_ZP_X() {
    let mut _cpu = cpu_prep(0x0800,"DEC",ZERO_PAGE_X);
    _cpu.setmem(0x0801,0x85);
    _cpu.x = 2;
    _cpu.setmem(0x87,0xFF);
    _cpu.step();
    let res = _cpu.getmem(0x87);
    assert_dec(&_cpu,res,0xFF);
}
#[test]
fn test_DEC_ABS() {
    let mut _cpu = cpu_prep(0x0800,"DEC",ABSOLUTE);
    _cpu.setmem(0x0801,0x87);
    _cpu.setmem(0x0802,0xA9);
    _cpu.setmem(0xA987,0x00);
    _cpu.step();
    let res = _cpu.getmem(0xA987);
    assert_dec(&_cpu,res,0x00);
}
#[test]
fn test_DEC_ABS_X() {
    let mut _cpu = cpu_prep(0x0800,"DEC",ABSOLUTE_X);
    _cpu.setmem(0x0801,0x83);
    _cpu.setmem(0x0802,0xA9);
    _cpu.x = 4;
    _cpu.setmem(0xA987,0x80);
    _cpu.step();
    let res = _cpu.getmem(0xA987);
    assert_dec(&_cpu, res, 0x80);
}
#[test]
fn test_DEX() {
    let mut _cpu = cpu_prep(0x0800,"DEX",IMPLIED);
    _cpu.x = 0x00;
    _cpu.step();
    assert_dec(&_cpu, _cpu.x, 0x00);
}
#[test]
fn test_DEY() {
    let mut _cpu = cpu_prep(0x0800,"DEY",IMPLIED);
    _cpu.y = 0x00;
    _cpu.step();
    assert_dec(&_cpu, _cpu.y, 0x00);
}