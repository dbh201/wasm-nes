use crate::tests::*;

#[test]
fn test_LDA_IMM() {
    let mut _cpu = cpu_prep(0, "LDA", IMMEDIATE);
    _cpu.setmem(1,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.a);
    t(&_cpu, _cpu.a == 0x88,format!("{} != {}",_cpu.a, 0x88).as_ref());
}

#[test]
fn test_LDA_ZP() {
    let mut _cpu = cpu_prep(0, "LDA", ZERO_PAGE);
    _cpu.setmem(1,255);
    _cpu.setmem(255,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.a);
    t(&_cpu, _cpu.a == 0x88,format!("{} != {}",_cpu.a,0x88).as_ref());
}

#[test]
fn test_LDA_ZP_X() {
    let mut _cpu = cpu_prep(0, "LDA", ZERO_PAGE_X);
    _cpu.setmem(1,255);
    _cpu.x = 255;
    _cpu.setmem(254,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.a);
    t(&_cpu, _cpu.a == 0x88,format!("{} != {}",_cpu.a,0x88).as_ref());
}
#[test]
fn test_LDA_ABS() {
    let mut _cpu = cpu_prep(0, "LDA", ABSOLUTE);
    _cpu.setmem(1,0xBA);
    _cpu.setmem(2,0xAB);
    _cpu.setmem(0xABBA,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.a);
    t(&_cpu, _cpu.a == 0x88,format!("{} != {}",_cpu.a,0x88).as_ref());
}

#[test]
fn test_LDA_ABS_X() {
    let mut _cpu = cpu_prep(0, "LDA", ABSOLUTE_X);
    _cpu.x = 0x13;
    _cpu.setmem(1,0xBA);
    _cpu.setmem(2,0xAB);
    _cpu.setmem(0xABCD,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.a);
    t(&_cpu, _cpu.a == 0x88,format!("{} != {}",_cpu.a,0x88).as_ref());
}
#[test]
fn test_LDA_ABS_Y() {
    let mut _cpu = cpu_prep(0, "LDA", ABSOLUTE_Y);
    _cpu.y = 0x13;
    _cpu.setmem(1,0xBA);
    _cpu.setmem(2,0xAB);
    _cpu.setmem(0xABCD,0x88);
    _cpu.step();
    nzf(&_cpu,_cpu.a);
    t(&_cpu, _cpu.a == 0x88,format!("{} != {}",_cpu.a,0x88).as_ref());
}