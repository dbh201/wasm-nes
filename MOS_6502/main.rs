#![allow(non_snake_case)]
use MOS_6502::Mos6502::Mos6502;


fn main() {
    let mut _cpu = Mos6502::new().unwrap();
    println!("Initialised cpu.\n\n");

    _cpu.setmem(0xFFFC, 0x4C);
    _cpu.setmem(0xFFFD, 0x00);
    _cpu.setmem(0xFFFE, 0x08);
    println!(
        "{:02X?} {:02X?} {:02X?}",
        _cpu.bus.get(0xFFFC).unwrap(),
        _cpu.bus.get(0xFFFD).unwrap(),
        _cpu.bus.get(0xFFFE).unwrap()
    );
    _cpu.setmem(0x0800, 0x4C);
    _cpu.setmem(0x0801, 0xFC);
    _cpu.setmem(0x0802, 0xFF);
    println!(
        "{:02X?} {:02X?} {:02X?}",
        _cpu.bus.get(0x0800).unwrap(),
        _cpu.bus.get(0x0801).unwrap(),
        _cpu.bus.get(0x0802).unwrap()
    );
    for _ in 0..100 {
        println!("{}\n\n", _cpu);
        _cpu.clock_tick();
    }
}

 
