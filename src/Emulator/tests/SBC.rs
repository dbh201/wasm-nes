use super::super::tests::*;

//TODO: not confident on these assertions, or SBC as a whole
fn SBC_assert(_cpu: &Mos6502, a: u8, imm: u8, carry: u8) {
    let val = (a as i16) - (imm as i16) - (1 - (carry as i16));
    t(
        _cpu,
        _cpu.a == (val % 256) as u8,
        format!(
            "expected a{} c{} i{}={}, got {}",
            a, carry, imm, val, _cpu.a
        )
        .as_ref(),
    );
    let carry_clear = val < 0;
    t(
        _cpu,
        carry_clear != _cpu.flag(Mos6502Flag::C),
        format!(
            "expected a{} c{} i{}={}, carry set {}, should be {}",
            a, carry, imm, (val as u16)%256, _cpu.flag(Mos6502Flag::C),!carry_clear
        )
        .as_ref()
    );
    let signed_overflow = val < -128 || val > 127;
    t(
        _cpu,
        signed_overflow == _cpu.flag(Mos6502Flag::V),
        format!(
            "expected a{} c{} i{}={}({}) overflow set {}, should be {}",
            a, carry, imm, val,(val as u16)%256, _cpu.flag(Mos6502Flag::C),!carry_clear
        )
        .as_ref()
    );
    nzf(_cpu, _cpu.a);
}
#[test]
fn test_SBC_IMM() {
    let mut _cpu = cpu_prep(0x0800, "SBC", IMMEDIATE);
    for a in 0..=255 {
        for imm in 0..=255 {
            for carry in [0, 1] {
                _cpu.reset();
                _cpu.a = a;
                _cpu.pc = 0x0800;
                _cpu.ps = carry;
                _cpu.setmem(0x0801, imm);
                _cpu.step();
                SBC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}
#[test]
fn test_SBC_ZP() {
    let mut _cpu = cpu_prep(0x0800, "SBC", IMMEDIATE);
    for a in 0..=255 {
        for imm in 0..=255 {
            for carry in [0, 1] {
                _cpu.reset();
                _cpu.a = a;
                _cpu.pc = 0x0800;
                _cpu.ps = carry;
                _cpu.setmem(0x0801, imm);
                _cpu.setmem(imm as u16, imm);
                _cpu.step();
                SBC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}
#[test]
fn test_SBC_ZP_X() {
    let mut _cpu = cpu_prep(0x0800, "SBC", ZERO_PAGE_X);
    for a in 0..=255 {
        for imm in 0..=255 {
            for carry in [0, 1] {
                _cpu.reset();
                _cpu.a = a;
                _cpu.x = a;
                _cpu.pc = 0x0800;
                _cpu.ps = carry;
                _cpu.setmem(0x0801, imm);
                _cpu.setmem(((imm as u16) + (a as u16)) % 256, imm);
                _cpu.step();
                SBC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}
#[test]
fn test_SBC_ABS() {
    let mut _cpu = cpu_prep(0x0800, "SBC", ABSOLUTE);
    let op = _cpu.getmem(0x0800).unwrap();
    for a in 0..=255 {
        for imm in 0..=255 {
            for carry in [0, 1] {
                let abs = ((a as u16) << 8) + (imm as u16);
                let load: u16;
                if abs >= 0x0800 && abs < 0x0803 {
                    load = 0x0900;
                } else {
                    load = 0x0800;
                }
                _cpu.reset();
                _cpu.a = a;
                _cpu.pc = load;
                _cpu.ps = carry;
                _cpu.setmem(load, op);
                _cpu.setmem(load + 1, imm);
                _cpu.setmem(load + 2, a);
                _cpu.setmem(abs, imm);
                _cpu.step();
                SBC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}

#[test]
fn test_SBC_ABS_X() {
    let mut _cpu = cpu_prep(0x0800, "SBC", ABSOLUTE_X);
    let op = _cpu.getmem(0x0800).unwrap();
    for a in 0..=254 {
        for imm in 0..=255 {
            for carry in [0, 1] {
                let abs = ((a as u16) << 8) + (imm as u16) + (a as u16);
                let load: u16;
                if abs >= 0x0800 && abs < 0x0803 {
                    load = 0x0900;
                } else {
                    load = 0x0800;
                }
                _cpu.reset();
                _cpu.a = a;
                _cpu.x = a;
                _cpu.pc = load;
                _cpu.ps = carry;
                _cpu.setmem(load, op);
                _cpu.setmem(load + 1, imm);
                _cpu.setmem(load + 2, a);
                _cpu.setmem(abs, imm);
                _cpu.step();
                SBC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}

#[test]
fn test_SBC_ABS_Y() {
    let mut _cpu = cpu_prep(0x0800, "SBC", ABSOLUTE_Y);
    let op = _cpu.getmem(0x0800).unwrap();
    for a in 0..=254 {
        for imm in 0..=255 {
            for carry in [0, 1] {
                let abs = ((a as u16) << 8) + (imm as u16) + (a as u16);
                let load: u16;
                if abs >= 0x0800 && abs < 0x0803 {
                    load = 0x0900;
                } else {
                    load = 0x0800;
                }
                _cpu.reset();
                _cpu.a = a;
                _cpu.y = a;
                _cpu.pc = load;
                _cpu.ps = carry;
                _cpu.setmem(load, op);
                _cpu.setmem(load + 1, imm);
                _cpu.setmem(load + 2, a);
                _cpu.setmem(abs, imm);
                _cpu.step();
                SBC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}

#[test]
fn test_SBC_IND_X() {
    let mut _cpu = cpu_prep(0x0800, "SBC", INDIRECT_X);
    let op = _cpu.getmem(0x0800).unwrap();
    for a in 0..=252 {
        for imm in 0..=255 {
            for carry in [0, 1] {
                let abs = 0x200 + ((a as u16) << 8) + (imm as u16) + (a as u16);
                let load: u16;
                if abs >= 0x0800 && abs < 0x0803 {
                    load = 0x0900;
                } else {
                    load = 0x0800;
                }
                _cpu.reset();
                _cpu.a = a;
                _cpu.x = a;
                _cpu.pc = load;
                _cpu.ps = carry;
                _cpu.setmem(load, op);
                _cpu.setmem(load + 1, imm);

                console_log!("Expect addr: {}",abs);
                _cpu.setmem(((imm as u16) + (a as u16)) % 256, (abs % 256) as u8);
                _cpu.setmem(((imm as u16) + (a as u16) + 1) % 256, (abs >> 8) as u8);
                _cpu.setmem(abs, imm);
                _cpu.step();
                SBC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}

#[test]
fn test_SBC_IND_Y() {
    let mut _cpu = cpu_prep(0x0800, "SBC", INDIRECT_Y);
    let op = _cpu.getmem(0x0800).unwrap();
    for a in 0..=252 {
        for imm in 0..=255 {
            for carry in [0, 1] {
                let abs = 0x200 + ((a as u16) << 8) + (imm as u16);
                console_log!("a{} c{} i{} @{}", a, carry, imm, abs);
                let load: u16;
                if abs + (a as u16) >= 0x0800 && abs + (a as u16) < 0x0803 {
                    load = 0x0900;
                } else {
                    load = 0x0800;
                }
                _cpu.reset();
                _cpu.a = a;
                _cpu.y = a;
                _cpu.pc = load;
                _cpu.ps = carry;
                _cpu.setmem(load, op);
                _cpu.setmem(load + 1, imm);

                _cpu.setmem(imm as u16, (abs % 256) as u8);
                _cpu.setmem((imm as u16 + 1) % 256, (abs >> 8) as u8);
                _cpu.setmem(abs + a as u16, imm);
                _cpu.step();
                SBC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}
