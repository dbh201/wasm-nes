// Three parts of each instruction function:
// - cycle should be set to the amount of cycles the function takes
// - calculation should be done
// - flags should be set

use crate::Mos6502::{Mos6502,Mos6502Flag};


pub trait Mos6502Isa {
    fn load_isa(&mut self);

    // A function for every opcode...
    // Was there an easier way?
    fn and_ind_x(&mut self);
    fn and_zp(&mut self);
    fn and_imm(&mut self);
    fn and_abs(&mut self);
    fn and_ind_y(&mut self);
    fn and_zp_x(&mut self);
    fn and_abs_y(&mut self);
    fn and_abs_x(&mut self);
    fn asl_zp(&mut self);
    fn asl_acc(&mut self);
    fn asl_abs(&mut self);
    fn asl_zp_x(&mut self);
    fn asl_abs_x(&mut self);
    fn eor_imm(&mut self);
    fn eor_zp(&mut self);
    fn eor_zp_x(&mut self);
    fn eor_abs(&mut self);
    fn eor_abs_x(&mut self);
    fn eor_abs_y(&mut self);
    fn eor_ind_x(&mut self);
    fn eor_ind_y(&mut self);
    fn lsr_acc(&mut self);
    fn lsr_zp(&mut self);
    fn lsr_zp_x(&mut self);
    fn lsr_abs(&mut self);
    fn lsr_abs_x(&mut self);
    fn ora_imm(&mut self);
    fn ora_zp(&mut self);
    fn ora_zp_x(&mut self);
    fn ora_abs(&mut self);
    fn ora_abs_x(&mut self);
    fn ora_abs_y(&mut self);
    fn ora_ind_x(&mut self);
    fn ora_ind_y(&mut self);
    fn rol_acc(&mut self);
    fn rol_zp(&mut self);
    fn rol_zp_x(&mut self);
    fn rol_abs(&mut self);
    fn rol_abs_x(&mut self);
    fn ror_acc(&mut self);
    fn ror_zp(&mut self);
    fn ror_zp_x(&mut self);
    fn ror_abs(&mut self);
    fn ror_abs_x(&mut self);
    fn bpl(&mut self);
    fn bmi(&mut self);
    fn bvc(&mut self);
    fn bvs(&mut self);
    fn bcc(&mut self);
    fn bcs(&mut self);
    fn bne(&mut self);
    fn beq(&mut self);
    fn cmp_imm(&mut self);
    fn cmp_zp(&mut self);
    fn cmp_zp_x(&mut self);
    fn cmp_abs(&mut self);
    fn cmp_abs_x(&mut self);
    fn cmp_abs_y(&mut self);
    fn cmp_ind_x(&mut self);
    fn cmp_ind_y(&mut self);
    fn bit_zp(&mut self);
    fn bit_abs(&mut self);
    fn cpx_imm(&mut self);
    fn cpx_zp(&mut self);
    fn cpx_abs(&mut self);
    fn cpy_imm(&mut self);
    fn cpy_zp(&mut self);
    fn cpy_abs(&mut self);
    fn clc(&mut self);
    fn sec(&mut self);
    fn cld(&mut self);
    fn sed(&mut self);
    fn cli(&mut self);
    fn sei(&mut self);
    fn clv(&mut self);
    fn jmp_abs(&mut self);
    fn jmp_ind(&mut self);
    fn rts(&mut self);
    fn jsr(&mut self);
    fn rti(&mut self);
    fn adc_imm(&mut self);
    fn adc_zp(&mut self);
    fn adc_zp_x(&mut self);
    fn adc_abs(&mut self);
    fn adc_abs_x(&mut self);
    fn adc_abs_y(&mut self);
    fn adc_ind_x(&mut self);
    fn adc_ind_y(&mut self);
    fn sbc_imm(&mut self);
    fn sbc_zp(&mut self);
    fn sbc_zp_x(&mut self);
    fn sbc_abs(&mut self);
    fn sbc_abs_x(&mut self);
    fn sbc_abs_y(&mut self);
    fn sbc_ind_x(&mut self);
    fn sbc_ind_y(&mut self);
    fn lda_imm(&mut self);
    fn lda_zp(&mut self);
    fn lda_zp_x(&mut self);
    fn lda_abs(&mut self);
    fn lda_abs_x(&mut self);
    fn lda_abs_y(&mut self);
    fn lda_ind_x(&mut self);
    fn lda_ind_y(&mut self);
    fn sta_zp(&mut self);
    fn sta_zp_x(&mut self);
    fn sta_abs(&mut self);
    fn sta_abs_x(&mut self);
    fn sta_abs_y(&mut self);
    fn sta_ind_x(&mut self);
    fn sta_ind_y(&mut self);
    fn ldx_imm(&mut self);
    fn ldx_zp(&mut self);
    fn ldx_zp_y(&mut self);
    fn ldx_abs(&mut self);
    fn ldx_abs_y(&mut self);
    fn stx_zp(&mut self);
    fn stx_zp_y(&mut self);
    fn stx_abs(&mut self);
    fn ldy_imm(&mut self);
    fn ldy_zp(&mut self);
    fn ldy_zp_x(&mut self);
    fn ldy_abs(&mut self);
    fn ldy_abs_x(&mut self);
    fn sty_zp(&mut self);
    fn sty_zp_x(&mut self);
    fn sty_abs(&mut self);
    fn dec_zp(&mut self);
    fn dec_zp_x(&mut self);
    fn dec_abs(&mut self);
    fn dec_abs_x(&mut self);
    fn inc_zp(&mut self);
    fn inc_zp_x(&mut self);
    fn inc_abs(&mut self);
    fn inc_abs_x(&mut self);
    fn tax(&mut self);
    fn tay(&mut self);
    fn txa(&mut self);
    fn tya(&mut self);
    fn dex(&mut self);
    fn dey(&mut self);
    fn inx(&mut self);
    fn iny(&mut self);
    fn pha(&mut self);
    fn php(&mut self);
    fn txs(&mut self);
    fn pla(&mut self);
    fn tsx(&mut self);
    fn plp(&mut self);
    fn brk(&mut self);
    fn nop(&mut self);

    // Internal functions
    // Many of these are the calculation part of
    // their respective instructions. The above functions
    // take care of memory management and other details
    // for each instruction opcode.


    fn _sty(&mut self, addr: u16);
    fn _stx(&mut self, addr: u16);
    fn _sta(&mut self, addr: u16);
    fn _sbc(&mut self, val: u8);
    fn _ldy(&mut self, val: u8);
    fn _ldx(&mut self, val: u8);
    fn _ror(&mut self, addr: u16);
    fn _rol(&mut self, addr: u16);
    fn _pop(&mut self) -> u8;
    fn _ora(&mut self, val: u8);
    fn _lsr(&mut self, addr: u16);
    fn _lda(&mut self, val: u8);
    fn _inc(&mut self, addr: u16);
    fn _eor(&mut self, val: u8);
    fn _dec(&mut self, addr: u16);
    fn _cpy(&mut self, val: u8);
    fn _cpx(&mut self, val: u8);
    fn _cmp(&mut self, val: u8);
    fn _sf(&mut self, flag: Mos6502Flag, set: bool);
    fn _psh(&mut self, val: u8);
    fn _br(&mut self, flag: Mos6502Flag, set: bool);
    fn _adc(&mut self, val: u8);
    fn _asl(&mut self, addr: u16);

    fn _check_page_boundary_rel(&mut self, addr: u16, rel: i8);
    fn _check_page_boundary(&mut self, addr: u16, off: u8);
    fn _addr_ind_y(&mut self, add_cycle_on_page_boundary: bool) -> u16;
    fn _decode_ind_y(&mut self, add_cycle_on_page_boundary: bool) -> u8;
    fn _set_z_n(&mut self, val: u8);
    fn _decode_imm(&mut self) -> u8;
    fn _decode_zp(&mut self) -> u8;
    fn _addr_zp(&mut self) -> u16;
    fn _decode_zp_x(&mut self) -> u8;
    fn _addr_zp_x(&mut self) -> u16;
    fn _decode_zp_y(&mut self) -> u8;
    fn _addr_zp_y(&mut self) -> u16;
    fn _decode_abs(&mut self) -> u8;
    fn _addr_abs(&mut self) -> u16;
    fn _decode_abs_x(&mut self, add_cycle_on_page_boundary: bool) -> u8;
    fn _addr_abs_x(&mut self, add_cycle_on_page_boundary: bool) -> u16;
    fn _decode_abs_y(&mut self, add_cycle_on_page_boundary: bool) -> u8;
    fn _addr_abs_y(&mut self, add_cycle_on_page_boundary: bool) -> u16;
    fn _decode_ind_x(&mut self) -> u8;
    fn _addr_ind_x(&mut self) -> u16;
}
impl Mos6502Isa for Mos6502<'_> {
    fn load_isa(&mut self) {
        self.isa[0x21] = &Mos6502Isa::and_ind_x;
        self.isa[0x25] = &Mos6502Isa::and_zp;
        self.isa[0x29] = &Mos6502Isa::and_imm;
        self.isa[0x2D] = &Mos6502Isa::and_abs;
        self.isa[0x31] = &Mos6502Isa::and_ind_y;
        self.isa[0x35] = &Mos6502Isa::and_zp_x;
        self.isa[0x39] = &Mos6502Isa::and_abs_y;
        self.isa[0x3D] = &Mos6502Isa::and_abs_x;
        self.isa[0x06] = &Mos6502Isa::asl_zp;
        self.isa[0x0A] = &Mos6502Isa::asl_acc;
        self.isa[0x0E] = &Mos6502Isa::asl_abs;
        self.isa[0x16] = &Mos6502Isa::asl_zp_x;
        self.isa[0x1E] = &Mos6502Isa::asl_abs_x;
        self.isa[0x49] = &Mos6502Isa::eor_imm;
        self.isa[0x45] = &Mos6502Isa::eor_zp;
        self.isa[0x55] = &Mos6502Isa::eor_zp_x;
        self.isa[0x4D] = &Mos6502Isa::eor_abs;
        self.isa[0x5D] = &Mos6502Isa::eor_abs_x;
        self.isa[0x59] = &Mos6502Isa::eor_abs_y;
        self.isa[0x41] = &Mos6502Isa::eor_ind_x;
        self.isa[0x51] = &Mos6502Isa::eor_ind_y;
        self.isa[0x4A] = &Mos6502Isa::lsr_acc;
        self.isa[0x46] = &Mos6502Isa::lsr_zp;
        self.isa[0x56] = &Mos6502Isa::lsr_zp_x;
        self.isa[0x4E] = &Mos6502Isa::lsr_abs;
        self.isa[0x5E] = &Mos6502Isa::lsr_abs_x;
        self.isa[0x09] = &Mos6502Isa::ora_imm;
        self.isa[0x05] = &Mos6502Isa::ora_zp;
        self.isa[0x15] = &Mos6502Isa::ora_zp_x;
        self.isa[0x0D] = &Mos6502Isa::ora_abs;
        self.isa[0x1D] = &Mos6502Isa::ora_abs_x;
        self.isa[0x19] = &Mos6502Isa::ora_abs_y;
        self.isa[0x01] = &Mos6502Isa::ora_ind_x;
        self.isa[0x11] = &Mos6502Isa::ora_ind_y;
        self.isa[0x2A] = &Mos6502Isa::rol_acc;
        self.isa[0x26] = &Mos6502Isa::rol_zp;
        self.isa[0x36] = &Mos6502Isa::rol_zp_x;
        self.isa[0x2E] = &Mos6502Isa::rol_abs;
        self.isa[0x3E] = &Mos6502Isa::rol_abs_x;
        self.isa[0x6A] = &Mos6502Isa::ror_acc;
        self.isa[0x66] = &Mos6502Isa::ror_zp;
        self.isa[0x76] = &Mos6502Isa::ror_zp_x;
        self.isa[0x6E] = &Mos6502Isa::ror_abs;
        self.isa[0x7E] = &Mos6502Isa::ror_abs_x;
        self.isa[0x10] = &Mos6502Isa::bpl;
        self.isa[0x30] = &Mos6502Isa::bmi;
        self.isa[0x50] = &Mos6502Isa::bvc;
        self.isa[0x70] = &Mos6502Isa::bvs;
        self.isa[0x90] = &Mos6502Isa::bcc;
        self.isa[0xB0] = &Mos6502Isa::bcs;
        self.isa[0xD0] = &Mos6502Isa::bne;
        self.isa[0xF0] = &Mos6502Isa::beq;
        self.isa[0xC9] = &Mos6502Isa::cmp_imm;
        self.isa[0xC5] = &Mos6502Isa::cmp_zp;
        self.isa[0xD5] = &Mos6502Isa::cmp_zp_x;
        self.isa[0xCD] = &Mos6502Isa::cmp_abs;
        self.isa[0xDD] = &Mos6502Isa::cmp_abs_x;
        self.isa[0xD9] = &Mos6502Isa::cmp_abs_y;
        self.isa[0xC1] = &Mos6502Isa::cmp_ind_x;
        self.isa[0xD1] = &Mos6502Isa::cmp_ind_y;
        self.isa[0x24] = &Mos6502Isa::bit_zp;
        self.isa[0x2C] = &Mos6502Isa::bit_abs;
        self.isa[0xE0] = &Mos6502Isa::cpx_imm;
        self.isa[0xE4] = &Mos6502Isa::cpx_zp;
        self.isa[0xEC] = &Mos6502Isa::cpx_abs;
        self.isa[0xC0] = &Mos6502Isa::cpy_imm;
        self.isa[0xC4] = &Mos6502Isa::cpy_zp;
        self.isa[0xCC] = &Mos6502Isa::cpy_abs;
        self.isa[0x18] = &Mos6502Isa::clc;
        self.isa[0x38] = &Mos6502Isa::sec;
        self.isa[0xD8] = &Mos6502Isa::cld;
        self.isa[0xF8] = &Mos6502Isa::sed;
        self.isa[0x58] = &Mos6502Isa::cli;
        self.isa[0x78] = &Mos6502Isa::sei;
        self.isa[0xB8] = &Mos6502Isa::clv;
        self.isa[0x4C] = &Mos6502Isa::jmp_abs;
        self.isa[0x6C] = &Mos6502Isa::jmp_ind;
        self.isa[0x60] = &Mos6502Isa::rts;
        self.isa[0x20] = &Mos6502Isa::jsr;
        self.isa[0x40] = &Mos6502Isa::rti;
        self.isa[0x69] = &Mos6502Isa::adc_imm;
        self.isa[0x65] = &Mos6502Isa::adc_zp;
        self.isa[0x75] = &Mos6502Isa::adc_zp_x;
        self.isa[0x6D] = &Mos6502Isa::adc_abs;
        self.isa[0x7D] = &Mos6502Isa::adc_abs_x;
        self.isa[0x79] = &Mos6502Isa::adc_abs_y;
        self.isa[0x61] = &Mos6502Isa::adc_ind_x;
        self.isa[0x71] = &Mos6502Isa::adc_ind_y;
        self.isa[0xE9] = &Mos6502Isa::sbc_imm;
        self.isa[0xE5] = &Mos6502Isa::sbc_zp;
        self.isa[0xF5] = &Mos6502Isa::sbc_zp_x;
        self.isa[0xED] = &Mos6502Isa::sbc_abs;
        self.isa[0xFD] = &Mos6502Isa::sbc_abs_x;
        self.isa[0xF9] = &Mos6502Isa::sbc_abs_y;
        self.isa[0xE1] = &Mos6502Isa::sbc_ind_x;
        self.isa[0xF1] = &Mos6502Isa::sbc_ind_y;
        self.isa[0xA9] = &Mos6502Isa::lda_imm;
        self.isa[0xA5] = &Mos6502Isa::lda_zp;
        self.isa[0xB5] = &Mos6502Isa::lda_zp_x;
        self.isa[0xAD] = &Mos6502Isa::lda_abs;
        self.isa[0xBD] = &Mos6502Isa::lda_abs_x;
        self.isa[0xB9] = &Mos6502Isa::lda_abs_y;
        self.isa[0xA1] = &Mos6502Isa::lda_ind_x;
        self.isa[0xB1] = &Mos6502Isa::lda_ind_y;
        self.isa[0x85] = &Mos6502Isa::sta_zp;
        self.isa[0x95] = &Mos6502Isa::sta_zp_x;
        self.isa[0x8D] = &Mos6502Isa::sta_abs;
        self.isa[0x9D] = &Mos6502Isa::sta_abs_x;
        self.isa[0x99] = &Mos6502Isa::sta_abs_y;
        self.isa[0x81] = &Mos6502Isa::sta_ind_x;
        self.isa[0x91] = &Mos6502Isa::sta_ind_y;
        self.isa[0xA2] = &Mos6502Isa::ldx_imm;
        self.isa[0xA6] = &Mos6502Isa::ldx_zp;
        self.isa[0xB6] = &Mos6502Isa::ldx_zp_y;
        self.isa[0xAE] = &Mos6502Isa::ldx_abs;
        self.isa[0xBE] = &Mos6502Isa::ldx_abs_y;
        self.isa[0x86] = &Mos6502Isa::stx_zp;
        self.isa[0x96] = &Mos6502Isa::stx_zp_y;
        self.isa[0x8E] = &Mos6502Isa::stx_abs;
        self.isa[0xA0] = &Mos6502Isa::ldy_imm;
        self.isa[0xA4] = &Mos6502Isa::ldy_zp;
        self.isa[0xB4] = &Mos6502Isa::ldy_zp_x;
        self.isa[0xAC] = &Mos6502Isa::ldy_abs;
        self.isa[0xBC] = &Mos6502Isa::ldy_abs_x;
        self.isa[0x84] = &Mos6502Isa::sty_zp;
        self.isa[0x94] = &Mos6502Isa::sty_zp_x;
        self.isa[0x8C] = &Mos6502Isa::sty_abs;
        self.isa[0xC6] = &Mos6502Isa::dec_zp;
        self.isa[0xD6] = &Mos6502Isa::dec_zp_x;
        self.isa[0xCE] = &Mos6502Isa::dec_abs;
        self.isa[0xDE] = &Mos6502Isa::dec_abs_x;
        self.isa[0xE6] = &Mos6502Isa::inc_zp;
        self.isa[0xF6] = &Mos6502Isa::inc_zp_x;
        self.isa[0xEE] = &Mos6502Isa::inc_abs;
        self.isa[0xFE] = &Mos6502Isa::inc_abs_x;
        self.isa[0xAA] = &Mos6502Isa::tax;
        self.isa[0xA8] = &Mos6502Isa::tay;
        self.isa[0x8A] = &Mos6502Isa::txa;
        self.isa[0x98] = &Mos6502Isa::tya;
        self.isa[0xCA] = &Mos6502Isa::dex;
        self.isa[0x88] = &Mos6502Isa::dey;
        self.isa[0xE8] = &Mos6502Isa::inx;
        self.isa[0xC8] = &Mos6502Isa::iny;
        self.isa[0x48] = &Mos6502Isa::pha;
        self.isa[0x08] = &Mos6502Isa::php;
        self.isa[0x9A] = &Mos6502Isa::txs;
        self.isa[0x68] = &Mos6502Isa::pla;
        self.isa[0xBA] = &Mos6502Isa::tsx;
        self.isa[0x28] = &Mos6502Isa::plp;
        self.isa[0x00] = &Mos6502Isa::brk;
        self.isa[0xEA] = &Mos6502Isa::nop;
    }
    // Utility functions
    fn _set_z_n(&mut self, val: u8) {
        self.ps = (self.ps & 0x7D)
            | if val != 0 {
                val & (Mos6502Flag::N as u8)
            } else {
                Mos6502Flag::Z as u8
            };
    }



    // Decode functions return the value from the opcode param, based on addressing mode.
    // This can be either the opcode value itself, or the value pointed to by the opcode.

    // Addr functions return the address that would need to be accessed.
    // ONLY ONE of decode or addr should be called, and only once per instruction.
    fn _decode_imm(&mut self) -> u8 {
        self.bus
            .get({
                let t = self.pc;
                self.pc += 1;
                t
            })
            .unwrap()
    }

    fn _decode_zp(&mut self) -> u8 {
        let addr = self._addr_zp();
        self.getmem(addr)
    }
    fn _addr_zp(&mut self) -> u16 {
        self.bus
            .get({
                let t = self.pc;
                self.pc += 1;
                t
            })
            .unwrap() as u16
    }

    fn _decode_zp_x(&mut self) -> u8 {
        let addr = self._addr_zp_x();
        self.getmem(addr)
    }
    #[allow(arithmetic_overflow)]
    fn _addr_zp_x(&mut self) -> u16 {
        (self
            .bus
            .get({
                let t = self.pc;
                self.pc += 1;
                t
            })
            .unwrap() as u16
            + self.x as u16) % 256
    }

    fn _decode_zp_y(&mut self) -> u8 {
        let addr = self._addr_zp_y();
        self.getmem(addr)
    }
    #[allow(arithmetic_overflow)]
    fn _addr_zp_y(&mut self) -> u16 {
        ((self
            .bus
            .get({
                let t = self.pc;
                self.pc += 1;
                t
            })
            .unwrap() as u16)
            + (self.y as u16)) % 256
    }

    fn _decode_abs(&mut self) -> u8 {
        let addr = self._addr_abs();
        self.getmem(addr)
    }
    fn _addr_abs(&mut self) -> u16 {
        let a = self.pc;
        self.pc += 2;
        self._fetch_u16(a)
    }

    fn _decode_abs_x(&mut self, add_cycle_on_page_boundary: bool) -> u8 {
        let addr = self._addr_abs_x(add_cycle_on_page_boundary);
        self.getmem(addr)
    }
    fn _addr_abs_x(&mut self, add_cycle_on_page_boundary: bool) -> u16 {
        let addr = self._fetch_u16(self.pc);
        self.pc += 2;
        if add_cycle_on_page_boundary {
            self._check_page_boundary(addr, self.x);
        }
        addr + (self.x as u16)
    }

    fn _decode_abs_y(&mut self, add_cycle_on_page_boundary: bool) -> u8 {
        let addr = self._addr_abs_y(add_cycle_on_page_boundary);
        self.getmem(addr)
    }

    fn _addr_abs_y(&mut self, add_cycle_on_page_boundary: bool) -> u16 {
        let addr = self._fetch_u16(self.pc);
        self.pc += 2;
        if add_cycle_on_page_boundary {
            self._check_page_boundary(addr, self.y);
        }
        addr + (self.y as u16)
    }

    fn _decode_ind_x(&mut self) -> u8 {
        let addr = self._addr_ind_x();
        let ret = self.getmem(addr);
        ret
    }
    #[allow(arithmetic_overflow)]
    fn _addr_ind_x(&mut self) -> u16 {
        let t = self.pc;
        self.pc += 1;
        let param = self.getmem(t) as u16;

        // Wrap around for x-indexed zero page? Is this a bug, or is it a feature?
        let addr = (self.getmem((param + (self.x as u16)) % 256) as u16)
            + ((self.getmem((param + (self.x as u16) + 1) % 256) as u16)<<8);
        addr
    }

    fn _decode_ind_y(&mut self, add_cycle_on_page_boundary: bool) -> u8 {
        let addr = self._addr_ind_y(add_cycle_on_page_boundary);
        let ret = self.getmem(addr);
        ret
    }
    fn _addr_ind_y(&mut self, add_cycle_on_page_boundary: bool) -> u16 {
        let t = self.pc;
        self.pc += 1;
        let param = self.getmem(t) as u16;

        // Wrap around for y-indexed zero page? Is this what the 6502 does?
        // If not, replace with _fetch_u16 @ 
        let addr = (self.getmem((param) % 256) as u16)
            + ((self.getmem((param + 1) % 256) as u16)<<8);
        if add_cycle_on_page_boundary {
            self._check_page_boundary(addr, self.y);
        }
        //println!("addr {} + {} from [{}]", addr, self.y, t);
        addr + (self.y as u16)
    }
    fn _check_page_boundary(&mut self, addr: u16, off: u8) {
        // If we cross a page boundary, add 1 cycle to latency
        if addr % 256 + (off as u16) > 256 {
            self.cycles += 1;
        }
    }
    fn _check_page_boundary_rel(&mut self, addr: u16, rel: i8) {
        // If we cross a page boundary, add 1 cycle to latency
        if (addr as i16) + (rel as i16) >> 8 != (addr as i16) >> 8 {
            self.cycles += 1;
        }
    }

    // Instruction calculator functions
    //
    // These functions are meant to be called by instruction functions
    // to perform common calculations.
    fn _pop(&mut self) -> u8 {
        self.bus
            .get(
                0x100 + {
                    let t = self.pc;
                    self.pc += 1;
                    t
                },
            )
            .unwrap()
    }
    fn _rol(&mut self, addr: u16) {
        let mut val: u8 = self.getmem(addr);
        let carry: bool = val & 0x80 != 0;
        val <<= 1;
        val |= self.ps & (Mos6502Flag::C as u8);
        self._sf(Mos6502Flag::C, carry);
        self._set_z_n(val);
        self.setmem(addr, val);
    }
    fn _ror(&mut self, addr: u16) {
        let mut val: u8 = self.getmem(addr);
        let carry: bool = val & 0x01 != 0;
        val >>= 1;
        val |= (self.ps & (Mos6502Flag::C as u8)) << 7;
        self._sf(Mos6502Flag::C, carry);
        self._set_z_n(val);
        self.setmem(addr, val);
    }
    fn _sta(&mut self, addr: u16) {
        self.setmem(addr, self.a);
    }
    fn _stx(&mut self, addr: u16) {
        self.setmem(addr, self.x);
    }
    fn _sty(&mut self, addr: u16) {
        self.setmem(addr, self.y);
    }
    #[allow(arithmetic_overflow)]
    fn _sbc(&mut self, val: u8) {
        //TODO: calculating twice is almost certainly unnecessary
        let t: i16 =
            (self.a as i16) - (val as i16) - ((1 - (self.ps & (Mos6502Flag::C as u8))) as i16);
        let v: u8 = self.a - val - !(self.ps & (Mos6502Flag::C as u8));

        // --WARNING: possible bug here--
        // using an existing emulator, you can see the following:
        // [V] is set when overflowing, unset otherwise.
        // [C] is *unset* only when the accumulator changes sign.
        // I think those are the only rules governing the flags...
        self._sf(Mos6502Flag::C, self.a & 0x80 == v & 0x80);
        self._sf(Mos6502Flag::V, t < -128 || t > 127);
        self.a = v;
        self._set_z_n(self.a);
    }
    fn _asl(&mut self, addr: u16) {
        let mut i = self.getmem(addr);
        self.ps = (self.ps & !(Mos6502Flag::C as u8)) | (i & (Mos6502Flag::C as u8));
        i <<= 1;
        self._set_z_n(i);
        self.setmem(addr, i);
    }
    fn _adc(&mut self, val: u8) {
        //TODO: ensure edge cases work appropriately here; probably needs optimising too
        if self.ps & (Mos6502Flag::D as u8) != 0 {
            // The absolute maximum value for lo and ho (low digit / high digit) is:
            //      15 + 15 + 1 = 31 or 0x1F
            // The logical maximum value should be 9 + 9 + 1 = 19 or 0x13
            // Not sure how the 6502 handles invalid BCD values, but we'll
            // only carry 1 when lo/ho is > 9
            let mut lo = (self.a & 0x0F) + (self.ps & (Mos6502Flag::C as u8)) + (val & 0x0F);
            let mut ho = (self.a & 0xF0) >> 4 + (val & 0xF0) >> 4;
            if lo > 9 {
                ho += 1;
                lo %= 10;
            }
            if ho > 9 {
                self.ps |= Mos6502Flag::C as u8;
                ho %= 10;
            }
            self.a = ho << 4 + lo;
        } else {
            let res: u16 = (self.a as u16) + (val as u16) + ((self.ps & (Mos6502Flag::C as u8)) as u16);
            //println!("a{} c{} i{} = {}",self.a,self.ps & 0x01,val,res);
            self.ps = (self.ps & !(Mos6502Flag::C as u8)) | if res >= 256 { Mos6502Flag::C as u8 } else { 0 };
            self.a = (res % 256) as u8;
        }
        self._set_z_n(self.a);
    }
    fn _br(&mut self, flag: Mos6502Flag, set: bool) {
        if ((self.ps & flag as u8) != 0) == set {
            self.cycles = 3;
            let rel: i8 = self._decode_imm() as i8;
            self._check_page_boundary_rel(self.pc, rel);
            self.pc += rel as u16;
        } else {
            self.cycles = 2;
        }
    }
    fn _psh(&mut self, val: u8) {
        self.setmem(0x100 + (self.sp as u16), val);
        self.sp -= 1;
    }
    fn _sf(&mut self, flag: Mos6502Flag, set: bool) {
        let f: u8 = flag as u8;
        self.ps &= !f | if set { f } else { 0 };
    }
    fn _cmp(&mut self, val: u8) {
        self._sf(Mos6502Flag::C, self.a > val);
        self._set_z_n(self.a - val);
    }
    fn _cpx(&mut self, val: u8) {
        self._sf(Mos6502Flag::C, self.x > val);
        self._set_z_n(self.x - val);
    }
    fn _cpy(&mut self, val: u8) {
        self._sf(Mos6502Flag::C, self.y > val);
        self._set_z_n(self.y - val);
    }
    fn _dec(&mut self, addr: u16) {
        let val = self.getmem(addr) - 1;
        self._set_z_n(val);
        self.setmem(addr, val);
    }
    fn _eor(&mut self, val: u8) {
        self.a ^= val;
        self._set_z_n(self.a);
    }
    fn _inc(&mut self, addr: u16) {
        let val = self.getmem(addr) + 1;
        self._set_z_n(val);
        self.setmem(addr, val);
    }
    fn _lda(&mut self, val: u8) {
        self.a = val;
        self._set_z_n(self.a);
    }
    fn _ldx(&mut self, val: u8) {
        self.x = val;
        self._set_z_n(self.x);
    }
    fn _ldy(&mut self, val: u8) {
        self.y = val;
        self._set_z_n(self.y);
    }
    fn _lsr(&mut self, addr: u16) {
        let mut val = self.getmem(addr);
        self._sf(Mos6502Flag::C, val & 0x01 != 0);
        val >>= 1;
        self._set_z_n(self.a);
        self.setmem(addr, val);
    }
    fn _ora(&mut self, val: u8) {
        self.a |= val;
        self._set_z_n(self.a);
    }

    // Instruction functions
    //
    // Call one of these to simulate an instruction.
    fn and_imm(&mut self) {
        self.cycles = 2;
        self.a &= self._decode_imm();
        self._set_z_n(self.a);
    }
    fn and_zp(&mut self) {
        self.cycles = 3;
        self.a &= self._decode_zp();
        self._set_z_n(self.a);
    }
    fn and_zp_x(&mut self) {
        self.cycles = 4;
        self.a &= self._decode_zp_x();
        self._set_z_n(self.a);
    }
    fn and_abs(&mut self) {
        self.cycles = 4;
        self.a &= self._decode_abs();
        self._set_z_n(self.a);
    }
    fn and_abs_x(&mut self) {
        self.cycles = 4;
        self.a &= self._decode_abs_x(true);
        self._set_z_n(self.a);
    }
    fn and_abs_y(&mut self) {
        self.cycles = 4;
        self.a &= self._decode_abs_y(true);
        self._set_z_n(self.a);
    }
    fn and_ind_x(&mut self) {
        self.cycles = 6;
        self.a &= self._decode_ind_x();
        self._set_z_n(self.a);
    }
    fn and_ind_y(&mut self) {
        self.cycles = 5;
        self.a &= self._decode_ind_y(true);
        self._set_z_n(self.a);
    }

    fn asl_acc(&mut self) {
        self.cycles = 2;
        self.ps = (self.ps & !(Mos6502Flag::C as u8)) | (self.a & (Mos6502Flag::C as u8));
        self.a <<= 1;
        self._set_z_n(self.a);
    }
    fn asl_zp(&mut self) {
        self.cycles = 5;
        let addr = self._addr_zp();
        self._asl(addr);
    }
    fn asl_zp_x(&mut self) {
        self.cycles = 6;
        let addr = self._addr_zp_x();
		self._asl(addr);
    }
    fn asl_abs(&mut self) {
        self.cycles = 6;
        let addr = self._addr_abs();
		self._asl(addr);
    }
    fn asl_abs_x(&mut self) {
        self.cycles = 7;
        let addr = self._addr_abs_x(false);
		self._asl(addr);
    }

    fn adc_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._adc(val);
    }
    fn adc_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._adc(val);
    }
    fn adc_zp_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_zp_x();
		self._adc(val);
    }
    fn adc_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._adc(val);
    }
    fn adc_abs_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_x(true);
		self._adc(val);
    }
    fn adc_abs_y(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_y(true);
		self._adc(val);
    }
    fn adc_ind_x(&mut self) {
        self.cycles = 6;
        let val = self._decode_ind_x();
		self._adc(val);
    }
    fn adc_ind_y(&mut self) {
        self.cycles = 5;
        let val = self._decode_ind_y(true);
		self._adc(val);
    }

    fn bcs(&mut self) {
        self._br(Mos6502Flag::C, true)
    }
    fn bcc(&mut self) {
        self._br(Mos6502Flag::C, false)
    }
    fn beq(&mut self) {
        self._br(Mos6502Flag::Z, true)
    }
    fn bne(&mut self) {
        self._br(Mos6502Flag::Z, false)
    }
    fn bmi(&mut self) {
        self._br(Mos6502Flag::N, true)
    }
    fn bpl(&mut self) {
        self._br(Mos6502Flag::N, false)
    }
    fn bvs(&mut self) {
        self._br(Mos6502Flag::V, true)
    }
    fn bvc(&mut self) {
        self._br(Mos6502Flag::V, false)
    }

    fn bit_zp(&mut self) {
        self.cycles = 3;
        let t = self.a & self._decode_zp();
        self.ps = (self.ps & 0x3F) | (t & 0xC0);
    }
    fn bit_abs(&mut self) {
        self.cycles = 4;
        let t = self.a & self._decode_abs();
        self.ps = (self.ps & 0x3F) | (t & 0xC0);
    }

    fn brk(&mut self) {
        self.cycles = 7;
        self._psh((self.pc >> 8) as u8);
        self._psh((self.pc % 256) as u8);
        self._psh(self.ps);
        self.pc = 0xFFFE;
        self._sf(Mos6502Flag::B, true);
    }

    fn clc(&mut self) {
        self.cycles = 2;
        self._sf(Mos6502Flag::C, false);
    }
    fn cld(&mut self) {
        self.cycles = 2;
        self._sf(Mos6502Flag::D, false);
    }
    fn cli(&mut self) {
        self.cycles = 2;
        self._sf(Mos6502Flag::I, false);
    }
    fn clv(&mut self) {
        self.cycles = 2;
        self._sf(Mos6502Flag::V, false);
    }

    fn cmp_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._cmp(val);
    }
    fn cmp_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._cmp(val);
    }
    fn cmp_zp_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_zp_x();
		self._cmp(val);
    }
    fn cmp_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._cmp(val);
    }
    fn cmp_abs_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_x(true);
		self._cmp(val);
    }
    fn cmp_abs_y(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_y(true);
		self._cmp(val);
    }
    fn cmp_ind_x(&mut self) {
        self.cycles = 6;
        let val = self._decode_ind_x();
		self._cmp(val);
    }
    fn cmp_ind_y(&mut self) {
        self.cycles = 5;
        let val = self._decode_ind_y(true);
		self._cmp(val);
    }

    fn cpx_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._cpx(val);
    }
    fn cpx_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._cpx(val);
    }
    fn cpx_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._cpx(val);
    }

    fn cpy_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._cpy(val);
    }
    fn cpy_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._cpy(val);
    }
    fn cpy_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._cpy(val);
    }

    fn dec_zp(&mut self) {
        self.cycles = 5;
        let addr = self._addr_zp();
		self._dec(addr);
    }
    fn dec_zp_x(&mut self) {
        self.cycles = 6;
        let addr = self._addr_zp_x();
		self._dec(addr);
    }
    fn dec_abs(&mut self) {
        self.cycles = 6;
        let addr = self._addr_abs();
		self._dec(addr);
    }
    fn dec_abs_x(&mut self) {
        self.cycles = 7;
        let addr = self._addr_abs_x(false);
		self._dec(addr);
    }

    fn dex(&mut self) {
        self.cycles = 2;
        self.x -= 1;
        self._set_z_n(self.x);
    }
    fn dey(&mut self) {
        self.cycles = 2;
        self.y -= 1;
        self._set_z_n(self.y);
    }

    fn eor_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._eor(val);
    }
    fn eor_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._eor(val);
    }
    fn eor_zp_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_zp_x();
		self._eor(val);
    }
    fn eor_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._eor(val);
    }
    fn eor_abs_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_x(true);
		self._eor(val);
    }
    fn eor_abs_y(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_y(true);
		self._eor(val);
    }
    fn eor_ind_x(&mut self) {
        self.cycles = 6;
        let val = self._decode_ind_x();
		self._eor(val);
    }
    fn eor_ind_y(&mut self) {
        self.cycles = 5;
        let val = self._decode_ind_y(true);
		self._eor(val);
    }

    fn inc_zp(&mut self) {
        self.cycles = 5;
        let addr = self._addr_zp();
		self._inc(addr);
    }
    fn inc_zp_x(&mut self) {
        self.cycles = 6;
        let addr = self._addr_zp_x();
		self._inc(addr);
    }
    fn inc_abs(&mut self) {
        self.cycles = 6;
        let addr = self._addr_abs();
		self._inc(addr);
    }
    fn inc_abs_x(&mut self) {
        self.cycles = 7;
        let addr = self._addr_abs_x(false);
		self._inc(addr);
    }

    fn inx(&mut self) {
        self.cycles = 2;
        self.x += 1;
        self._set_z_n(self.x);
    }
    fn iny(&mut self) {
        self.cycles = 2;
        self.y += 1;
        self._set_z_n(self.y);
    }

    fn jmp_abs(&mut self) {
        self.cycles = 3;
        let addr = self._addr_abs();
        self.pc = addr;
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
        let addr = self._addr_abs();
        self.pc = self._fetch_u16(addr);
        // println!("{} -> {}", addr, self.pc);
        //TODO: add code to simulate a bug?
    }

    fn jsr(&mut self) {
        self.cycles = 6;
        // --WARNING: possible bug here--
        // JSR pushes the "return address - 1" onto the stack. Is
        // this to compensate for hardware incrementing it somewhere?

        // return address would be self.pc + 2
        let ret: u16 = self.pc + 1;
        self._psh((ret >> 8) as u8);
        self._psh((ret % 256) as u8);
        self.pc = self._addr_abs();
    }

    fn lda_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._lda(val);
    }
    fn lda_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._lda(val);
    }
    fn lda_zp_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_zp_x();
		self._lda(val);
    }
    fn lda_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._lda(val);
    }
    fn lda_abs_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_x(true);
		self._lda(val);
    }
    fn lda_abs_y(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_y(true);
		self._lda(val);
    }
    fn lda_ind_x(&mut self) {
        self.cycles = 6;
        let val = self._decode_ind_x();
		self._lda(val);
    }
    fn lda_ind_y(&mut self) {
        self.cycles = 5;
        let val = self._decode_ind_y(true);
		self._lda(val);
    }

    fn ldx_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._ldx(val);
    }
    fn ldx_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._ldx(val);
    }
    fn ldx_zp_y(&mut self) {
        self.cycles = 4;
        let val = self._decode_zp_y();
		self._ldx(val);
    }
    fn ldx_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._ldx(val);
    }
    fn ldx_abs_y(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_y(true);
		self._ldx(val);
    }

    fn ldy_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._ldy(val);
    }
    fn ldy_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._ldy(val);
    }
    fn ldy_zp_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_zp_x();
		self._ldy(val);
    }
    fn ldy_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._ldy(val);
    }
    fn ldy_abs_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_x(true);
		self._ldy(val);
    }

    fn lsr_acc(&mut self) {
        self.cycles = 2;
        self._sf(Mos6502Flag::C, self.a & 0x01 != 0);
        self.a >>= 1;
        self._set_z_n(self.a);
    }

    fn lsr_zp(&mut self) {
        self.cycles = 5;
        let addr = self._addr_zp();
		self._lsr(addr);
    }
    fn lsr_zp_x(&mut self) {
        self.cycles = 6;
        let addr = self._addr_zp_x();
		self._lsr(addr);
    }
    fn lsr_abs(&mut self) {
        self.cycles = 6;
        let addr = self._addr_abs();
		self._lsr(addr);
    }
    fn lsr_abs_x(&mut self) {
        self.cycles = 7;
        let addr = self._addr_abs_x(false);
		self._lsr(addr);
    }

    fn nop(&mut self) {
        self.cycles = 2;
    }

    fn ora_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._ora(val);
    }
    fn ora_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._ora(val);
    }
    fn ora_zp_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_zp_x();
		self._ora(val);
    }
    fn ora_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._ora(val);
    }
    fn ora_abs_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_x(true);
		self._ora(val);
    }
    fn ora_abs_y(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_y(true);
		self._ora(val);
    }
    fn ora_ind_x(&mut self) {
        self.cycles = 6;
        let val = self._decode_ind_x();
		self._ora(val);
    }
    fn ora_ind_y(&mut self) {
        self.cycles = 5;
        let val = self._decode_ind_y(true);
		self._ora(val);
    }

    fn pha(&mut self) {
        self.cycles = 3;
        self._psh(self.a);
    }

    fn php(&mut self) {
        self.cycles = 3;
        self._psh(self.ps);
    }

    fn rol_acc(&mut self) {
        let carry: bool = self.a & 0x80 != 0;
        self.a <<= 1;
        self.a |= self.ps & (Mos6502Flag::C as u8);
        self._sf(Mos6502Flag::C, carry);
        self._set_z_n(self.a);
    }

    fn pla(&mut self) {
        self.cycles = 4;
        self.a = self._pop();
    }
    fn plp(&mut self) {
        self.cycles = 4;
        self.ps = self._pop();
    }

    fn rol_abs(&mut self) {
        self.cycles = 6;
        let addr = self._addr_abs();
		self._rol(addr);
    }
    fn rol_abs_x(&mut self) {
        self.cycles = 7;
        let addr = self._addr_abs_x(false);
		self._rol(addr);
    }
    fn rol_zp(&mut self) {
        self.cycles = 5;
        let addr = self._addr_zp();
		self._rol(addr);
    }
    fn rol_zp_x(&mut self) {
        self.cycles = 6;
        let addr = self._addr_zp_x();
		self._rol(addr);
    }

    fn ror_acc(&mut self) {
        let carry: bool = self.a & 0x80 != 0;
        self.a <<= 1;
        self.a |= self.ps & (Mos6502Flag::C as u8);
        self._sf(Mos6502Flag::C, carry);
        self._set_z_n(self.a);
    }

    fn ror_abs(&mut self) {
        self.cycles = 6;
        let addr = self._addr_abs();
		self._ror(addr);
    }
    fn ror_abs_x(&mut self) {
        self.cycles = 7;
        let addr = self._addr_abs_x(false);
		self._ror(addr);
    }
    fn ror_zp(&mut self) {
        self.cycles = 5;
        let addr = self._addr_zp();
		self._ror(addr);
    }
    fn ror_zp_x(&mut self) {
        self.cycles = 6;
        let addr = self._addr_zp_x();
		self._ror(addr);
    }

    fn rti(&mut self) {
        self.cycles = 6;
        self.ps = self._pop();
        self.pc = (self._pop() as u16) + (self._pop() as u16) << 8;
    }

    // JSR pushes "return address - 1", so we increment on
    // popping it from the stack.

    fn rts(&mut self) {
        self.cycles = 6;
        self.pc = 1 + (self._pop() as u16) + (self._pop() as u16) << 8;
    }

    fn sbc_imm(&mut self) {
        self.cycles = 2;
        let val = self._decode_imm();
		self._sbc(val);
    }
    fn sbc_zp(&mut self) {
        self.cycles = 3;
        let val = self._decode_zp();
		self._sbc(val);
    }
    fn sbc_zp_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_zp_x();
		self._sbc(val);
    }
    fn sbc_abs(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs();
		self._sbc(val);
    }
    fn sbc_abs_x(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_x(true);
		self._sbc(val);
    }
    fn sbc_abs_y(&mut self) {
        self.cycles = 4;
        let val = self._decode_abs_y(true);
		self._sbc(val);
    }
    fn sbc_ind_x(&mut self) {
        self.cycles = 6;
        let val = self._decode_ind_x();
		self._sbc(val);
    }
    fn sbc_ind_y(&mut self) {
        self.cycles = 5;
        let val = self._decode_ind_y(true);
		self._sbc(val);
    }

    fn sec(&mut self) {
        self._sf(Mos6502Flag::C, true);
    }
    fn sed(&mut self) {
        self._sf(Mos6502Flag::D, true);
    }
    fn sei(&mut self) {
        self._sf(Mos6502Flag::I, true);
    }

    fn sta_zp(&mut self) {
        self.cycles = 3;
        let addr = self._addr_zp();
		self._sta(addr);
    }
    fn sta_zp_x(&mut self) {
        self.cycles = 4;
        let addr = self._addr_zp_x();
		self._sta(addr);
    }
    fn sta_abs(&mut self) {
        self.cycles = 4;
        let addr = self._addr_abs();
		self._sta(addr);
    }
    fn sta_abs_x(&mut self) {
        self.cycles = 5;
        let addr = self._addr_abs_x(false);
		self._sta(addr);
    }
    fn sta_abs_y(&mut self) {
        self.cycles = 5;
        let addr = self._addr_abs_y(false);
		self._sta(addr);
    }
    fn sta_ind_x(&mut self) {
        self.cycles = 6;
        let addr = self._addr_ind_x();
		self._sta(addr);
    }
    fn sta_ind_y(&mut self) {
        self.cycles = 6;
        let addr = self._addr_ind_y(false);
		self._sta(addr);
    }

    fn stx_zp(&mut self) {
        self.cycles = 3;
        let addr = self._addr_zp();
		self._stx(addr);
    }
    fn stx_zp_y(&mut self) {
        self.cycles = 4;
        let addr = self._addr_zp_y();
		self._stx(addr);
    }
    fn stx_abs(&mut self) {
        self.cycles = 4;
        let addr = self._addr_abs();
		self._stx(addr);
    }

    fn sty_zp(&mut self) {
        self.cycles = 3;
        let addr = self._addr_zp();
		self._sty(addr);
    }
    fn sty_zp_x(&mut self) {
        self.cycles = 4;
        let addr = self._addr_zp_x();
		self._sty(addr);
    }
    fn sty_abs(&mut self) {
        self.cycles = 4;
        let addr = self._addr_abs();
		self._sty(addr);
    }

    fn tax(&mut self) {
        self.cycles = 2;
        self.x = self.a;
        self._set_z_n(self.x);
    }
    fn tay(&mut self) {
        self.cycles = 2;
        self.y = self.a;
        self._set_z_n(self.y);
    }
    fn tsx(&mut self) {
        self.cycles = 2;
        self.x = self.sp;
        self._set_z_n(self.x);
    }
    fn txa(&mut self) {
        self.cycles = 2;
        self.a = self.x;
        self._set_z_n(self.a);
    }
    fn txs(&mut self) {
        self.cycles = 2;
        self.sp = self.x;
    }
    fn tya(&mut self) {
        self.cycles = 2;
        self.a = self.y;
        self._set_z_n(self.a);
    }
}
