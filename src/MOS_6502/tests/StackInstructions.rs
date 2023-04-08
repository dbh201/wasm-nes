use super::super::tests::*;

#[test]
fn test_PHA() {
    let mut _cpu = cpu_prep(0,"PHA", IMPLIED);
    _cpu.a = 0x99;
    _cpu.step();
    let res = _cpu.getmem(0x01FF).unwrap();
    t(&_cpu, _cpu.a == res,format!("a{} != m{}",_cpu.a,res).as_ref());
}
#[test]
fn test_PHP() {
    let mut _cpu = cpu_prep(0,"PHP", IMPLIED);
    _cpu.ps = 0x99;
    _cpu.step();
    let res = _cpu.getmem(0x01FF).unwrap();
    t(&_cpu, _cpu.ps == res,format!("ps{} != m{}",_cpu.ps,res).as_ref());
}
#[test]
fn test_PLA() {
    let mut _cpu = cpu_prep(0,"PLA", IMPLIED);
    _cpu.setmem(0x01FF,0x99);
    _cpu.sp -= 1;
    _cpu.step();
    t(&_cpu, _cpu.a == 0x99,format!("a{} != m{}",_cpu.a,0x99).as_ref());
}
#[test]
fn test_PLP() {
    let mut _cpu = cpu_prep(0,"PLP", IMPLIED);
    _cpu.setmem(0x01FF,0x99);
    _cpu.sp -= 1;
    _cpu.step();
    t(&_cpu, _cpu.ps == 0x99,format!("ps{} != m{}",_cpu.ps,0x99).as_ref());
}