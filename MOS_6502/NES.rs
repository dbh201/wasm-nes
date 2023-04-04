struct NES {
    cpu: Mos6502,
}
impl NES {
    pub fn new() -> NES {
        let mut cpu = Mos6502::new();
        let mut ram = MmioNode::new("64KB RAM".to_string(), 0, 0x07FF);
        ram.make_ram().expect("Couldn't initialize RAM MmioNode:");
        ram.change_addr_range(0,0x1FFF);
        ram.enable_mirror(true);
        cpu.bus.register_MmioNode(ram).expect("Couldn't register RAM MmioNode:");
        NES { cpu }
    }
}
        