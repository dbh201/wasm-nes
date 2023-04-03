use crate::tests::*;

#[test]
fn test_AND_IMM() {
    let mut _cpu = cpu_prep(0x0800,"AND", IMMEDIATE);
    for a in 0..=255 {
        for imm in 0..=255 {
            _cpu.reset();
            _cpu.a = a;
            _cpu.setmem(0x0801, imm);
            _cpu.step();
            AND_assert(&_cpu, a, imm);
        }
    }
}
#[test]
fn test_AND_ZP() {
    let mut _cpu = cpu_prep(0x0800,"AND", ZERO_PAGE);
    for a in 0..=255 {
        for imm in 0..=255 {
            _cpu.reset();
            _cpu.a = a;
            _cpu.setmem(0x0801, imm);
            _cpu.setmem(imm as u16, imm);
            _cpu.step();
            AND_assert(&_cpu, a, imm);
        }
    }

}
#[test]
fn test_AND_ZP_X() {
    let mut _cpu = cpu_prep(0x0800,"AND", ZERO_PAGE_X);
    for a in 0..=255 {
        for imm in 0..=255 {
            _cpu.reset();
            _cpu.a = a;
            _cpu.x = imm;
            _cpu.setmem(0x0801, a);
            _cpu.setmem( ((imm as u16) + (a as u16)) % 256, imm);
            _cpu.step();
            AND_assert(&_cpu, a, imm);
        }
    }

}
#[test]
fn test_AND_ABS() {
    let mut _cpu = cpu_prep(0x0800,"AND", ABSOLUTE);
    for a in 0..=255 {
        for imm in 8..=245 {
            let addr = ((255-imm as u16)<<8) + (255-a as u16);
            _cpu.reset();
            _cpu.a = a;
            _cpu.setmem(0x0801, (addr%256) as u8);
            _cpu.setmem(0x0802, (addr>>8) as u8);
            _cpu.setmem(addr , imm);
            _cpu.step();
            AND_assert(&_cpu, a, imm);
        }
    }
}
#[test]
fn test_AND_ABS_X() {
    let mut _cpu = cpu_prep(0x0800,"AND", ABSOLUTE_X);
    for a in 0..=255 {
        for imm in 8..=245 {
            let addr = ((255-imm as u16)<<8) + (255-a as u16);
            let final_addr = addr + (255-imm as u16);
            _cpu.reset();
            _cpu.a = a;
            _cpu.x = 255-imm;
            _cpu.setmem(0x0801, (addr%256) as u8);
            _cpu.setmem(0x0802, (addr>>8) as u8);
            _cpu.setmem(final_addr , imm);
            _cpu.step();
            AND_assert(&_cpu, a, imm);
        }
    }
}
#[test]
fn test_AND_ABS_Y() {
    let mut _cpu = cpu_prep(0x0800,"AND", ABSOLUTE_Y);
    for a in 0..=255 {
        for imm in 8..=245 {
            let addr = ((255-imm as u16)<<8) + (255-a as u16);
            let final_addr = addr + (255-imm as u16);
            _cpu.reset();
            _cpu.a = a;
            _cpu.y = 255-imm;
            _cpu.setmem(0x0801, (addr%256) as u8);
            _cpu.setmem(0x0802, (addr>>8) as u8);
            _cpu.setmem(final_addr , imm);
            _cpu.step();
            AND_assert(&_cpu, a, imm);
        }
    }
}

#[test]
fn test_AND_IND_X() {
    let mut _cpu = cpu_prep(0x0800,"AND", INDIRECT_X);
    for a in 0..=255 {
        for imm in 8..=245 {
            let addr = ((255-imm as u16)<<8) + (255-a as u16);
            let zp: u8 = (((a as u16) + (imm as u16))%256) as u8;
            _cpu.reset();
            _cpu.a = a;
            _cpu.x = imm;
            _cpu.setmem(0x0801, a);

            _cpu.setmem(zp as u16, (addr % 256) as u8);
            _cpu.setmem(((zp as u16) + 1) % 256, (addr >> 8) as u8);
            _cpu.setmem( addr, imm);
            _cpu.step();
            AND_assert(&_cpu, a, imm);
        }
    }
}

#[test]
fn test_AND_IND_Y() {
    let mut _cpu = cpu_prep(0x0800,"AND", INDIRECT_Y);
    for a in 0..=255 {
        for imm in 8..=245 {
            let addr = ((255-imm as u16)<<8) + (255-a as u16);
            let y_offset = 255-imm;
            _cpu.reset();
            _cpu.a = a;
            _cpu.y = y_offset;
            _cpu.setmem(0x0801, a);

            _cpu.setmem(a as u16, (addr % 256) as u8);
            _cpu.setmem(((a as u16) + 1) % 256, (addr >> 8) as u8);
            _cpu.setmem( addr + (y_offset as u16), imm);
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
        format!("expected {}&{} = {}, got {}", a,imm, val, _cpu.a).as_str()
    );
    nzf(_cpu, val);
}
