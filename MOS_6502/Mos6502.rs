use crate::Mos6502Isa::Mos6502Isa;
use crate::Mos6502Debug::Mos6502Debug;
use crate::MMU::MMU;
use std::fmt;

#[derive(Clone,Copy)]
#[repr(u8)]
pub enum Mos6502Flag {
    C = 0x01,
    Z = 0x02,
    I = 0x04,
    D = 0x08,
    B = 0x10,
    S = 0x20,
    V = 0x40,
    N = 0x80,
}
//TODO: not public everything here
pub struct Mos6502<'la> {
    pub jmp_ind_bug: bool,
    pub enable_bcd: bool,
    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub sp: u8,
    pub pc: u16,
    pub current_instruction: u16,
    pub ps: u8,
    pub bus: MMU,
    pub isa: Vec<&'la dyn Fn(&mut Self)>,
    pub cycles: u8,
    pub debug: Mos6502Debug<'la>
}


impl<'la> Mos6502<'la> {
    pub fn new() -> Result<Mos6502<'la>,String> {
        let enable_bcd = false;
        let jmp_ind_bug = false;
        let x: u8 = 0;
        let y: u8 = 0;
        let a: u8 = 0;
        let sp: u8 = 0xFF;
        let pc: u16 = 0xFFFC;
        let current_instruction: u16 = pc;
        let ps: u8 = 0;
        let bus = MMU::new().unwrap();
        let cycles = 0;
        let mut isa = Vec::<&'la dyn Fn(&mut Self)>::new();
        let debug = Mos6502Debug::new();
        for _ in 0..256 {
            isa.push(&Mos6502::invalid);
        }
        let mut ret = Mos6502 { x, y, a, sp, pc, ps, bus, isa, cycles, debug, current_instruction, enable_bcd, jmp_ind_bug };
        ret.load_isa();
        Ok(ret)
    }
    // Returns a u16 from memory location
    pub fn _fetch_u16(&self, addr: u16) -> u16 {
        //TODO: addr overflows should be handled in some way.
        let lo = self.getmem(addr);
        let ho = self.getmem(addr + 1);
        ((ho as u16)<<8) + (lo as u16)
    }
    pub fn _place_u16(&mut self, addr: u16, val: u16) {
        //TODO: addr overflows should be handled in some way.
        self.setmem(addr, (val % 256) as u8);
        self.setmem(addr + 1, (val >> 8) as u8);
    }
    pub fn flag(&self,f: Mos6502Flag) -> bool {
        self.ps & (f as u8) != 0
    }
    pub fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.a = 0;
        self.sp = 0xFF;
        self.pc = self._fetch_u16(0xFFFC);
        self.ps = 0;
        self.current_instruction = self.pc;
        self.cycles = 0;
    }
    pub fn peek_opcode(&self) -> u8 {
        self.bus.get(self.pc).unwrap()
    }
    pub fn step(&mut self) {
        self._step().expect("CPU failed to step?");
    }
    pub fn _step(&mut self) -> Result<(),String> {
        //finish previous instruction
        while self.cycles != 0 {
            self.clock_tick();
        }
        //start new instruction
        self._clock_tick()
    }
    pub fn clock_tick(&mut self) {
        self._clock_tick().expect("CPU clock tick failed?");
    }
    pub fn _clock_tick(&mut self) -> Result<(),String> {
        // If cycles == 0, the means the previous instruction is complete
        if self.cycles == 0 {
            let opcode = self.peek_opcode() as usize;
            self.current_instruction = self.pc;
            self.pc += 1;
            let func = self.isa[opcode];
            func(self);
        }
        if self.cycles > 0 {
            self.cycles -= 1;
            return Ok(())
        } else {
            return Err(format!("Cycles were not set by instruction:\n{}",self).replace("\\n", "\n"));
        }
        
        
    }
    pub fn setmem(&mut self, addr: u16, val: u8) {
        self.bus.set(addr,val).expect(format!("Mem write failed @{}={}",addr,val).as_ref())
    }
    pub fn getmem(&self, addr: u16) -> u8 {
        self.bus.get(addr).expect(format!("Mem read failed @{}",addr).as_ref())
    }
    fn invalid(&mut self) {
        println!("Invalid opcode detected");
    }

}

impl fmt::Display for Mos6502<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A: {:X?}\tX: {:X?}\tY: {:X?}\tSP: {:X?}\tPC: {:X?}\tPS: {:#010b}\n{}\nCycles until next instruction: {}",
            self.a,
            self.x,
            self.y,
            self.sp,
            self.pc,
            self.ps,
            self.debug.current_instruction_info(self),
            self.cycles
        )
    }
}


