use std::collections::hash_map::HashMap;


fn even(v: u8) -> bool {
    v & 0x01 == 0
}
fn odd(v: u8) -> bool {
    !even(v)
}
fn within(v: u8,l: u8, h: u8) -> bool {
    v >= l && v <= h
}

/* How do you do this in rust??
pub enum AddrMode {
    ABSOLUTE = "ABS ",
    ABSOLUTE_X = "ABSX",
    ABSOLUTE_Y = "ABSY",
    ACCUMULATOR = "ACC ",
    ERROR = "ERR ",
    IMMEDIATE = "IMM ",
    IMPLIED = "IMPL",
    INDIRECT = "IND ",
    INDIRECT_X = "INDX",
    INDIRECT_Y = "INDY",
    RELATIVE = "REL ",
    ZERO_PAGE = "ZP  ",
    ZERO_PAGE_X = "ZP X",
    ZERO_PAGE_Y = "ZP Y"
}
 */
pub struct Mos6502Debug<'a> {
    mnemonic: HashMap<u8,&'a str>
}
impl<'a> Mos6502Debug<'a> {
    pub fn new() -> Mos6502Debug<'a> {
        let mut ret = Mos6502Debug {
            mnemonic: HashMap::new()
        };
        ret._load_mnemonics();
        ret
    }
    pub fn getMnemonic(&self, op: u8) -> &'a str {
        self.mnemonic[&op]
    }
    /*
    pub fn decodeInstruction(&self, bus: &MMU, addr: u16) -> str {
        //TODO: Implement detailed display here
        "NYI"
    }
     */
    fn _load_mnemonics(&mut self) {
        self.mnemonic.insert(0x10, "BPL ");
        self.mnemonic.insert(0x30, "BMI ");
        self.mnemonic.insert(0x50, "BVC ");
        self.mnemonic.insert(0x70, "BVS ");
        self.mnemonic.insert(0x90, "BCC ");
        self.mnemonic.insert(0xB0, "BCS ");
        self.mnemonic.insert(0xD0, "BNE ");
        self.mnemonic.insert(0xF0, "BEQ ");
        self.mnemonic.insert(0x18, "CLC ");
        self.mnemonic.insert(0x38, "SEC ");
        self.mnemonic.insert(0x58, "CLI ");
        self.mnemonic.insert(0x78, "SEI ");
        self.mnemonic.insert(0xB8, "CLV ");
        self.mnemonic.insert(0xD8, "CLD ");
        self.mnemonic.insert(0xF8, "SED ");
        self.mnemonic.insert(0xAA, "TAX ");
        self.mnemonic.insert(0x8A, "TXA ");
        self.mnemonic.insert(0xCA, "DEX ");
        self.mnemonic.insert(0xE8, "INX ");
        self.mnemonic.insert(0xA8, "TAY ");
        self.mnemonic.insert(0x98, "TYA ");
        self.mnemonic.insert(0x88, "DEY ");
        self.mnemonic.insert(0xC8, "INY ");
        self.mnemonic.insert(0x9A, "TXS ");
        self.mnemonic.insert(0xBA, "TSX ");
        self.mnemonic.insert(0x48, "PHA ");
        self.mnemonic.insert(0x68, "PLA ");
        self.mnemonic.insert(0x08, "PHP ");
        self.mnemonic.insert(0x28, "PLP ");
        self.mnemonic.insert(0x00, "BRK ");
        self.mnemonic.insert(0x20, "JSR ");
        self.mnemonic.insert(0x40, "RTI ");
        self.mnemonic.insert(0x60, "RTS ");
        self.mnemonic.insert(0xEA, "NOP ");

        for op in [0x69,0x65,0x75,0x6D,0x7D,0x79,0x61,0x71] {
            self.mnemonic.insert(op, "ADC ");
        }
        for op in [0x29,0x25,0x35,0x2D,0x3D,0x39,0x21,0x31] {
            self.mnemonic.insert(op, "AND ");
        }
        for op in [0x0A,0x06,0x16,0x0E,0x1E] {
            self.mnemonic.insert(op, "ASL ");
        }
        for op in [0x24,0x2C] {
            self.mnemonic.insert(op, "BIT ");
        }
        for op in [0xC9,0xC5,0xD5,0xCD,0xDD,0xD9,0xC1,0xD1] {
            self.mnemonic.insert(op, "CMP ");
        }
        for op in [0xE0,0xE4,0xEC] {
            self.mnemonic.insert(op, "CPX ");
        }
        for op in [0xC0,0xC4,0xCC] {
            self.mnemonic.insert(op, "CPY ");
        }
        for op in [0xC6,0xD6,0xCE,0xDE] {
            self.mnemonic.insert(op, "DEC ");
        }
        for op in [0x49,0x45,0x55,0x4D,0x5D,0x59,0x41,0x51] {
            self.mnemonic.insert(op, "EOR ");
        }
        for op in [0xE6,0xF6,0xEE,0xFE] {
            self.mnemonic.insert(op, "INC ");
        }
        for op in [0x4C,0x6C] {
            self.mnemonic.insert(op, "JMP ");
        }
        for op in [0xA9,0xA5,0xB5,0xAD,0xBD,0xB9,0xA1,0xB1] {
            self.mnemonic.insert(op, "LDA ");
        }
        for op in [0xA2,0xA6,0xB6,0xAE,0xBE] {
            self.mnemonic.insert(op, "LDX ");
        }
        for op in [0xA0,0xA4,0xB4,0xAC,0xBC] {
            self.mnemonic.insert(op, "LDY ");
        }
        for op in [0x4A,0x46,0x56,0x4E,0x5E] {
            self.mnemonic.insert(op, "LSR ");
        }
        for op in [0x09,0x05,0x15,0x0D,0x1D,0x19,0x01,0x11] {
            self.mnemonic.insert(op, "ORA ");
        }
        for op in [0x2A,0x26,0x36,0x2E,0x3E] {
            self.mnemonic.insert(op, "ROL ");
        }
        for op in [0x6A,0x66,0x76,0x6E,0x7E] {
            self.mnemonic.insert(op, "ROR ");
        }
        for op in [0xE9,0xE5,0xF5,0xED,0xFD,0xF9,0xE1,0xF1] {
            self.mnemonic.insert(op, "SBC ");
        }
        for op in [0x85,0x95,0x8D,0x9D,0x99,0x81,0x91] {
            self.mnemonic.insert(op, "STA ");
        }
        for op in [0x86,0x96,0x8E] {
            self.mnemonic.insert(op, "STX ");
        }
        for op in [0x84,0x94,0x8C] {
            self.mnemonic.insert(op, "STY ");
        }
    }
    pub fn getAddrMode(&self, op: u8) -> &'a str {
        Mos6502Debug::<'_>::_getAddrMode(op)
    }
    fn _getAddrMode(opcode: u8) -> &'a str {
        let ho = opcode >> 4;
        let lo = opcode & 0x0F;
        if lo == 8 || (lo == 0xA && ho >= 0x8 && ho != 0xD && ho != 0xF) ||
            opcode == 0x00 || opcode == 0x40 || opcode == 0x60 {
            return "IMPL"
        }
        if odd(ho) {
            if lo == 0x0 {
                return "REL "
            }
            if lo == 0x1 {
                return "INDX"
            }
            if lo == 0x9 || opcode == 0xBE {
                return "ABSY"
            }
            if opcode == 0xBE {
                return "ABSY"
            }
            if within(lo,0xD,0xE) && opcode != 0x9E {
                return "ABSX"
            }
            if within(lo,0x5,0x6) || opcode == 0x94 || opcode == 0xB4 {
                return "ZP X"
            }
            if opcode == 0x96 || opcode == 0xB6 {
                return "ZP Y"
            }
            return "ERR "
        } else {
            if lo == 0x1 {
                return "INDY"
            }
            if lo == 0x9 && ho != 0x8{
                return "IMM "
            }
            if lo == 0xA && ho & 0x9 == 0 {
                return "ACC "
            }
            if opcode == 0x6C {
                return "IND "
            }
            if (within(lo,0xC,0xE) && opcode != 0x0C) || opcode == 0x20 {
                return "ABS "
            }
            if within(lo,0x5,0x6) && opcode != 0x04 && opcode != 0x44 && opcode != 0x64 {
                return "ZP  "
            }
            return "ERR "
        }
    }
}