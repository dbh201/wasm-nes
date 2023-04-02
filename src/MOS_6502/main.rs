mod MOS_6502;
mod MOS_6502_ISA;
mod MOS_6502_DEBUG;
mod MMU;
mod MMIO_node;
use crate::MOS_6502::Mos6502;


fn main() {
    let mut _cpu = Mos6502::new().unwrap();
    println!("Initialised cpu.\n\n");

    _cpu.bus.set(0xFFFC,0x4C);
    _cpu.bus.set(0xFFFD,0x00);
    _cpu.bus.set(0xFFFE,0x08);
    println!("{:02X?} {:02X?} {:02X?}",
        _cpu.bus.get(0xFFFC).unwrap(),
        _cpu.bus.get(0xFFFD).unwrap(),
        _cpu.bus.get(0xFFFE).unwrap()
    );
    _cpu.bus.set(0x0800,0x4C);
    _cpu.bus.set(0x0801,0xFC);
    _cpu.bus.set(0x0802,0xFF);
    println!("{:02X?} {:02X?} {:02X?}",
        _cpu.bus.get(0x0800).unwrap(),
        _cpu.bus.get(0x0801).unwrap(),
        _cpu.bus.get(0x0802).unwrap()
    );
    for _ in 0..100 {
        println!("{}\n\n",_cpu);
        _cpu.step();
    }
}
