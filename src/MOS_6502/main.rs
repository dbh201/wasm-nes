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
    for _ in 0..3 {
        println!("{}\n\n",_cpu);
        _cpu.step();
    }
}
