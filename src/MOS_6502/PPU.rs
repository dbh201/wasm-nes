#![allow(non_camel_case_types)]
#![allow(dead_code)]
use super::{AddressBus::AddressBus, MmioNode::MmioObject};
use std::{cell::{RefCell}, rc::Rc};

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
    ppuscroll: u8,
    ppuaddr: u8,
    vmemaddr: u16,
    ppudata: u8,
    oamdma: u8,

    oamdata: [u8;256],
    ppuaddr_byte2: bool,
    ppuscroll_byte2: bool,
    mmu: Rc<RefCell<AddressBus<'a>>>,
    bus: Rc<RefCell<AddressBus<'a>>>
}

impl<'a> PPU<'_> {
    pub fn new(bus: Rc<RefCell<AddressBus<'a>>>, mmu: Rc<RefCell<AddressBus<'a>>>) -> Result<PPU<'a>,String> {
        Ok(PPU {
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0,
            oamaddr: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            vmemaddr: 0,
            ppudata: 0,
            oamdma: 0,
            
            oamdata: [0;256], 
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
    fn check_ctrl(&mut self) -> Result<(),String> {
        //TODO: do stuff related to ppuctrl
        Ok(())
    }
    fn check_mask(&mut self) -> Result<(),String> {
        //TODO: do stuff related to ppumask
        Ok(())
    }
    pub fn step(&mut self) -> Result<(),String> {
        // TODO: calculate and possibly render a frame
        // This might need to be implemented somewhere else in a trait,
        // so that different rendering engines can be used.
        Ok(())
    }
    pub fn clock_tick(&mut self) -> Result<(), String> {
        // This may or may not be equivalent to step()
        // Some things may take more than one clock tick!
        Ok(())
    }
    fn clear_vblank(&mut self) {
        self.ppustatus &= !(PPUStatusFlag::VBLANK_STARTED as u8);
    }
}
/*
trait Renderer {
    fn render(&self);
}
impl Renderer for PPU<'_> {
    fn render(&self) {
        
    }
}
*/
// Addresses are relative to addr start (0x2000)
impl MmioObject for PPU<'_> {
    fn get(&mut self, addr: u16) -> Result<u8, String> {
        match addr {
            0x0002 => {
                let ret = self.ppustatus;
                self.clear_vblank();
                // clear address latch for PPUSCROLL and PPUADDR
                Ok(ret)
            },
            0x0004 => {
                Ok(self.oamdata[self.oamaddr as usize])
            },
            0x0007 => {
                Ok(self.ppudata)
            },
            _ => Err(format!("PPU: attempt to get addr @{:04X} failed (not owned)",addr))
        }
    }
    fn set(&mut self, addr: u16, val: u8) -> Result<(), String> {
        match addr {
            0x0000 => {
                self.ppuctrl = val;
                return self.check_ctrl()
            }
            0x0001 => {
                self.ppumask = val;
                return self.check_mask()
            }
            0x0003 => {
                self.oamaddr = val;
                Ok(())
            }
            0x0004 => {
                self.oamdata[self.oamaddr as usize] = val;
                self.oamaddr = (((self.oamaddr as u16) + 1) % 256) as u8;
                Ok(())
            }
            0x0005 => {
                if !self.ppuscroll_byte2 {
                    self.ppuscroll_byte2 = true;
                    self.ppuscroll = val;
                } else {

                }
                Ok(())
            }
            0x0006 => {
                if !self.ppuaddr_byte2 {
                    self.ppuaddr_byte2 = true;
                    self.ppuaddr = val;
                } else {
                    self.vmemaddr = ((self.ppuaddr as u16)<<8) + (val as u16);
                    self.ppuaddr_byte2 = false;
                }
                Ok(())
            },
            0x0007 => {
                let ret = self.mmu.borrow_mut().set(self.vmemaddr,val);
                if self.ctrl_flag(PPUCtrlFlag::INCREMENT_MODE) {
                    self.vmemaddr += 32;
                    self.vmemaddr %= 0x2000;
                } else {
                    self.vmemaddr += 1;
                    self.vmemaddr %= 0x2000;
                }
                return ret
            },
            0x2014 => {
                let mut bus_ref = self.bus.borrow_mut();
                for i in 0..=255 {
                    self.oamdata[i] = bus_ref.get(((val as u16) << 8) + (i as u16)).unwrap();
                };
                Ok(())
            }
            _ => {
                return Err(format!("PPU: attempt to set addr @{:04X}={:02X} failed (not owned)",addr,val));
            }
        }
    }
    fn len(&self) -> usize {
        9 // External access is limited to 0x2000..0x2007 or 0x4014
    }
}