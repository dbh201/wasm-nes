use super::super::tests::*;

//TODO: test BCD addition
fn ADC_assert(_cpu: &Mos6502, a: u8, imm: u8, carry: u8) {
    let val = (a as u16) + (imm as u16) + (carry as u16);
    t(
        _cpu,
        _cpu.a == (val % 256) as u8,
        format!(
            "expected a{} c{} i{}={}, got {}",
            a, carry, imm, val, _cpu.a
        )
        .as_str(),
    );
    nzf(_cpu, _cpu.a);
    cf(_cpu, val);
}
#[test]
fn test_ADC_IMM() {
    let mut _cpu = cpu_prep(0x0800, "ADC", IMMEDIATE);
    for a in 0..=255 {
        for imm in 0..=255 {
            for carry in [0, 1] {
                _cpu.reset();
                _cpu.a = a;
                _cpu.pc = 0x0800;
                _cpu.ps = carry;
                _cpu.setmem(0x0801, imm);
                _cpu.step();
                ADC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}
#[test]
fn test_ADC_ZP() {
    let mut _cpu = cpu_prep(0x0800, "ADC", IMMEDIATE);
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
                ADC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}
#[test]
fn test_ADC_ZP_X() {
    let mut _cpu = cpu_prep(0x0800, "ADC", ZERO_PAGE_X);
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
                ADC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}
#[test]
fn test_ADC_ABS() {
    let mut _cpu = cpu_prep(0x0800, "ADC", ABSOLUTE);
    let op = _cpu.getmem(0x0800);
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
                ADC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}

#[test]
fn test_ADC_ABS_X() {
    let mut _cpu = cpu_prep(0x0800, "ADC", ABSOLUTE_X);
    let op = _cpu.getmem(0x0800);
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
                ADC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}

#[test]
fn test_ADC_ABS_Y() {
    let mut _cpu = cpu_prep(0x0800, "ADC", ABSOLUTE_Y);
    let op = _cpu.getmem(0x0800);
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
                ADC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}

#[test]
fn test_ADC_IND_X() {
    let mut _cpu = cpu_prep(0x0800, "ADC", INDIRECT_X);
    let op = _cpu.getmem(0x0800);
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
                ADC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}

#[test]
fn test_ADC_IND_Y() {
    let mut _cpu = cpu_prep(0x0800, "ADC", INDIRECT_Y);
    let op = _cpu.getmem(0x0800);
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
                ADC_assert(&_cpu, a, imm, carry);
            }
        }
    }
}
