use super::super::tests::*;

#[test]
fn test_TSX() {
    let mut _cpu = cpu_prep(0x0800, "TSX", IMPLIED);
    _cpu.sp = 0x64;
    _cpu.step();
    t(&_cpu,_cpu.x == _cpu.sp,format!("x{:X} != sp{:X}",_cpu.sp,_cpu.x).as_ref());
}
#[test]
fn test_TXS() {
    let mut _cpu = cpu_prep(0x0800, "TXS", IMPLIED);
    _cpu.x = 0x64;
    _cpu.step();
    t(&_cpu,_cpu.x == _cpu.sp,format!("x{:X} != sp{:X}",_cpu.sp,_cpu.x).as_ref());
}
#[test]
fn test_TAX() {
    let mut _cpu = cpu_prep(0x0800, "TAX", IMPLIED);
    _cpu.a = 0x64;
    _cpu.step();
    t(&_cpu,_cpu.x == _cpu.a,format!("x{:X} != sp{:X}",_cpu.a,_cpu.x).as_ref());
}
#[test]
fn test_TXA() {
    let mut _cpu = cpu_prep(0x0800, "TXA", IMPLIED);
    _cpu.x = 0x64;
    _cpu.step();
    t(&_cpu,_cpu.x == _cpu.a,format!("x{:X} != sp{:X}",_cpu.a,_cpu.x).as_ref());
}
#[test]
fn test_TAY() {
    let mut _cpu = cpu_prep(0x0800, "TAY", IMPLIED);
    _cpu.a = 0x64;
    _cpu.step();
    t(&_cpu, _cpu.y == _cpu.a, format!("x{:X} != sp{:X}",_cpu.a,_cpu.y).as_ref());
}
#[test]
fn test_TYA() {
    let mut _cpu = cpu_prep(0x0800, "TYA", IMPLIED);
    _cpu.y = 0x64;
    _cpu.step();
    t(&_cpu, _cpu.y == _cpu.a, format!("x{:X} != sp{:X}",_cpu.a,_cpu.y).as_ref());
}