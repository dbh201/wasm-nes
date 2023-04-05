struct NES {
    cpu: Mos6502,
    ppu: PPU,
    apu: APU,
}
impl NES {
    pub fn new() -> NES {
        let mut bus = RefCell::new(MMU::new("NES MAINBUS"));
        let mut ppu_mmu = RefCell::new(MMU::new("NES PPUMMU"));
        let mut cpu = Mos6502::new("NES CPU",bus.borrow_mut());
        
        let mut ram = MmioNode::new("2KB RAM".to_string(), 0, 0x07FF);
        ram.make_ram().expect("NES RAM:");
        ram.change_addr_range(0,0x1FFF);
        ram.enable_mirror(true,0);
        bus.register_MmioNode(ram).expect("NES RAM:");

        let mut ppu = MmioNode::new("NES PPU".to_string());
        ppu.make_ppu(ppu_mmu.borrow_mut()).expect("NES PPU:");
        ppu.add_mirror_addr_range(0x2000, 0x3FFF, 8);
        ppu.add_addr_range(0x4014, 0); // 1 byte for DMA
        bus.register_MmioNode(ram).expect("NES PPU:");

        let mut apu = MmioNode::new("NES APU/Joystick".to_string());
        NES { cpu, ppu, apu }
    }
}
        