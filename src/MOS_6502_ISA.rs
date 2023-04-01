// Three parts of each instruction function:
// - cycle should be set to the amount of cycles the function takes
// - calculation should be done
// - flags should be set

#[repr(u8)]
enum MOS_6502_flag {
    C = 0x01,
    N = 0x02,
    I = 0x04,
    D = 0x08,
    B = 0x10,
    S = 0x20,
    V = 0x40,
    N = 0x80,
}
struct MOS_6502_inst {
    calc: &dyn Fn(&mut MOS_6502),
    cycles: u8,
    flags: u8,
}
impl MOS_6502_ISA for MOS_6502 {
    pub fn load_isa(&mut self) {
        self.isa[0x21] = MOS_6502_ISA::and_ind_x;
        self.isa[0x25] = MOS_6502_ISA::and_zp;
        self.isa[0x29] = MOS_6502_ISA::and_imm; 
        self.isa[0x2D] = MOS_6502_ISA::and_abs;
        self.isa[0x31] = MOS_6502_ISA::and_ind_y;
        self.isa[0x35] = MOS_6502_ISA::and_zp_x;
        self.isa[0x39] = MOS_6502_ISA::and_abs_y;
        self.isa[0x3D] = MOS_6502_ISA::and_abs_x;
        self.isa[0x06] = MOS_6502_ISA::asl_zp;
        self.isa[0x0A] = MOS_6502_ISA::asl_acc;
        self.isa[0x0E] = MOS_6502_ISA::asl_abs;
        self.isa[0x16] = MOS_6502_ISA::asl_zp_x;
        self.isa[0x1E] = MOS_6502_ISA::asl_abs_x;
        self.isa[0x49] = MOS_6502_ISA::eor_imm;
        self.isa[0x45] = MOS_6502_ISA::eor_zp;
        self.isa[0x55] = MOS_6502_ISA::eor_zp_x;
        self.isa[0x4D] = MOS_6502_ISA::eor_abs;
        self.isa[0x5D] = MOS_6502_ISA::eor_abs_x;
        self.isa[0x59] = MOS_6502_ISA::eor_abs_y;
        self.isa[0x41] = MOS_6502_ISA::eor_ind_x;
        self.isa[0x51] = MOS_6502_ISA::eor_ind_y;
        self.isa[0x4A] = MOS_6502_ISA::lsr_acc;
        self.isa[0x46] = MOS_6502_ISA::lsr_zp;
        self.isa[0x56] = MOS_6502_ISA::lsr_zp_x;
        self.isa[0x4E] = MOS_6502_ISA::lsr_abs;
        self.isa[0x5E] = MOS_6502_ISA::lsr_abs_x;
        self.isa[0x09] = MOS_6502_ISA::ora_imm;
        self.isa[0x05] = MOS_6502_ISA::ora_zp;
        self.isa[0x15] = MOS_6502_ISA::ora_zp_x;
        self.isa[0x0D] = MOS_6502_ISA::ora_abs;
        self.isa[0x1D] = MOS_6502_ISA::ora_abs_x;
        self.isa[0x19] = MOS_6502_ISA::ora_abs_y;
        self.isa[0x01] = MOS_6502_ISA::ora_ind_x;
        self.isa[0x11] = MOS_6502_ISA::ora_ind_y;
        self.isa[0x2A] = MOS_6502_ISA::rol_acc;
        self.isa[0x26] = MOS_6502_ISA::rol_zp;
        self.isa[0x36] = MOS_6502_ISA::rol_zp_x;
        self.isa[0x2E] = MOS_6502_ISA::rol_abs;
        self.isa[0x3E] = MOS_6502_ISA::rol_abs_x;
        self.isa[0x6A] = MOS_6502_ISA::ror_acc;
        self.isa[0x66] = MOS_6502_ISA::ror_zp;
        self.isa[0x76] = MOS_6502_ISA::ror_zp_x;
        self.isa[0x6E] = MOS_6502_ISA::ror_abs;
        self.isa[0x7E] = MOS_6502_ISA::ror_abs_x;
        self.isa[0x10] = MOS_6502_ISA::bpl;
        self.isa[0x30] = MOS_6502_ISA::bmi;
        self.isa[0x50] = MOS_6502_ISA::bvc;
        self.isa[0x70] = MOS_6502_ISA::bvs;
        self.isa[0x90] = MOS_6502_ISA::bcc;
        self.isa[0xB0] = MOS_6502_ISA::bcs;
        self.isa[0xD0] = MOS_6502_ISA::bne;
        self.isa[0xF0] = MOS_6502_ISA::beq;
        self.isa[0xC9] = MOS_6502_ISA::cmp_imm;
        self.isa[0xC5] = MOS_6502_ISA::cmp_zp;
        self.isa[0xD5] = MOS_6502_ISA::cmp_zp_x;
        self.isa[0xCD] = MOS_6502_ISA::cmp_abs;
        self.isa[0xDD] = MOS_6502_ISA::cmp_abs_x;
        self.isa[0xD9] = MOS_6502_ISA::cmp_abs_y;
        self.isa[0xC1] = MOS_6502_ISA::cmp_ind_x;
        self.isa[0xD1] = MOS_6502_ISA::cmp_ind_y;
        self.isa[0x24] = MOS_6502_ISA::bit_zp;
        self.isa[0x2C] = MOS_6502_ISA::bit_abs;
        self.isa[0xE0] = MOS_6502_ISA::cpx_imm;
        self.isa[0xE4] = MOS_6502_ISA::cpx_zp;
        self.isa[0xEC] = MOS_6502_ISA::cpx_abs;
        self.isa[0xC0] = MOS_6502_ISA::cpy_imm;
        self.isa[0xC4] = MOS_6502_ISA::cpy_zp;
        self.isa[0xCC] = MOS_6502_ISA::cpy_abs;
        self.isa[0x18] = MOS_6502_ISA::clc;
        self.isa[0x38] = MOS_6502_ISA::sec;
        self.isa[0xD8] = MOS_6502_ISA::cld;
        self.isa[0xF8] = MOS_6502_ISA::sed;
        self.isa[0x58] = MOS_6502_ISA::cli;
        self.isa[0x78] = MOS_6502_ISA::sei;
        self.isa[0xB8] = MOS_6502_ISA::clv;
        self.isa[0x4C] = MOS_6502_ISA::jmp_abs;
        self.isa[0x6C] = MOS_6502_ISA::jmp_ind;
        self.isa[0x60] = MOS_6502_ISA::rts;
        self.isa[0x20] = MOS_6502_ISA::jsr;
        self.isa[0x40] = MOS_6502_ISA::rti;
        self.isa[0x69] = MOS_6502_ISA::adc_imm;
        self.isa[0x65] = MOS_6502_ISA::adc_zp;
        self.isa[0x75] = MOS_6502_ISA::adc_zp_x;
        self.isa[0x6D] = MOS_6502_ISA::adc_abs;
        self.isa[0x7D] = MOS_6502_ISA::adc_abs_x;
        self.isa[0x79] = MOS_6502_ISA::adc_abs_y;
        self.isa[0x61] = MOS_6502_ISA::adc_ind_x;
        self.isa[0x71] = MOS_6502_ISA::adc_ind_y;
        self.isa[0xE9] = MOS_6502_ISA::sbc_imm;
        self.isa[0xE5] = MOS_6502_ISA::sbc_zp;
        self.isa[0xF5] = MOS_6502_ISA::sbc_zp_x;
        self.isa[0xED] = MOS_6502_ISA::sbc_abs;
        self.isa[0xFD] = MOS_6502_ISA::sbc_abs_x;
        self.isa[0xF9] = MOS_6502_ISA::sbc_abs_y;
        self.isa[0xE1] = MOS_6502_ISA::sbc_ind_x;
        self.isa[0xF1] = MOS_6502_ISA::sbc_ind_y;
        self.isa[0xA9] = MOS_6502_ISA::lda_imm;
        self.isa[0xA5] = MOS_6502_ISA::lda_zp;
        self.isa[0xB5] = MOS_6502_ISA::lda_zp_x;
        self.isa[0xAD] = MOS_6502_ISA::lda_abs;
        self.isa[0xBD] = MOS_6502_ISA::lda_abs_x;
        self.isa[0xB9] = MOS_6502_ISA::lda_abs_y;
        self.isa[0xA1] = MOS_6502_ISA::lda_ind_x;
        self.isa[0xB1] = MOS_6502_ISA::lda_ind_y;
        self.isa[0x85] = MOS_6502_ISA::sta_zp;
        self.isa[0x95] = MOS_6502_ISA::sta_zp_x;
        self.isa[0x8D] = MOS_6502_ISA::sta_abs;
        self.isa[0x9D] = MOS_6502_ISA::sta_abs_x;
        self.isa[0x99] = MOS_6502_ISA::sta_abs_y;
        self.isa[0x81] = MOS_6502_ISA::sta_ind_x;
        self.isa[0x91] = MOS_6502_ISA::sta_ind_y;
        self.isa[0xA2] = MOS_6502_ISA::ldx_imm;
        self.isa[0xA6] = MOS_6502_ISA::ldx_zp;
        self.isa[0xB6] = MOS_6502_ISA::ldx_zp_y;
        self.isa[0xAE] = MOS_6502_ISA::ldx_abs;
        self.isa[0xBE] = MOS_6502_ISA::ldx_abs_y;
        self.isa[0x86] = MOS_6502_ISA::stx_zp;
        self.isa[0x96] = MOS_6502_ISA::stx_zp_y;
        self.isa[0x8E] = MOS_6502_ISA::stx_abs;
        self.isa[0xA0] = MOS_6502_ISA::ldy_imm;
        self.isa[0xA4] = MOS_6502_ISA::ldy_zp;
        self.isa[0xB4] = MOS_6502_ISA::ldy_zp_x;
        self.isa[0xAC] = MOS_6502_ISA::ldy_abs;
        self.isa[0xBC] = MOS_6502_ISA::ldy_abs_x;
        self.isa[0x84] = MOS_6502_ISA::sty_zp;
        self.isa[0x94] = MOS_6502_ISA::sty_zp_x;
        self.isa[0x8C] = MOS_6502_ISA::sty_abs;
        self.isa[0xC6] = MOS_6502_ISA::dec_zp;
        self.isa[0xD6] = MOS_6502_ISA::dec_zp_x;
        self.isa[0xCE] = MOS_6502_ISA::dec_abs;
        self.isa[0xDE] = MOS_6502_ISA::dec_abs_x;
        self.isa[0xE6] = MOS_6502_ISA::inc_zp;
        self.isa[0xF6] = MOS_6502_ISA::inc_zp_x;
        self.isa[0xEE] = MOS_6502_ISA::inc_abs;
        self.isa[0xFE] = MOS_6502_ISA::inc_abs_x;
        self.isa[0xAA] = MOS_6502_ISA::tax;
        self.isa[0xA8] = MOS_6502_ISA::tay;
        self.isa[0x8A] = MOS_6502_ISA::txa;
        self.isa[0x98] = MOS_6502_ISA::tya;
        self.isa[0xCA] = MOS_6502_ISA::dex;
        self.isa[0x88] = MOS_6502_ISA::dey;
        self.isa[0xE8] = MOS_6502_ISA::inx;
        self.isa[0xC8] = MOS_6502_ISA::iny;
        self.isa[0x48] = MOS_6502_ISA::pha;
        self.isa[0x08] = MOS_6502_ISA::php;
        self.isa[0x9A] = MOS_6502_ISA::txs;
        self.isa[0x68] = MOS_6502_ISA::pla;
        self.isa[0xBA] = MOS_6502_ISA::tsx;
        self.isa[0x28] = MOS_6502_ISA::plp;
        self.isa[0x00] = MOS_6502_ISA::brk;
        self.isa[0xEA] = MOS_6502_ISA::nop;
    }
    // Utility functions
    fn set_z_n(&mut self) {
        self.set_z_n(self.a);
    }
    fn set_z_n(&mut self,val: u8) {
        self.ps = (self.ps & 0x7D) | (val != 0 ? val & MOS_6502_flag::N : MOS_6502_flag::Z);
    }

    // Returns a u16 from memory location
    fn fetch_u16(&mut self, addr: u16) -> u16 {
        //TODO: check endianness here
        (self.mmu.get(addr) as u16)<<8 + (self.mmu.get(addr+1) as u16)
    }

    // Decode functions return the value from the opcode param, based on addressing mode.
    // This can be either the opcode value itself, or the value pointed to by the opcode.

    // Addr functions return the address that would need to be accessed.
    // ONLY ONE of decode or addr should be called, and only once per instruction.
    fn decode_imm(&mut self) -> u8 {
        self.mmu.get(self.pc++)
    }

    fn decode_zp(&mut self) -> u8 {
        self.mmu.get(self.addr_zp())
    }
    fn addr_zp(&mut self) -> u16 {
        self.mmu.get(self.pc++) as u16
    }

    fn decode_zp_x(&mut self) -> u8 {
        self.mmu.get(self.addr_zp_x())
    }
    #![allow(arithmetic_overflow)]
    fn addr_zp_x(&mut self) -> u16 {
        (self.mmu.get(self.pc++) + self.x) as u16
    }

    fn decode_zp_y(&mut self) -> u8 {
        self.mmu.get(self.addr_zp_y())
    }
    #![allow(arithmetic_overflow)]
    fn addr_zp_y(&mut self) -> u16 {
        (self.mmu.get(self.pc++) + self.y) as u16
    }

    fn decode_abs(&mut self) -> u8 {
        self.mmu.get(self.addr_abs())
    }
    fn addr_abs(&mut self) -> u16 {
        let a = self.pc;
        self.pc += 2;
        self.fetch_u16(a)
    }

    fn decode_abs_x(&mut self) -> u8 {
        self.mmu.get(self.addr_abs_x())
    }
    fn addr_abs_x(&mut self) -> u16 {
        let a = self.pc;
        self.pc += 2;
        self.fetch_u16(a + (self.x as u16))
    }

    fn decode_abs_y(&mut self, add_cycle_on_page_boundary: bool) -> u8 {
        self.mmu.get(self.addr_abs_y(add_cycle_on_page_boundary));
    }
    fn decode_abs_y(&mut self) -> u8 {
        self.decode_abs_y(false)
    }
    
    fn addr_abs_y(&mut self) -> u16 {
        self.addr_abs_y(false)
    }
    fn addr_abs_y(&mut self, add_cycle_on_page_boundary: bool) -> u16 {
        let addr = self.fetch_u16(self.pc);
        self.pc += 2;
        if add_cycle_on_page_boundary {
            self.check_page_boundary(addr,self.y);
        }
        addr + (self.y as u16) 
    }

    fn decode_ind_x(&mut self) -> u8 {
        self.mmu.get(self.addr_ind_x())
    }
    #![allow(arithmetic_overflow)]
    fn addr_ind_x(&mut self) -> u16 {
        self.decode_u16((self.mmu.get(self.pc++) + self.x) as u16)
    }

    fn decode_ind_y(&mut self) -> u8 {
        self.mmu.get(self.addr_ind_y())
    }
    fn addr_ind_y(&mut self) -> u16 {
        let addr: u16 = self.decode_u16(self.mmu.get(self.pc++) as u16);
        if add_cycle_on_page_boundary {
            self.check_page_boundary(addr,self.y);
        }
        addr + (self.y as u16)
    }
    fn check_page_boundary(&mut self,addr: u16, off: u8) {
        // If we cross a page boundary, add 1 cycle to latency
        if addr%256 + off > 256 {
            self.cycles ++;
        }
    }
    fn check_page_boundary(&mut self,addr: u16, rel: i8) {
        // If we cross a page boundary, add 1 cycle to latency
        if (addr + rel)>>8 != addr>>8 {
            self.cycles ++;
        }
    }
    // Instructions
    fn and_imm(&mut self) {
        self.cycles = 2;
        self.a &= self.decode_imm();
        self.set_z_n();
    }
    fn and_zp(&mut self) {
        self.cycles = 3;
        self.a &= self.decode_zp();
        self.set_z_n();
    }
    fn and_zp_x(&mut self) {
        self.cycles = 4;
        self.a &= self.decode_zp_x();
        self.set_z_n();
    }
    fn and_abs(&mut self) {
        self.cycles = 4; 
        self.a &= self.decode_abs();
        self.set_z_n();
    }
    fn and_abs_x(&mut self) {
        self.cycles = 4;
        self.a &= self.decode_abs_x(true);
        self.set_z_n();
    }
    fn and_abs_y(&mut self) {
        self.cycles = 4;
        self.a &= self.decode_abs_y(true);
        self.set_z_n();
    }
    fn and_ind_x(&mut self) {
        self.cycles = 6;
        self.a &= self.decode_ind_x();
        self.set_z_n();
    }
    fn and_ind_y(&mut self) {
        self.cycles = 5;
        self.a &= self.decode_ind_y(true);
        self.set_z_n();
    }

    fn _asl(&mut self,&mut i: u8) -> u8{
        self.ps |= (i & MOS_6502_flag::C);
        i <<= 1;
        self.set_z_n(i);
        i
    }
    fn asl_acc(&mut self) {
        self.cycles = 2;
        self._asl(&mut self.a);
    }
    fn asl_zp(&mut self) {
        self.cycles = 5;
        let addr: u16 = self.decode_zp() as u16;
        let i = self.mmu.get(addr);
        self.mmu.set(addr, self._asl(i));
    }
    fn asl_zp_x(&mut self) {
        self.cycles = 6;
        let addr: u16 = self.decode_zp_x() as u16;
        let i = self.mmu.get(addr);
        self.mmu.set(addr, self._asl(i));
    }
    fn asl_abs(&mut self) {
        self.cycles = 6;
        let addr: u16 = self.addr_abs();
        let i: u8 = self.mmu.get(addr);
        self.mmu.set(addr, self._asl(i));
    }
    fn asl_abs_x(&mut self) {
        self.cycles = 7;
        let addr: u16 = self.addr_abs_x();
        let i: u8 = self.mmu.get(addr);
        self.mmu.set(addr, self._asl(i));
    }
    fn _adc(&mut self,val: u8) {
        //TODO: ensure edge cases work appropriately here; probably needs optimising too
        if(self.ps & MOS_6502_flag::D) {
            // The absolute maximum value for lo and ho (low digit / high digit) is:
            //      15 + 15 + 1 = 31 or 0x1F
            // The logical maximum value should be 9 + 9 + 1 = 19 or 0x13
            // Not sure how the 6502 handles invalid BCD values, but we'll
            // only carry 1 when lo/ho is > 9
            let lo = (self.a & 0x0F) + (self.ps & MOS_6502_flag::C) + (val & 0x0F);
            let ho = (self.a & 0xF0)>>4 + (val & 0xF0)>>4;
            if(lo > 9) {
                ho ++; 
                lo %= 10;
            }
            if(ho > 9) {
                self.ps |= MOS_6502_flag::C; 
                ho %= 10;
            }
            self.a = ho<<4 + lo;
        } else {
            let res: u16 = self.a + val;
            self.ps |= (res >= 256 ? MOS_6502_flag::C : 0);
            self.a += val;
        }
        self.set_z_n();
    }
    fn adc_imm(&mut self) {
        self.cycles = 2;
        self._adc(self.decode_imm());
    }
    fn adc_zp(&mut self) {
        self.cycles = 3;
        self._adc(self.decode_zp());
    }
    fn adc_zp_x(&mut self) {
        self.cycles = 4;
        self._adc(self.decode_zp_x());
    }
    fn adc_abs(&mut self) {
        self.cycles = 4;
        self._adc(self.decode_abs());
    }
    fn adc_abs_x(&mut self) {
        self.cycles = 4;
        self._adc(self.decode_abs_x(true));
    }
    fn adc_abs_y(&mut self) {
        self.cycles = 4;
        self._adc(self.decode_abs_y(true));
    }
    fn adc_ind_x(&mut self) {
        self.cycles = 6;
        self._adc(self.decode_ind_x());
    }
    fn adc_ind_y(&mut self) {
        self.cycles = 5;
        self._adc(self.decode_ind_y(true));
    }

    fn _br(&mut self,flag: MOS_6502_flag, set: bool) {
        if ((self.ps & flag) == set) {
            self.cycles = 3;
            let rel: i8 = self.decode_imm() as i8;
            self.check_page_boundary(self.pc, rel);
            self.pc += rel;
        } else {
            self.cycles = 2;
        }
    }
    fn bcs(&mut self) {
        self._br(MOS_6502_flag::C, true)
    }
    fn bcc(&mut self) {
        self._br(MOS_6502_flag::C, false)
    }
    fn beq(&mut self) {
        self._br(MOS_6502_flag::Z, true)
    }
    fn bne(&mut self) {
        self._br(MOS_6502_flag::Z, false)
    }
    fn bmi(&mut self) {
        self._br(MOS_6502_flag::N, true)
    }
    fn bpl(&mut self) {
        self._br(MOS_6502_flag::N, false)
    }
    fn bvs(&mut self) {
        self._br(MOS_6502_flag::V, true)
    }
    fn bvc(&mut self) {
        self._br(MOS_6502_flag::V, false)
    }

    fn bit_zp(&mut self) {
        self.cycles = 3;
        let t = self.a & self.decode_zp();
        self.ps = (self.ps & 0x3F) | (t & 0xC0);
    }
    fn bit_abs(&mut self) {
        self.cycles = 4;
        let t = self.a & self.decode_abs();
        self.ps = (self.ps & 0x3F) | (t & 0xC0);
    }
    fn _psh(&mut self, val: u8) {
        self.mmu.set(0x100 + (self.sp as u16),val);
        self.sp--;
    }
    fn brk(&mut self) {
        self.cycles = 7;
        self._psh(self.pc>>8 as u8);
        self._psh(self.pc%256 as u8);
        self._psh(self.ps);
        self.pc = 0xFFFE;
        self._sf(MOS_6502_flag::B, true);
    }
    fn _sf(&mut self, flag: MOS_6502_flag, set: bool) {
        let f: u8 = flag as u8;
        self.ps &= !f | (set ? flag : 0);
    }
    fn clc(&mut self) {
        self.cycles = 2;
        self._sf(MOS_6502_flag::C,false);
    }
    fn cld(&mut self) {
        self.cycles = 2;
        self._sf(MOS_6502_flag::D,false);
    }
    fn cli(&mut self) {
        self.cycles = 2;
        self._sf(MOS_6502_flag::I,false);
    }
    fn clv(&mut self) {
        self.cycles = 2;
        self._sf(MOS_6502_flag::V,false);
    }

    fn _cmp(&mut self, val: u8) {
        self._sf(MOS_6502_flag::C, self.a > val);
        self.set_z_n(self.a - val);
    }
    fn cmp_imm(&mut self) {
        self.cycles = 2;
        self._cmp(self.decode_imm());
    }
    fn cmp_zp(&mut self) {
        self.cycles = 3;
        self._cmp(self.decode_zp());
    }
    fn cmp_zp_x(&mut self) {
        self.cycles = 4;
        self._cmp(self.decode_zp_x());
    }
    fn cmp_abs(&mut self) {
        self.cycles = 4;
        self._cmp(self.decode_abs());
    }
    fn cmp_abs_x(&mut self) {
        self.cycles = 4;
        self._cmp(self.decode_abs_x(true));
    }
    fn cmp_abs_y(&mut self) {
        self.cycles = 4;
        self._cmp(self.decode_abs_y(true));
    }
    fn cmp_ind_x(&mut self) {
        self.cycles = 6;
        self._cmp(self.decode_ind_x());
    }
    fn cmp_ind_y(&mut self) {
        self.cycles = 5;
        self._cmp(self.decode_ind_y(true));
    }

    fn _cpx(&mut self, val: u8) {
        self._sf(MOS_6502_flag::C, self.x > val);
        self.set_z_n(self.x - val);
    }
    fn cpx_imm(&mut self) {
        self.cycles = 2;
        self._cpx(self.decode_imm);
    }
    fn cpx_abs(&mut self) {
        self.cycles = 4;
        self._cpx(self.decode_abs);
    }
    fn cpx_zp(&mut self) {
        self.cycles = 3;
        self._cpx(self.decode_zp);
    }

    fn _cpy(&mut self) {
        self._sf(MOS_6502_flag::C, self.y > val);
        self.set_z_n(self.y - val);
    }
    fn cpy_imm(&mut self) {
        self.cycles = 2;
        self._cpy(self.decode_imm);
    }
    fn cpy_abs(&mut self) {
        self.cycles = 4;
        self._cpy(self.decode_abs);
    }
    fn cpy_zp(&mut self) {
        self.cycles = 3;
        self._cpy(self.decode_zp);
    }

    fn _dec(&mut self, addr: u16) {
        let val = self.mmu.get(addr) - 1;
        self.set_z_n(val);
        self.mmu.set(addr, val);
    }
    fn dec_zp(&mut self) {
        self.cycles = 5;
        self._dec(self.addr_zp());
    }
    fn dec_zp_x(&mut self) {
        self.cycles = 6;
        self._dec(self.addr_zp_x());
    }
    fn dec_abs(&mut self) {
        self.cycles = 6;
        self._dec(self.addr_abs());
    }
    fn dec_abs_x(&mut self) {
        self.cycles = 7;
        self._dec(self.addr_abs_x());
    }

    fn dex(&mut self) {
        self.cycles = 2;
        self.x--;
        self.set_z_n(self.x);
    }
    fn dey(&mut self) {
        self.cycles = 2;
        self.y--;
        self.set_z_n(self.y);
    }

    fn _eor(&mut self,val: u8) {
        self.a ^= val;
        self.set_z_n();
    }
    fn eor_imm(&mut self) {
        self.cycles = 2;
        self._eor(self.decode_imm());
    }
    fn eor_zp(&mut self) {
        self.cycles = 3;
        self._eor(self.decode_zp());
    }
    fn eor_zp_x(&mut self) {
        self.cycles = 4;
        self._eor(self.decode_zp_x());
    }
    fn eor_abs(&mut self) {
        self.cycles = 4;
        self._eor(self.decode_abs());
    }
    fn eor_abs_x(&mut self) {
        self.cycles = 4;
        self._eor(self.decode_abs_x(true));
    }
    fn eor_abs_y(&mut self) {
        self.cycles = 4;
        self._eor(self.decode_abs_y(true));
    }
    fn eor_ind_x(&mut self) {
        self.cycles = 6;
        self._eor(self.decode_ind_x());
    }
    fn eor_ind_y(&mut self) {
        self.cycles = 5;
        self._eor(self.decode_ind_y(true));
    }

    fn _inc(&mut self, addr: u16) {
        let val = self.mmu.get(addr) + 1;
        self.set_z_n(val);
        self.mmu.set(addr, val);
    }
    fn inc_zp(&mut self) {
        self.cycles = 5;
        self._inc(self.addr_zp());
    }
    fn inc_zp_x(&mut self) {
        self.cycles = 6;
        self._inc(self.addr_zp_x());
    }
    fn inc_abs(&mut self) {
        self.cycles = 6;
        self._inc(self.addr_abs());
    }
    fn inc_abs_x(&mut self) {
        self.cycles = 7;
        self._inc(self.addr_abs_x());
    }

    fn inx(&mut self) {
        self.cycles = 2;
        self.x++;
        self.set_z_n(self.x);
    }
    fn iny(&mut self) {
        self.cycles = 2;
        self.y++;
        self.set_z_n(self.y);
    }

    fn jmp_abs(&mut self) {
        self.cycles = 3;
        self.pc = self.decode_abs();
    }
/*
    NB:
An original 6502 has does not correctly fetch the target address if the indirect 
vector falls on a page boundary (e.g. $xxFF where xx is any value from $00 to $FF). 
In this case fetches the LSB from $xxFF as expected but takes the MSB from $xx00. 
This is fixed in some later chips like the 65SC02 so for compatibility always ensure 
the indirect vector is not at the end of the page.
*/
    fn jmp_ind(&mut self) {
        self.cycles = 5;
        self.pc = self.decode_ind();
        //TODO: add code to simulate a bug?
    }

    fn jsr(&mut self) {
        self.cycles = 6;
        // --WARNING: possible bug here--
        // JSR pushes the "return address - 1" onto the stack. Is
        // this to compensate for an increment somewhere?
        // Let's just push the exact address for now.
        let ret: u16 = self.pc + 2;
        self._psh(ret>>8 as u8);
        self._psh(ret%256 as u8);
        self.pc = self.decode_abs();
    }

    fn _lda(&mut self, val: u8) {
        self.a = val;
        self.set_z_n();
    }
    fn lda_imm(&mut self) {
        self.cycles = 2;
        self._lda(self.decode_imm());
    }
    fn lda_zp(&mut self) {
        self.cycles = 3;
        self._lda(self.decode_zp());
    }
    fn lda_zp_x(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_zp_x());
    }
    fn lda_abs(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_abs());
    }
    fn lda_abs_x(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_abs_x(true));
    }
    fn lda_abs_y(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_abs_y(true));
    }
    fn lda_ind_x(&mut self) {
        self.cycles = 6;
        self._lda(self.decode_ind_x());
    }
    fn lda_ind_y(&mut self) {
        self.cycles = 5;
        self._lda(self.decode_ind_y(true));
    }

    fn _ldx(&mut self, val: u8) {
        self.x = val;
        self.set_z_n();
    }
    fn ldx_imm(&mut self) {
        self.cycles = 2;
        self._lda(self.decode_imm());
    }
    fn ldx_zp(&mut self) {
        self.cycles = 3;
        self._lda(self.decode_zp());
    }
    fn ldx_zp_y(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_zp_y());
    }
    fn ldx_abs(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_abs());
    }
    fn ldx_abs_y(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_abs_y(true));
    }

    fn _ldy(&mut self, val: u8) {
        self.y = val;
        self.set_z_n();
    }
    fn ldy_imm(&mut self) {
        self.cycles = 2;
        self._lda(self.decode_imm());
    }
    fn ldy_zp(&mut self) {
        self.cycles = 3;
        self._lda(self.decode_zp());
    }
    fn ldy_zp_x(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_zp_x());
    }
    fn ldy_abs(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_abs());
    }
    fn ldy_abs_x(&mut self) {
        self.cycles = 4;
        self._lda(self.decode_abs_x(true));
    }

    fn lsr_acc(&mut self) {
        self.cycles = 2;
        self._sf(MOS_6502_flag::C, (self.a & 0x01));
        self.a >>= 1;
        self.set_z_n();
    }
    fn _lsr(&mut self, addr: u16) {
        let val = self.mmu.get(addr);
        self._sf(MOS_6502_flag::C, (val & 0x01));
        val >>= 1;
        self.set_z_n();
        self.mmu.set(addr,val);
    }
    fn lsr_zp(&mut self) {
        self.cycles = 5;
        self._lsr(self.addr_zp());
    }
    fn lsr_zp_x(&mut self) {
        self.cycles = 6;
        self._lsr(self.addr_zp_x());
    }
    fn lsr_abs(&mut self) {
        self.cycles = 6;
        self._lsr(self.addr_abs());
    }
    fn lsr_abs_x(&mut self) {
        self.cycles = 7;
        self._lsr(self.addr_abs_x());
    }

    fn nop(&mut self) {
        self.cycles = 2;
    }

    fn _ora(&mut self, val: u8) {
        self.a |= val;
        self.set_z_n();
    }
    fn ora_imm(&mut self) {
        self.cycles = 2;
        self._ora(self.decode_imm());
    }
    fn ora_zp(&mut self) {
        self.cycles = 3;
        self._ora(self.decode_zp());
    }
    fn ora_zp_x(&mut self) {
        self.cycles = 4;
        self._ora(self.decode_zp_x());
    }
    fn ora_abs(&mut self) {
        self.cycles = 4;
        self._ora(self.decode_abs());
    }
    fn ora_abs_x(&mut self) {
        self.cycles = 4;
        self._ora(self.decode_abs_x(true));
    }
    fn ora_abs_y(&mut self) {
        self.cycles = 4;
        self._ora(self.decode_abs_y(true));
    }
    fn ora_ind_x(&mut self) {
        self.cycles = 6;
        self._ora(self.decode_ind_x());
    }
    fn ora_ind_y(&mut self) {
        self.cycles = 5;
        self._ora(self.decode_ind_y(true));
    }

    fn pha(&mut self) {
        self.cycles = 3;
        self._psh(self.a);
    }

    fn php(&mut self) {
        self.cycles = 3;
        self._psh(self.ps);
    }
    fn _pop(&mut self) -> u8 {
        self.mmu.get(0x100 + (self.sp++))
    }
    fn pla(&mut self) {
        self.cycles = 4;
        self.a = self._pop();
    }
    fn plp(&mut self) {
        self.cycles = 4;
        self.ps = self._pop();
    }

    fn rol_acc(&mut self) {
        let carry: bool = self.a & 0x80 != 0;
        self.a <<= 1;
        self.a |= self.ps & MOS_6502_flag::C;
        self._sf(MOS_6502_flag::C, carry);
        self.set_z_n();
    }
    fn _rol(&mut self,addr: u16) {
        let val: u8 = self.mmu.get(addr);
        let carry: bool = val & 0x80 != 0;
        val <<= 1;
        val |= self.ps & MOS_6502_flag::C;
        self._sf(MOS_6502_flag::C, carry);
        self.set_z_n(val);       
        self.mmu.set(addr,val);
    }
    fn rol_abs(&mut self) {
        self.cycles = 6;
        self._rol(self.addr_abs());
    }
    fn rol_abs_x(&mut self) {
        self.cycles = 7;
        self._rol(self.addr_abs_x());
    }
    fn rol_zp(&mut self) {
        self.cycles = 5;
        self._rol(self.addr_zp());
    }
    fn rol_zp_x(&mut self) {
        self.cycles = 6;
        self._rol(self.addr_zp_x());
    }

    fn ror_acc(&mut self) {
        let carry: bool = self.a & 0x80 != 0;
        self.a <<= 1;
        self.a |= self.ps & MOS_6502_flag::C;
        self._sf(MOS_6502_flag::C, carry);
        self.set_z_n();
    }
    fn _ror(&mut self,addr: u16) {
        let val: u8 = self.mmu.get(addr);
        let carry: bool = val & 0x01 != 0;
        val >>= 1;
        val |= (self.ps & MOS_6502_flag::C)<<7;
        self._sf(MOS_6502_flag::C, carry);
        self.set_z_n(val);       
        self.mmu.set(addr,val);
    }
    fn ror_abs(&mut self) {
        self.cycles = 6;
        self._ror(self.addr_abs());
    }
    fn ror_abs_x(&mut self) {
        self.cycles = 7;
        self._ror(self.addr_abs_x());
    }
    fn ror_zp(&mut self) {
        self.cycles = 5;
        self._ror(self.addr_zp());
    }
    fn ror_zp_x(&mut self) {
        self.cycles = 6;
        self._ror(self.addr_zp_x());
    }


}