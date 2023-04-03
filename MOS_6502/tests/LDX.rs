use crate::tests::*;

#[test]
fn test_LDX_IMM() {
    let mut _cpu = cpu_prep(0, "LDX", IMMEDIATE);
    _cpu.setmem(1,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.x);
    t(&_cpu, _cpu.x == 0x88,format!("{} != {}",_cpu.x,0x88).as_ref());
}

#[test]
fn test_LDX_ZP() {
    let mut _cpu = cpu_prep(0, "LDX", ZERO_PAGE);
    _cpu.setmem(1,255);
    _cpu.setmem(255,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.x);
    t(&_cpu, _cpu.x == 0x88,format!("{} != {}",_cpu.x,0x88).as_ref());
}

#[test]
fn test_LDX_ZP_Y() {
    let mut _cpu = cpu_prep(0, "LDX", ZERO_PAGE_Y);
    _cpu.setmem(1,255);
    _cpu.y = 255;
    _cpu.setmem(254,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.x);
    t(&_cpu, _cpu.x == 0x88,format!("{} != {}",_cpu.x,0x88).as_ref());
}

#[test]
fn test_LDX_ABS() {
    let mut _cpu = cpu_prep(0, "LDX", ABSOLUTE);
    _cpu.setmem(1,0xBA);
    _cpu.setmem(2,0xAB);
    _cpu.setmem(0xABBA,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.x);
    t(&_cpu, _cpu.x == 0x88,format!("{} != {}",_cpu.x,0x88).as_ref());
}

#[test]
fn test_LDX_ABS_Y() {
    let mut _cpu = cpu_prep(0, "LDX", ABSOLUTE_Y);
    _cpu.y = 0x13;
    _cpu.setmem(1,0xBA);
    _cpu.setmem(2,0xAB);
    _cpu.setmem(0xABCD,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.x);
    t(&_cpu, _cpu.x == 0x88,format!("{} != {}",_cpu.x,0x88).as_ref());
}