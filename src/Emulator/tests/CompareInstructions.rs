use super::super::tests::*;

fn assert_CMP(_cpu: &Mos6502,  a: u8, b: u8) {
    let res: i16 = a as i16 - b as i16;
    let res: u8 = ((res as u16) % 256) as u8;
    t(_cpu, (a >= b) == _cpu.flag(Mos6502Flag::C), format!("{} >= {} {}?",a,b,_cpu.flag(Mos6502Flag::C)).as_ref());
    t(_cpu, (a == b) == _cpu.flag(Mos6502Flag::Z), format!("{} == {} {}?",a,b,_cpu.flag(Mos6502Flag::Z)).as_ref());
    t(_cpu, ((res & 0x80) > 0) == _cpu.flag(Mos6502Flag::N), format!("{:02X} & 0x80 > 0 {}?",res,_cpu.flag(Mos6502Flag::N)).as_ref());
}

#[test]
fn test_CMP_IMM() {
    let mut _cpu = cpu_prep(0x1000,"CMP", IMMEDIATE);
    _cpu.setmem(0x1001,0b01010101);
    _cpu.a = 0b10101010;
    _cpu.step();
    assert_CMP(&_cpu,0b10101010,0b01010101);
}
#[test]
fn test_CMP_ZP() {
    let mut _cpu = cpu_prep(0x1000,"CMP", ZERO_PAGE);
    _cpu.setmem(0x1001,0x23);
    _cpu.setmem(0x23,0b11001100);
    _cpu.a = 0b11000011;
    _cpu.step();
    assert_CMP(&_cpu,0b11000011,0b11001100);
}
#[test]
fn test_CMP_ZP_X() {
    let mut _cpu = cpu_prep(0x1000,"CMP", ZERO_PAGE_X);
    _cpu.setmem(0x1001,0x45);
    _cpu.x = 4;
    _cpu.setmem(0x49,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_CMP(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_CMP_ABS() {
    let mut _cpu = cpu_prep(0x1000,"CMP", ABSOLUTE);
    _cpu.setmem(0x1001,0x45);
    _cpu.setmem(0x1002,0x67);
    _cpu.setmem(0x6745,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_CMP(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_CMP_ABS_X() {
    let mut _cpu = cpu_prep(0x1000,"CMP", ABSOLUTE_X);
    _cpu.setmem(0x1001,0x45);
    _cpu.setmem(0x1002,0x67);
    _cpu.x = 5;
    _cpu.setmem(0x674A,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_CMP(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_CMP_ABS_Y() {
    let mut _cpu = cpu_prep(0x1000,"CMP", ABSOLUTE_Y);
    _cpu.setmem(0x1001,0x45);
    _cpu.setmem(0x1002,0x67);
    _cpu.y = 5;
    _cpu.setmem(0x674A,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_CMP(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_CMP_IND_X() {
    let mut _cpu = cpu_prep(0x1000,"CMP", INDIRECT_X);
    _cpu.setmem(0x1001, 0x67);
    _cpu._place_u16(0x006A, 0x1234);
    _cpu.x = 3;
    _cpu.setmem(0x1234, 0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_CMP(&_cpu,0b00111100,0b11110000);
}
#[test]
fn test_CMP_IND_Y() {
    let mut _cpu = cpu_prep(0x1000,"CMP", INDIRECT_Y);
    _cpu.setmem(0x1001,0x01);
    _cpu.y = 5;
    _cpu._place_u16(0x01,0x1234);
    _cpu.setmem(0x1239,0b11110000);
    _cpu.a = 0b00111100;
    _cpu.step();
    assert_CMP(&_cpu,0b00111100,0b11110000);
}


#[test]
fn test_CPX_IMM() {
    let mut _cpu = cpu_prep(0x1000,"CPX", IMMEDIATE);
    _cpu.setmem(0x1001,0b01010101);
    _cpu.x = 0b10101010;
    _cpu.step();
    assert_CMP(&_cpu,0b10101010,0b01010101);
}
#[test]
fn test_CPX_ZP() {
    let mut _cpu = cpu_prep(0x1000,"CPX", ZERO_PAGE);
    _cpu.setmem(0x1001,0x23);
    _cpu.setmem(0x23,0b11001100);
    _cpu.x = 0b11000011;
    _cpu.step();
    assert_CMP(&_cpu,0b11000011,0b11001100);
}
#[test]
fn test_CPX_ABS() {
    let mut _cpu = cpu_prep(0x1000,"CPX", ABSOLUTE);
    _cpu.setmem(0x1001,0x45);
    _cpu.setmem(0x1002,0x67);
    _cpu.setmem(0x6745,0b11110000);
    _cpu.x = 0b00111100;
    _cpu.step();
    assert_CMP(&_cpu,0b00111100,0b11110000);
}


#[test]
fn test_CPY_IMM() {
    let mut _cpu = cpu_prep(0x1000,"CPY", IMMEDIATE);
    _cpu.setmem(0x1001,0b01010101);
    _cpu.y = 0b10101010;
    _cpu.step();
    assert_CMP(&_cpu,0b10101010,0b01010101);
}
#[test]
fn test_CPY_ZP() {
    let mut _cpu = cpu_prep(0x1000,"CPY", ZERO_PAGE);
    _cpu.setmem(0x1001,0x23);
    _cpu.setmem(0x23,0b11001100);
    _cpu.y = 0b11000011;
    _cpu.step();
    assert_CMP(&_cpu,0b11000011,0b11001100);
}
#[test]
fn test_CPY_ABS() {
    let mut _cpu = cpu_prep(0x1000,"CPY", ABSOLUTE);
    _cpu.setmem(0x1001,0x45);
    _cpu.setmem(0x1002,0x67);
    _cpu.setmem(0x6745,0b11110000);
    _cpu.y = 0b00111100;
    _cpu.step();
    assert_CMP(&_cpu,0b00111100,0b11110000);
}