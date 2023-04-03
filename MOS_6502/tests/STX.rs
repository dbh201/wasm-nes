use crate::tests::*;

#[test]
fn test_STX_ZP_Y() { 
    let mut _cpu = cpu_prep(0,"STX",ZERO_PAGE_Y);
    _cpu.setmem(1,255);
    _cpu.y = 255;
    _cpu.x = 0x88;
    _cpu.step();
    let res = _cpu.getmem(254);
    t(&_cpu, res == 0x88,format!("{} != {}",res,0x88).as_ref());
}