
struct MOS_6502<'la> {
    x: u8,
    y: u8,
    a: u8,
    sp: u8,
    pc: u16,
    ps: u8,
    bus: MMU,
    isa: Vec<&'la dyn Fn(&mut Self)>,
    cycles: u8,
}
impl<'la> MOS_6502<'la> {
    fn new() -> Result<MOS_6502<'la>,String> {
        let x: u8 = 0;
        let y: u8 = 0;
        let a: u8 = 0;
        let sp: u8 = 0xFF;
        let pc: u16 = 0xFFFC;
        let ps: u8 = 0;
        let bus = MMU::new();
        let cycles = 0;
        let isa = vec![MOS_6502::invalid, 256];
        let ret = MOS_6502 { x, y, a, sp, pc, ps, bus, isa, cycles };
        ret.load_isa();
        Ok()
    }
    fn step(&mut self) -> Result<(),String> {
        if --self.cycles > 0 {
            Ok(())
        }
        self.isa[self.pc++]();
        Ok(())
    }
}


