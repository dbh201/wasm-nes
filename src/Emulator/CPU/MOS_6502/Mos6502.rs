use super::Mos6502Isa::Mos6502Isa;
use super::Mos6502Debug::Mos6502Debug;
use super::super::super::Common::AddressBus::{AddressBus, MemRW};
use super::super::super::console_log;

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

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
    pub name: String,
    pub jmp_ind_bug: bool,
    pub enable_bcd: bool,
    pub x: u8,
    pub y: u8,
    pub a: u8,
    pub sp: u8,
    pub pc: u16,
    pub current_instruction: u16,
    pub ps: u8,
    pub bus: Rc<RefCell<AddressBus<'la>>>,
    pub isa: Vec<&'la dyn Fn(&mut Self)>,
    pub cycles: u8,
    pub debug: Mos6502Debug<'la>
}

impl<'la> Mos6502<'la> {
    pub fn new(name: String, bus: Rc<RefCell<AddressBus<'la>>>) -> Result<Mos6502<'la>,String> {
        let enable_bcd = false;
        let jmp_ind_bug = false;
        let x: u8 = 0;
        let y: u8 = 0;
        let a: u8 = 0;
        let sp: u8 = 0xFF;
        let pc: u16 = 0xFFFC; // This isn't correct, but there's no way to tell where the vector points to yet
        let current_instruction: u16 = pc;
        let ps: u8 = 0;
        let cycles = 0;
        let mut isa = Vec::<&'la dyn Fn(&mut Self)>::new();
        let debug = Mos6502Debug::new();
        for _ in 0..256 {
            isa.push(&Mos6502::invalid);
        }
        let mut ret = Mos6502 { name, x, y, a, sp, pc, ps, bus, isa, cycles, debug, current_instruction, enable_bcd, jmp_ind_bug };
        ret.load_isa();
        Ok(ret)
    }

    // Returns a u16 from memory location
    pub fn _fetch_u16(&self, addr: u16) -> Result<u16, String> {
        let lo = self.getmem(addr)?;
        let ho = self.getmem(addr + 1)?;
        Ok(((ho as u16)<<8) + (lo as u16))
    }
    pub fn _place_u16(&mut self, addr: u16, val: u16) -> Result<(), String>{
        //TODO: addr overflows should be handled in some way.
        self.setmem(addr, (val % 256) as u8)?;
        self.setmem(addr + 1, (val >> 8) as u8)?;
        Ok(())
    }
    pub fn flag(&self,f: Mos6502Flag) -> bool {
        self.ps & (f as u8) != 0
    }
    pub fn reset(&mut self) -> Result<(), String>{
        self.x = 0;
        self.y = 0;
        self.a = 0;
        self.sp = 0xFF;
        self.pc = self._fetch_u16(0xFFFC)?;
        self.ps = 0;
        self.current_instruction = self.pc;
        self.cycles = 0;
        Ok(())
    }
    pub fn peek_opcode(&self) -> Result<u8, String> {
        self.bus.borrow_mut().get(self.pc)
    }
    pub fn step(&mut self) {
        self._step().expect("CPU failed to step?");
    }
    pub fn _step(&mut self) -> Result<(),String> {
        //finish previous instruction
        while self.cycles != 0 {
            self.clock_tick()?;
        }
        //start new instruction
        self._clock_tick()
    }
    pub fn clock_tick(&mut self) -> Result<(), String> {
        self._clock_tick()
    }
    pub fn _clock_tick(&mut self) -> Result<(),String> {
        // If cycles == 0, the means the previous instruction is complete
        if self.cycles == 0 {
            let nmi = self.bus.borrow_mut().nmi_flag();
            if nmi {
                self._interrupt()?;
                self.bus.borrow_mut().clear_nmi();
                // TODO: should we run the interrupt instruction on this tick or the next?
                return Ok(())
            }
            let opcode = self.peek_opcode()? as usize;
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
    fn invalid(&mut self) {
        console_log!("Invalid opcode detected");
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
            self.debug.current_instruction_info(self).unwrap(),
            self.cycles
        )
    }
}
impl MemRW for Mos6502<'_> {
    fn setmem(&mut self, addr: u16, val: u8) -> Result<(), String> {
        let resp = self.bus.borrow_mut().set(addr,val);
        if resp.is_err() {
            console_log!("CPU MAINBUS: {}",resp.clone().err().unwrap());
        }
        return resp
    }
    fn getmem(&self, addr: u16) -> Result<u8, String> {
        let resp = self.bus.borrow_mut().get(addr);
        if resp.is_err() {
            console_log!("CPU MAINBUS: {}",resp.clone().err().unwrap());
        }
        return resp
    }
}


