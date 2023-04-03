use crate::tests::*;
#[test]
fn test_STA_ZP() { 
    let mut _cpu = cpu_prep(0,"STA",ZERO_PAGE);
    _cpu.setmem(1,255);
    _cpu.a = 0x88;
    _cpu.step();
    let res = _cpu.getmem(255);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STA_ZP_X() { 
    let mut _cpu = cpu_prep(0,"STA",ZERO_PAGE_X);
    _cpu.setmem(1,255);
    _cpu.x = 255;
    _cpu.a = 0x88;
    _cpu.step();
    let res = _cpu.getmem(254);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STA_ABS() { 
    let mut _cpu = cpu_prep(0,"STA",ABSOLUTE);
    _cpu.setmem(1,0xCD);
    _cpu.setmem(2,0xAB);
    _cpu.a = 0x88;
    _cpu.step();
    let res = _cpu.getmem(0xABCD);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STA_ABS_X() { 
    let mut _cpu = cpu_prep(0,"STA",ABSOLUTE_X);
    _cpu.setmem(1,0xCD);
    _cpu.setmem(2,0xAB);
    _cpu.a = 0x88;
    _cpu.x = 0x22;
    _cpu.step();
    let res = _cpu.getmem(0xABEF);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STA_ABS_Y() { 
    let mut _cpu = cpu_prep(0,"STA",ABSOLUTE_Y);
    _cpu.setmem(1,0xCD);
    _cpu.setmem(2,0xAB);
    _cpu.a = 0x88;
    _cpu.y = 0x22;
    _cpu.step();
    let res = _cpu.getmem(0xABEF);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STA_IND_X() { 
    let mut _cpu = cpu_prep(0,"STA",INDIRECT_X);
    _cpu.setmem(1,0xAB);
    _cpu.setmem(0xCD,0x34);
    _cpu.setmem(0xCE,0x12);
    _cpu.a = 0x88;
    _cpu.x = 0x22;
    _cpu.step();
    let res = _cpu.getmem(0x1234);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}
#[test]
fn test_STA_IND_Y() { 
    let mut _cpu = cpu_prep(0,"STA",INDIRECT_Y);
    _cpu.setmem(1,0xAB);
    _cpu.setmem(0xAB,0x23);
    _cpu.setmem(0xAC,0x12);
    _cpu.a = 0x88;
    _cpu.y = 0x11;
    _cpu.step();
    let res = _cpu.getmem(0x1234);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}