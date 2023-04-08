use super::super::tests::*;
#[test]
fn test_STX_ZP() { 
    let mut _cpu = cpu_prep(0,"STX",ZERO_PAGE);
    _cpu.setmem(1,255);
    _cpu.x = 0x88;
    _cpu.step();
    let res = _cpu.getmem(255).unwrap();
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STX_ZP_Y() { 
    let mut _cpu = cpu_prep(0,"STX",ZERO_PAGE_Y);
    _cpu.setmem(1,255);
    _cpu.y = 255;
    _cpu.x = 0x88;
    _cpu.step();
    let res = _cpu.getmem(254).unwrap();
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STX_ABS() { 
    let mut _cpu = cpu_prep(0,"STX",ABSOLUTE);
    _cpu.setmem(1,0xCD);
    _cpu.setmem(2,0xAB);
    _cpu.x = 0x88;
    _cpu.step();
    let res = _cpu.getmem(0xABCD).unwrap();
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}