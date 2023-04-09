use std::collections::hash_map::HashMap;
use super::super::super::console_log;

use super::Mos6502::Mos6502;
use super::super::super::Common::AddressBus::MemRW;

fn even(v: u8) -> bool {
    v & 0x01 == 0
}
fn odd(v: u8) -> bool {
    !even(v)
}
fn within(v: u8,l: u8, h: u8) -> bool {
    v >= l && v <= h
}

#[allow(non_camel_case_types)]
#[derive(Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum AddrMode {
    ABSOLUTE,
    ABSOLUTE_X,
    ABSOLUTE_Y,
    ACCUMULATOR,
    ERROR,
    IMMEDIATE,
    IMPLIED,
    INDIRECT,
    INDIRECT_X,
    INDIRECT_Y,
    RELATIVE,
    ZERO_PAGE,
    ZERO_PAGE_X,
    ZERO_PAGE_Y
}

pub struct Mos6502Debug<'a> {
    mnemonic: HashMap<u8,&'a str>,
    addr_mode: HashMap<AddrMode,&'a str>
}
impl<'a> Mos6502Debug<'a> {
    pub fn new() -> Mos6502Debug<'a> {
        let mut ret = Mos6502Debug {
            mnemonic: HashMap::new(),
            addr_mode: HashMap::new()
        };
        ret._load_addr_modes();
        ret._load_mnemonics();
        ret
    }
    pub fn get_mnemonic(&self, op: u8) -> &'a str {
        self.mnemonic[&op]
    }
    
    pub fn current_instruction_info(&self, cpu: &Mos6502) -> Result<String, String> {
        let addr = cpu.current_instruction;
        let op = cpu.getmem(addr)?;
        let mode = Mos6502Debug::<'_>::_get_addr_mode(op);
        let mn = self.mnemonic[&op];
        let param: String;
        match mode {
            AddrMode::ABSOLUTE    => {
                let p = cpu._fetch_u16(addr+1)? + cpu.x as u16;
                param = format!("{:04X?} -> {:02X?}",p,cpu.getmem(p));
            },
            AddrMode::ABSOLUTE_X  => {
                let p = cpu._fetch_u16(addr+1)?;
                param = format!("{:04X?} + X:{:02X?} -> {:02X?}",p,cpu.x,cpu.getmem(p + cpu.x as u16));
            },
            AddrMode::ABSOLUTE_Y  => {
                let p = cpu._fetch_u16(addr+1)?;
                param = format!("{:04X?} + Y:{:02X?} -> {:02X?}",p,cpu.y,cpu.getmem(p + cpu.y as u16));
            },
            AddrMode::ACCUMULATOR => param = format!("A: {:02X?}",cpu.a),
            AddrMode::ERROR       => param = format!("INVALID OPCODE"),
            AddrMode::IMMEDIATE   => param = format!("{:02X?}",cpu.getmem(addr+1)),
            AddrMode::IMPLIED     => param = format!(""),
            AddrMode::INDIRECT    => {
                let t = cpu._fetch_u16(addr+1)?;
                let i: u16;
                if cpu.jmp_ind_bug {
                    i = cpu.getmem(t)? as u16 + ((cpu.getmem((t & 0xFF00) + ( ((t&0x0FFF)+1) & 0xFF))? as u16)<<8);
                } else {
                    i = cpu._fetch_u16(t)?;
                }
                param = format!("({:04X?}) -> {:04X?}",t,i);
            },
            AddrMode::INDIRECT_X  => {
                let t = cpu.getmem(addr+1)? as u16;
                let ind: u16 = cpu._fetch_u16((t + (cpu.x as u16))%256)?;
                param = format!("({:04X?}) + X:{:02X?} -> {:04X?}[{:02X?}]",t,cpu.x,ind,cpu.getmem(ind));
            },
            AddrMode::INDIRECT_Y  => {
                let t = cpu._fetch_u16(addr+1)?;
                let ind: u16 = cpu._fetch_u16(t)? + (cpu.y as u16);
                param = format!("({:04X?}) + Y:{:02X?} -> {:04X?}[{:02X?}]",t,cpu.y,ind,cpu.getmem(ind));
            },
            AddrMode::RELATIVE    => {
                let offset = cpu.getmem(addr+1)? as i8;
                let res: i16 = ((addr + 2) as i16) + offset as i16;
                param = format!("({:04X?}) + {} -> {:04X?}",addr+2,offset, res as u16);
            }
            AddrMode::ZERO_PAGE   => {
                let t = cpu.getmem(addr+1)?;
                param = format!("{:02X?} = {:04X?}", t, t as u16);
            }
            AddrMode::ZERO_PAGE_X => {
                let t = cpu.getmem(addr+1)?;
                param = format!("{:02X?} + X:{:02X?} = {:04X?}", t, cpu.x, ((t as u16) + (cpu.x as u16))%256);
            }
            AddrMode::ZERO_PAGE_Y => {
                let t = cpu.getmem(addr+1)?;
                param = format!("{:02X?} + Y:{:02X?} = {:04X?}", t, cpu.y, ((t as u16) + (cpu.y as u16))%256);
            }
        }
        Ok(format!("[{:04X?}]:{:02X?}   {} {} ({})",addr,op,mn,param,self.addr_mode[&mode]))
    }
    
    fn _load_addr_modes(&mut self) {
        self.addr_mode.insert(AddrMode::ABSOLUTE,"ABS");
        self.addr_mode.insert(AddrMode::ABSOLUTE_X,"ABSX");
        self.addr_mode.insert(AddrMode::ABSOLUTE_Y,"ABSY");
        self.addr_mode.insert(AddrMode::ACCUMULATOR,"ACC");
        self.addr_mode.insert(AddrMode::ERROR,"ERR");
        self.addr_mode.insert(AddrMode::IMMEDIATE,"IMM");
        self.addr_mode.insert(AddrMode::IMPLIED,"IMPL");
        self.addr_mode.insert(AddrMode::INDIRECT,"IND");
        self.addr_mode.insert(AddrMode::INDIRECT_X,"INDX");
        self.addr_mode.insert(AddrMode::INDIRECT_Y,"INDY");
        self.addr_mode.insert(AddrMode::RELATIVE,"REL");
        self.addr_mode.insert(AddrMode::ZERO_PAGE,"ZP");
        self.addr_mode.insert(AddrMode::ZERO_PAGE_X,"ZPX");
        self.addr_mode.insert(AddrMode::ZERO_PAGE_Y,"ZPY");
    }
    fn _load_mnemonics(&mut self) {
        self.mnemonic.insert(0x10, "BPL");
        self.mnemonic.insert(0x30, "BMI");
        self.mnemonic.insert(0x50, "BVC");
        self.mnemonic.insert(0x70, "BVS");
        self.mnemonic.insert(0x90, "BCC");
        self.mnemonic.insert(0xB0, "BCS");
        self.mnemonic.insert(0xD0, "BNE");
        self.mnemonic.insert(0xF0, "BEQ");
        self.mnemonic.insert(0x18, "CLC");
        self.mnemonic.insert(0x38, "SEC");
        self.mnemonic.insert(0x58, "CLI");
        self.mnemonic.insert(0x78, "SEI");
        self.mnemonic.insert(0xB8, "CLV");
        self.mnemonic.insert(0xD8, "CLD");
        self.mnemonic.insert(0xF8, "SED");
        self.mnemonic.insert(0xAA, "TAX");
        self.mnemonic.insert(0x8A, "TXA");
        self.mnemonic.insert(0xCA, "DEX");
        self.mnemonic.insert(0xE8, "INX");
        self.mnemonic.insert(0xA8, "TAY");
        self.mnemonic.insert(0x98, "TYA");
        self.mnemonic.insert(0x88, "DEY");
        self.mnemonic.insert(0xC8, "INY");
        self.mnemonic.insert(0x9A, "TXS");
        self.mnemonic.insert(0xBA, "TSX");
        self.mnemonic.insert(0x48, "PHA");
        self.mnemonic.insert(0x68, "PLA");
        self.mnemonic.insert(0x08, "PHP");
        self.mnemonic.insert(0x28, "PLP");
        self.mnemonic.insert(0x00, "BRK");
        self.mnemonic.insert(0x20, "JSR");
        self.mnemonic.insert(0x40, "RTI");
        self.mnemonic.insert(0x60, "RTS");
        self.mnemonic.insert(0xEA, "NOP");

        for op in [0x69,0x65,0x75,0x6D,0x7D,0x79,0x61,0x71] {
            self.mnemonic.insert(op, "ADC");
        }
        for op in [0x29,0x25,0x35,0x2D,0x3D,0x39,0x21,0x31] {
            self.mnemonic.insert(op, "AND");
        }
        for op in [0x0A,0x06,0x16,0x0E,0x1E] {
            self.mnemonic.insert(op, "ASL");
        }
        for op in [0x24,0x2C] {
            self.mnemonic.insert(op, "BIT");
        }
        for op in [0xC9,0xC5,0xD5,0xCD,0xDD,0xD9,0xC1,0xD1] {
            self.mnemonic.insert(op, "CMP");
        }
        for op in [0xE0,0xE4,0xEC] {
            self.mnemonic.insert(op, "CPX");
        }
        for op in [0xC0,0xC4,0xCC] {
            self.mnemonic.insert(op, "CPY");
        }
        for op in [0xC6,0xD6,0xCE,0xDE] {
            self.mnemonic.insert(op, "DEC");
        }
        for op in [0x49,0x45,0x55,0x4D,0x5D,0x59,0x41,0x51] {
            self.mnemonic.insert(op, "EOR");
        }
        for op in [0xE6,0xF6,0xEE,0xFE] {
            self.mnemonic.insert(op, "INC");
        }
        for op in [0x4C,0x6C] {
            self.mnemonic.insert(op, "JMP");
        }
        for op in [0xA9,0xA5,0xB5,0xAD,0xBD,0xB9,0xA1,0xB1] {
            self.mnemonic.insert(op, "LDA");
        }
        for op in [0xA2,0xA6,0xB6,0xAE,0xBE] {
            self.mnemonic.insert(op, "LDX");
        }
        for op in [0xA0,0xA4,0xB4,0xAC,0xBC] {
            self.mnemonic.insert(op, "LDY");
        }
        for op in [0x4A,0x46,0x56,0x4E,0x5E] {
            self.mnemonic.insert(op, "LSR");
        }
        for op in [0x09,0x05,0x15,0x0D,0x1D,0x19,0x01,0x11] {
            self.mnemonic.insert(op, "ORA");
        }
        for op in [0x2A,0x26,0x36,0x2E,0x3E] {
            self.mnemonic.insert(op, "ROL");
        }
        for op in [0x6A,0x66,0x76,0x6E,0x7E] {
            self.mnemonic.insert(op, "ROR");
        }
        for op in [0xE9,0xE5,0xF5,0xED,0xFD,0xF9,0xE1,0xF1] {
            self.mnemonic.insert(op, "SBC");
        }
        for op in [0x85,0x95,0x8D,0x9D,0x99,0x81,0x91] {
            self.mnemonic.insert(op, "STA");
        }
        for op in [0x86,0x96,0x8E] {
            self.mnemonic.insert(op, "STX");
        }
        for op in [0x84,0x94,0x8C] {
            self.mnemonic.insert(op, "STY");
        }
    }

    // This is comparatively slow, but should only be used during testing...
    // so...
    pub fn get_opcode(&self, mnemonic: &str, mode: AddrMode) -> u8 {
        for (key,value) in &self.mnemonic {
            if value.eq(&mnemonic) {
                if Mos6502Debug::<'_>::_get_addr_mode(*key) == mode {
                    console_log!("OK: {} {:X}({} == {})",mnemonic,*key, self.get_addr_mode(*key), self.addr_mode[&mode]);
                    return *key
                }
                console_log!("{} {:X}({} != {})",mnemonic,*key, self.get_addr_mode(*key), self.addr_mode[&mode]);
            }
        }
        console_log!("could not find opcode {} {}",mnemonic,self.addr_mode[&mode]);
        return 0xFF
    }
    pub fn get_addr_mode(&self, op: u8) -> &str {
        self.addr_mode[&Mos6502Debug::<'_>::_get_addr_mode(op)]
    }
    fn _get_addr_mode(opcode: u8) ->  AddrMode {
        let ho = opcode >> 4;
        let lo = opcode & 0x0F;
        if lo == 8 || (lo == 0xA && ho >= 0x8 && ho != 0xD && ho != 0xF) ||
            opcode == 0x00 || opcode == 0x40 || opcode == 0x60 {
            return AddrMode::IMPLIED
        }
        if odd(ho) {
            if lo == 0x0 {
                return AddrMode::RELATIVE
            }
            if lo == 0x1 {
                return AddrMode::INDIRECT_Y
            }
            if lo == 0x9 || opcode == 0xBE {
                return AddrMode::ABSOLUTE_Y
            }
            if (within(lo,0xD,0xE) && opcode != 0x9E) || opcode == 0xBC {
                return AddrMode::ABSOLUTE_X
            }
            if opcode == 0x96 || opcode == 0xB6 {
                return AddrMode::ZERO_PAGE_Y
            }
            if within(lo,0x5,0x6) || opcode == 0x94 || opcode == 0xB4 {
                return AddrMode::ZERO_PAGE_X
            }
            return AddrMode::ERROR
        } else {
            if lo == 0x1 {
                return AddrMode::INDIRECT_X
            }
            if (lo == 0x9 && ho != 0x8) || opcode == 0xA0 || opcode == 0xC0 || opcode == 0xE0 || opcode == 0xA2 {
                return AddrMode::IMMEDIATE
            }
            if within(lo,0x4,0x6) && opcode != 0x04 && opcode != 0x44 && opcode != 0x64 {
                return AddrMode::ZERO_PAGE
            }
            if lo == 0xA && ho & 0x9 == 0 {
                return AddrMode::ACCUMULATOR
            }
            if opcode == 0x6C {
                return AddrMode::INDIRECT
            }
            if (within(lo,0xC,0xE) && opcode != 0x0C) || opcode == 0x20 {
                return AddrMode::ABSOLUTE
            }
            return AddrMode::ERROR
        }
    }
}
