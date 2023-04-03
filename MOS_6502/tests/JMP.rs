use crate::tests::*;
#[test]
fn test_JMP_ABS() {
    let mut _cpu = cpu_prep(0x0800,"JMP",ABSOLUTE);
    _cpu.setmem(0x0801,0x65);
    _cpu.setmem(0x0802,0x87);
    _cpu.step();
    t(&_cpu,_cpu.pc == 0x8765,format!("{} != {}",_cpu.pc,0x8765).as_ref());
}
#[test]
fn test_JMP_IND() {
    let mut _cpu = cpu_prep(0x0800,"JMP",INDIRECT);
    _cpu.setmem(0x0801,0x65);
    _cpu.setmem(0x0802,0x87);
    _cpu.setmem(0x8765,0xEF);
    _cpu.setmem(0x8766,0xCD);
    _cpu.step();   
    t(&_cpu, _cpu.pc == 0xCDEF, format!("{} != {}",_cpu.pc,0x1234).as_ref());
}