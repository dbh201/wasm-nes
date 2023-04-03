use crate::tests::*;
#[test]
fn test_AND_IMM() {

    let mut _cpu = cpu_prep(0x0800,"AND", IMMEDIATE);
    let op = _cpu.getmem(0x0800);
    for a in 0..=255 {
        for imm in 0..=255 {
            _cpu.reset();
            _cpu.a = a;
            _cpu.pc = 0x0800;
            _cpu.setmem(0x0800, op);
            _cpu.setmem(0x0801, imm);
            _cpu.step();
            AND_assert(&_cpu, a, imm);
        }
    }
}
fn AND_assert(_cpu: &Mos6502, a: u8, imm: u8) {
    let val = a & imm;
    t(
        _cpu,
        _cpu.a == val,
        format!("expected a == {}", val).as_str()
    );
    nzf(_cpu, val);
}
