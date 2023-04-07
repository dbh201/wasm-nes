use super::super::tests::*;
#[test]
fn test_STY_ZP() { 
    let mut _cpu = cpu_prep(0,"STY",ZERO_PAGE);
    _cpu.setmem(1,255);
    _cpu.y = 0x88;
    _cpu.step();
    let res = _cpu.getmem(255);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STY_ZP_X() { 
    let mut _cpu = cpu_prep(0,"STY",ZERO_PAGE_X);
    _cpu.setmem(1,255);
    _cpu.x = 255;
    _cpu.y = 0x88;
    _cpu.step();
    let res = _cpu.getmem(254);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STY_ABS() { 
    let mut _cpu = cpu_prep(0,"STY",ABSOLUTE);
    _cpu.setmem(1,0xCD);
    _cpu.setmem(2,0xAB);
    _cpu.y = 0x88;
    _cpu.step();
    let res = _cpu.getmem(0xABCD);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}