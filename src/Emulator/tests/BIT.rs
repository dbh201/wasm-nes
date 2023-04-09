use super::super::tests::*;

#[test]
fn test_BIT_ZP() {
    let mut _cpu = cpu_prep(0x0100,"BIT", ZERO_PAGE);
    
    // bit 7 and 6
    _cpu.a = 0xFF;
    _cpu.setmem(0x0101,0x55);
    _cpu.setmem(0x55,0xC0);
    _cpu.step();
    t(&_cpu, _cpu.ps & 0xC0 == 0xC0, format!("ps: {:08b}, should be 0xB0",_cpu.ps).as_ref());

    // bit 7
    _cpu.reset();
    _cpu.a = 0xFF;
    _cpu.setmem(0x0101,0x56);
    _cpu.setmem(0x56,0x80);
    _cpu.step();
    t(&_cpu, _cpu.ps & 0xC0 == 0x80, format!("ps: {:08b}, should be 0x80",_cpu.ps).as_ref());

    // bit 6
    _cpu.reset();
    _cpu.a = 0xFF;
    _cpu.setmem(0x0101,0x57);
    _cpu.setmem(0x57,0x40);
    _cpu.step();
    t(&_cpu, _cpu.ps & 0xC0 == 0x40, format!("ps: {:08b}, should be 0x40",_cpu.ps).as_ref());
}

#[test]
fn test_BIT_ABS() {
    let mut _cpu = cpu_prep(0x0100,"BIT",ABSOLUTE);
    
    // bit 7 and 6
    _cpu.a = 0xFF;
    _cpu.setmem(0x0101,0x55);
    _cpu.setmem(0x0102,0x44);
    _cpu.setmem(0x4455,0xC0);
    _cpu.step();
    t(&_cpu, _cpu.ps & 0xC0 == 0xC0, format!("ps: {:08b}, should be 0xB0",_cpu.ps).as_ref());

    // bit 7
    _cpu.reset();
    _cpu.a = 0xFF;
    _cpu.setmem(0x0101,0x56);
    _cpu.setmem(0x0102,0x45);
    _cpu.setmem(0x4556,0x80);
    _cpu.step();
    t(&_cpu, _cpu.ps & 0xC0 == 0x80, format!("ps: {:08b}, should be 0x80",_cpu.ps).as_ref());

    // bit 6
    _cpu.reset();
    _cpu.a = 0xFF;
    _cpu.setmem(0x0101,0x57);
    _cpu.setmem(0x0102,0x46);
    _cpu.setmem(0x4657,0x40);
    _cpu.step();
    t(&_cpu, _cpu.ps & 0xC0 == 0x40, format!("ps: {:08b}, should be 0x40",_cpu.ps).as_ref());
}