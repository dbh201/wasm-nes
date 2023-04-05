#![allow(non_camel_case_types)]
#![allow(dead_code)]
use crate::{Mainbus::Mainbus, MmioNode::MmioObject};
use std::cell::RefMut;

#[repr(u8)]
pub enum PPUCtrlFlag {
    NAMETABLE_BIT0 = 0x01,
    NAMETABLE_BIT1 = 0x02,
    INCREMENT_MODE = 0x04,
    SPR_TILE_SELECT = 0x08,
    BG_TILE_SELECT = 0x10,
    SPRITE_HEIGHT = 0x20,
    EXT_READ_WRITE = 0x40,
    VBLANK = 0x80
}
#[repr(u8)]
pub enum PPUMaskFlag {
    GREYSCALE = 0x01,
    BG_LEFT_COLUMN = 0x02,
    SPR_LEFT_COLUMN = 0x04,
    BG_ENABLE = 0x08,
    SPR_ENABLE = 0x10,
    RED_EMPHASIS = 0x20,
    GREEN_EMPHASIS = 0x40,
    BLUE_EMPHASIS = 0x80
}
#[repr(u8)]
pub enum PPUStatusFlag {
    SPR_OVERFLOW = 0x20,
    SPR_HIT = 0x40,
    VBLANK_STARTED = 0x80,
}
pub struct PPU<'a> {
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    oamdata: u8,
    ppuscroll: u8,
    ppuaddr: u8,
    vmemaddr: u16,
    ppudata: u8,
    oamdma: u8,

    oam_data: [u8;256],
    ppuaddr_byte2: bool,
    ppuscroll_byte2: bool,
    mmu: RefMut<'a, Mainbus<'a>>,
    bus: RefMut<'a, Mainbus<'a>>
}

impl<'a> PPU<'_> {
    pub fn new(bus: RefMut<'a, Mainbus<'a>>, mmu: RefMut<'a, Mainbus<'a>>) -> Result<PPU<'a>,String> {
        Ok(PPU {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            oamdata: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            vmemaddr: 0,
            ppudata: 0,
            oamdma: 0,
            
            oam_data: [0;256], 
            ppuaddr_byte2: false,
            ppuscroll_byte2: false,
            mmu,
            bus
        })
    }
    pub fn ctrl_flag(&self, f: PPUCtrlFlag) -> bool {
        self.ppuctrl & f as u8 != 0
    }
    pub fn mask_flag(&self, f: PPUMaskFlag) -> bool {
        self.ppumask & f as u8 != 0
    }
    pub fn status_flag(&self, f: PPUStatusFlag) -> bool {
        self.ppustatus & f as u8 != 0
    }
}
// Addresses are relative to addr start (0x2000)
impl MmioObject for PPU<'_> {
    fn get(&self, addr: u16) -> Result<u8, String> {
        self.mmu.get(addr)
    }
    fn set(&mut self, addr: u16, val: u8) -> Result<(), String> {
        match addr {
            0x0000 => {

            }
            0x0001 => {

            }
            0x0003 => {
                self.oamaddr = val;
            }
            0x0004 => {
                self.oam_data[self.oamaddr as usize] = val;
                self.oamaddr = (((self.oamaddr as u16) + 1) % 256) as u8;
            }
            0x0005 => {
                if !self.ppuscroll_byte2 {
                    self.ppuscroll_byte2 = true;
                    self.ppuscroll = val;
                } else {

                }
            }
            0x0006 => {
                if !self.ppuaddr_byte2 {
                    self.ppuaddr_byte2 = true;
                    self.ppuaddr = val;
                } else {
                    self.vmemaddr = ((self.ppuaddr as u16)<<8) + (val as u16);
                    self.ppuaddr_byte2 = false;
                }
            },
            0x0007 => {
                let ret = self.mmu.set(self.vmemaddr,val);
                if self.ctrl_flag(PPUCtrlFlag::INCREMENT_MODE) {
                    self.vmemaddr += 32;
                } else {
                    self.vmemaddr += 1;
                }
                return ret
            },
            0x2014 => {
                //OAMDMA! How to transfer from CPU MMU to PPU MMU ?
            }
            _ => {
                return Err(format!("PPU: attempt to set addr @{:04X}={:02X} failed (not owned)",addr,val));
            }
        }
        self.mmu.set(addr,val)
    }
    fn len(&self) -> usize {
        9 // External access is limited to 0x2000..0x2007 or 0x4014
    }
}