use crate::tests::*;
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