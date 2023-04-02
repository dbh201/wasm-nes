use MOS_6502_ISA::Mos6502Isa;
use MOS_6502_DEBUG::Mos6502Debug;
use MMU::MMU;
use std::fmt;

pub struct Mos6502<'la> {
    pub(crate) x: u8,
    pub(crate) y: u8,
    pub(crate) a: u8,
    pub(crate) sp: u8,
    pub(crate) pc: u16,
    pub(crate) ps: u8,
    pub(crate) bus: MMU<'la>,
    pub(crate) isa: Vec<&'la dyn Fn(&mut Self)>,
    pub(crate) cycles: u8,
    pub(crate) debug: Mos6502Debug<'la>
}
impl<'la> Mos6502<'la> {
    pub fn new() -> Result<Mos6502<'la>,String> {
        let x: u8 = 0;
        let y: u8 = 0;
        let a: u8 = 0;
        let sp: u8 = 0xFF;
        let pc: u16 = 0xFFFC;
        let ps: u8 = 0;
        let bus = MMU::new().unwrap();
        let cycles = 1;
        let mut isa = Vec::<&'la dyn Fn(&mut Self)>::new();
        let debug = Mos6502Debug::new();
        for i in 0..256 {
            isa.push(&Mos6502::invalid);
        }
        let mut ret = Mos6502 { x, y, a, sp, pc, ps, bus, isa, cycles, debug };
        ret.load_isa();
        Ok(ret)
    }
    pub fn peek_opcode(&self) -> u8 {
        self.bus.get(self.pc).unwrap()
    }
    pub fn step(&mut self) -> Result<(),String> {
        // If cycles == 0, the means the previous instruction is complete
        if self.cycles == 0 {
            let opcode = self.peek_opcode() as usize;
            self.pc += 1;
            let func = self.isa[opcode];
            func(self);
        }
        self.cycles -= 1;
        Ok(())
    }
    fn invalid(&mut self) {
        println!("Invalid opcode detected");
    }
}

impl fmt::Display for Mos6502<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let inst = self.peek_opcode();
        
        write!(f, "A: {:X?}\tX: {:X?}\tY: {:X?}\tSP: {:X?}\tPS: {:X?}\nPC: {:X?}\t{} {}\nCycles left: {}",
            self.a,
            self.x,
            self.y,
            self.sp,
            self.ps,
            self.pc,
            self.debug.getMnemonic(inst),
            self.debug.getAddrMode(inst),
            self.cycles
        )
    }
}

