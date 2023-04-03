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
    _cpu.jmp_ind_bug = false;
    _cpu.setmem(0x0801,0xFF);
    _cpu.setmem(0x0802,0x8F);
    _cpu.setmem(0x8FFF,0xEF);
    _cpu.setmem(0x9000,0xCD);
    _cpu.step();   
    t(&_cpu, _cpu.pc == 0xCDEF, format!("[NOJMPBUG] pc was {:X}, != {:X}",_cpu.pc,0xCDEF).as_ref());
    _cpu.reset();
    _cpu.jmp_ind_bug = true;
    _cpu.setmem(0x0801,0xFF);
    _cpu.setmem(0x0802,0x7F);
    _cpu.setmem(0x7FFF,0xAB);
    _cpu.setmem(0x7F00,0xCD);
    _cpu.step();   
    t(&_cpu, _cpu.pc == 0xCDAB, format!("[JMPBUG] pc was {:X}, != {:X}",_cpu.pc,0xCDAB).as_ref());
}