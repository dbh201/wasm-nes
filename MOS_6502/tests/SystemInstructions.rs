use crate::tests::*;

#[test]
fn test_BRK_RTI() {
    let mut _cpu = cpu_prep(0x0800, "BRK", IMPLIED);
    let op = _cpu.debug.get_opcode("RTI", IMPLIED);
    _cpu.setmem(0xFFFE, 0xEF);
    _cpu.setmem(0xFFFF, 0xBE);
    _cpu.setmem(0xBEEF, op);
    _cpu.ps = 0b11001000;
    _cpu.step();
    let ps = _cpu.getmem(0x01FD);
    let pc = _cpu._fetch_u16(0x01FE);
    t(&_cpu, _cpu.ps == ps | (Mos6502Flag::B as u8), format!("BRK flags: {:08b} != {:08b}", _cpu.ps, ps).as_ref());
    t(&_cpu, pc == 0x0801, format!("BRK pc: {:08b} != {:08b}", _cpu.pc, 0x0801).as_ref());
    _cpu.step();
    t(&_cpu, _cpu.pc == pc + 1, format!("RTI pc: {:04X} != {:04X}", _cpu.pc, pc + 1).as_ref());
}
// TODO: check the memory for changes too?
#[test]
fn test_NOP() {
    let mut _cpu = cpu_prep(0x0800, "NOP", IMPLIED);
    let x  = _cpu.x;
    let y  = _cpu.y;
    let a  = _cpu.a;
    let sp = _cpu.sp;
    let pc = _cpu.pc + 1;
    let ps = _cpu.ps;
    _cpu.step();
    t(&_cpu, _cpu.x == x, format!("x: {:04X} != {:04X}", _cpu.x, x).as_ref());
    t(&_cpu, _cpu.y == y, format!("y: {:04X} != {:04X}", _cpu.y, y).as_ref());
    t(&_cpu, _cpu.a == a, format!("a: {:04X} != {:04X}", _cpu.a, a).as_ref());
    t(&_cpu, _cpu.sp == sp, format!("sp: {:04X} != {:04X}", _cpu.sp, sp).as_ref());
    t(&_cpu, _cpu.pc == pc, format!("pc: {:04X} != {:04X}", _cpu.pc, pc).as_ref());
    t(&_cpu, _cpu.ps == ps, format!("ps: {:04X} != {:04X}", _cpu.ps, ps).as_ref());
}
