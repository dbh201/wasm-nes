use super::super::tests::*;

fn assert_EOR(_cpu: &Mos6502,  a: u8, b: u8) {
    t(_cpu, _cpu.a == a^b,format!("{} | {} = {} expected, got {}",a,b,a^b,_cpu.a).as_ref());
    nzf(_cpu,_cpu.a);
}
#[test]
fn test_EOR_IMM() {
    let mut _cpu = cpu_prep(0x1000,"EOR", IMMEDIATE);
    _cpu.setmem(0x1001,0b01010101);
    _cpu.a = 0b10101010;
    _cpu.step();
    assert_EOR(&_cpu,0b10101010,0b01010101);
}
#[test]
fn test_EOR_ZP() {
    let mut _cpu = cpu_prep(0x1000,"EOR", ZERO_PAGE);
    _cpu.setmem(0x1001,0x23);
    _cpu.setmem(0x23,0b11001100);
    _cpu.a = 0b11000011;
    _cpu.step();
    assert_EOR(&_cpu,0b11000011,0b11001100);
}
#[test]
fn test_EOR_ZP_X() {
    let mut _cpu = cpu_prep(0x1000,"EOR", ZERO_PAGE_X);
    _cpu.setmem(0x1001,0x45);
    _cpu.x = 4;
    _cpu.setmem(0x49,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_EOR(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_EOR_ABS() {
    let mut _cpu = cpu_prep(0x1000,"EOR", ABSOLUTE);
    _cpu.setmem(0x1001,0x45);
    _cpu.setmem(0x1002,0x67);
    _cpu.setmem(0x6745,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_EOR(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_EOR_ABS_X() {
    let mut _cpu = cpu_prep(0x1000,"EOR", ABSOLUTE_X);
    _cpu.setmem(0x1001,0x45);
    _cpu.setmem(0x1002,0x67);
    _cpu.x = 5;
    _cpu.setmem(0x674A,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_EOR(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_EOR_ABS_Y() {
    let mut _cpu = cpu_prep(0x1000,"EOR", ABSOLUTE_Y);
    _cpu.setmem(0x1001,0x45);
    _cpu.setmem(0x1002,0x67);
    _cpu.y = 5;
    _cpu.setmem(0x674A,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_EOR(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_EOR_IND_X() {
    let mut _cpu = cpu_prep(0x1000,"EOR", INDIRECT_X);
    _cpu.setmem(0x1001, 0x67);
    _cpu._place_u16(0x006A, 0x1234);
    _cpu.x = 3;
    _cpu.setmem(0x1234, 0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_EOR(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_EOR_IND_Y() {
    let mut _cpu = cpu_prep(0x1000,"EOR", INDIRECT_Y);
    _cpu.setmem(0x1001,0x01);
    _cpu.y = 5;
    _cpu._place_u16(0x01,0x1234);
    _cpu.setmem(0x1239,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_EOR(&_cpu,0b00111100,0b11110000);
}