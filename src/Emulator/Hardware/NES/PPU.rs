#![allow(non_camel_case_types)]
#![allow(dead_code)]
use super::super::super::Common::{AddressBus::AddressBus, AddressNode::AddressObject};
use crate::dummy_console_log as console_log;
use std::{cell::RefCell, rc::Rc};

#[repr(u8)]
pub enum PPUCtrlFlag {
    NAMETABLE_BIT0  = 0x01,
    NAMETABLE_BIT1  = 0x02,
    INCREMENT_MODE  = 0x04,
    SPR_TABLE_SEL   = 0x08,
    BG_TABLE_SEL    = 0x10,
    SPR_HEIGHT_8x16 = 0x20,
    EXT_READ_WRITE  = 0x40,
    VBLANK          = 0x80
}
#[repr(u8)]
pub enum PPUMaskFlag {
    GREYSCALE       = 0x01,
    BG_LEFT_COLUMN  = 0x02,
    SPR_LEFT_COLUMN = 0x04,
    BG_ENABLE       = 0x08,
    SPR_ENABLE      = 0x10,
    RED_EMPHASIS    = 0x20,
    GREEN_EMPHASIS  = 0x40,
    BLUE_EMPHASIS   = 0x80
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
    ppuaddr: u8,
    vmemaddr: u16,
    ppudata: u8,
    oamdma: u8,

    pub oamdata:    [u8;256],
    pub _oambuf:    u8,
    pub regsproam:  [u8;32], //sprites for current scanline
    pub regsprp:    [[u8;2];8],    //sprite pattern table data
    pub regspra:    [u8;8],    //sprite attribute latches
    pub regsprx:    [u8;8],    //sprite x position



    ppuscroll_x: u8,
    ppuscroll_y: u8,

    // write latches
    // TODO: are these the same latch in the real machine?
    ppuaddr_byte2: bool,
    ppuscroll_y_write: bool,

    //tile registers
    pub regtilel: u16,
    pub regtileh: u16,

    //palette registers
    pub regpala: u8,
    pub regpalb: u8,

    vram: u16,
    tvram: u16,
    finex: u8,
    pub clock: usize,

    pub iter1: usize, // generic iterators for storing indices between clock cycles
    pub iter2: usize,
    pub iter3: usize,

    pub ntsc: bool,
    pub odd_frame: bool,

    pub ppubus: Rc<RefCell<AddressBus<'a>>>,
    mainbus: Rc<RefCell<AddressBus<'a>>>,
    //For rendering cycles, store the processed pixel here
    pub pixel: Option<u8>,
    pub scanline: usize,
    pub scanline_cycle: usize,
    //TODO: This should probably be stored on the Renderer side
    pub framebuffer: Rc<RefCell<Vec<u8>>>, 
}

impl<'a> PPU<'_> {
    pub fn sprite_height(&self) -> u8 {
        return 8;
    }
    pub fn get_sprite_data(&self, sprite: usize, plane: usize, scanline: usize) -> Result<u8, String> {
        // sprite is the index into the regsproam data structure (sprite 0-7)
        // offset is which byte within the regsprp to return (byte 0-1)
        // returns one byte of pattern data per call, to sync with clock pulses
        // PPUcontrol will decide which half of the pattern table to use
        // regsproam contains sprite info in OAM format
        let tile_index = self.regsproam[sprite + 1];
        let y_offset = (self.regsproam[sprite] as usize - scanline) as u16;
        let mut addr: u16;
        if self.ppuctrl & PPUCtrlFlag::SPR_HEIGHT_8x16 as u8 != 0 {
            // 0x1000 or 0x0000
            addr = ((self.ppuctrl & PPUCtrlFlag::SPR_TABLE_SEL as u8) as u16) << 9;
            addr += (tile_index as u16) << 4;
        } else {
            addr = ((tile_index & 0x01) as u16) << 12;
            addr += ((tile_index & 0xFE) as u16) << 4;
        }
        return self.ppubus.borrow_mut().get(addr + y_offset + ((plane as u16)<< 3));     
    }
    pub fn new(
        mainbus: Rc<RefCell<AddressBus<'a>>>, 
        ppubus: Rc<RefCell<AddressBus<'a>>>,
        framebuffer: Rc<RefCell<Vec<u8>>>
    ) -> Result<PPU<'a>,String> {
        console_log!("Init PPU");
        let ret = PPU {
            ppuctrl:    0,
            ppumask:    0,
            ppustatus:  0,

            ppuscroll_x:  0,
            ppuscroll_y:  0,
            ppuaddr:    0,
            vmemaddr:   0,
            ppudata:    0,

            oamaddr:    0,
            oamdma:     0,
            _oambuf:    0,
            

            regtilel:   0,
            regtileh:   0,
            regpala:    0,
            regpalb:    0,

            regsproam:  [0;32],
            regsprp:    [[0;2];8],
            regspra:    [0;8],
            regsprx:    [0;8],

            vram:       0,
            tvram:      0,
            finex:      0,
            clock:      0,
            ntsc:       true,
            odd_frame:  false,

            iter1:      0,
            iter2:      0,
            iter3:      0,

            oamdata:    [0;256], 
            ppuaddr_byte2: false,
            ppuscroll_y_write: false,
            ppubus,
            mainbus,
            framebuffer,
            pixel: None,
            scanline: 0,
            scanline_cycle: 0
        };
        return Ok(ret)
    }
    pub fn ctrl_flag(&self, f: PPUCtrlFlag) -> bool {
        console_log!("TESTING CTRL FLAG...");
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
    pub fn is_vblank(&self) -> bool {
        self.ppustatus & (PPUStatusFlag::VBLANK_STARTED as u8) != 0
    }
    pub fn set_vblank(&mut self) {
        self.ppustatus |= PPUStatusFlag::VBLANK_STARTED as u8;
    }
    fn clear_latches(&mut self) {
        self.ppustatus &= !(PPUStatusFlag::VBLANK_STARTED as u8);
        self.ppuscroll_y_write = false;
    }
    pub fn get_scanline(&self) -> usize {
        self.clock / 341
    }
    pub fn get_scanline_cycle(&self) -> usize {
        self.clock % 341
    }
    pub fn fetch_data(&mut self) -> Result<(), String>{
        let scanline = self.get_scanline();
        let scanline_cycle = self.get_scanline_cycle();
        if self.ntsc {
            // Sprite loading
            if scanline < 240 {
                // visible scanline
                if scanline_cycle == 0 {
                    // Do nothing
                    return Ok(())
                } else if scanline_cycle < 65 {
                    if scanline_cycle % 2 == 1 {
                        self.regsproam[scanline_cycle>>1] = 0xFF;
                    }
                } else if scanline_cycle == 65 {
                    //console_log!("SCANLINE INIT");
                    self.iter1 = 0;
                    self.iter2 = 0;
                    self.iter3 = 0;
                } else if scanline_cycle < 257 {
                    /*  console_log!("cycle: {}\niter1: {} iter2: {} iter3: {} ",
                        scanline_cycle,
                        self.iter1,
                        self.iter2,
                        self.iter3
                    );  */
                    if self.iter3>>2 < 8 && self.iter1 < 256 {
                        if scanline_cycle % 2 == 1 {
                            self._oambuf = self.oamdata[self.iter1 + self.iter2];
                        } else {
                            // check next scanline
                            if  self._oambuf >= scanline as u8 + 1 &&
                                self._oambuf <= scanline as u8 + 1 + self.sprite_height() {
                                    //console_log!("Active sprite");
                                // sprite is active
                                    if self.iter2 >= 4 {
                                        self.iter1 += 4;
                                        self.iter2 = 0;
                                        self.iter3 += 4;
                                    } else {
                                        self.regsproam[self.iter3 + self.iter2] = self._oambuf;
                                        self.iter2 += 1;
                                    }
                            } else {
                                self.iter1 += 4;
                                self.iter2 = 0;
                            }
                        }
                    }
                } else if scanline_cycle == 257 {
                    //console_log!("SCANLINE END");
                } else if scanline_cycle < 321 {
                    let sprite = (scanline_cycle - 257)>>3;
                    // starts at cycle 257, 8 cycles per sprite

                    // since we store these in the PPU struct, no
                    // need to actually move data
                    match scanline_cycle % 8 {
                        1 => {
                            // read Y coord; this is done in get_sprite_data
                        },
                        2 => {
                            // read tile number; this is done in get_sprite_data
                        },
                        3 => {
                            // read attributes
                            self.regspra[sprite] = self.regsproam[sprite*4 + 2]
                        },
                        4 => {
                            // read X coord
                            self.regsprx[sprite] = self.regsproam[sprite*4 + 3]
                        },
                        5..=7 | 0 => {
                            let plane = ((scanline_cycle - 1) >> 1) % 2;
                            // read 2 bytes of tile data
                            self.regsprp[sprite][plane] = self.get_sprite_data(sprite, plane, scanline)?;
                            //console_log!("Sprite: {} Plane: {} = {:02X}",sprite,plane, self.regsprp[sprite][plane]);
                        },
                        _ => {
                            return Err("A modulo 8 evaluated to 8 or more? This is surely the devil's work".to_owned())
                        }
                    }
                    // Fetch sprites from pattern table
                } else /*if scanline_cycle < 341*/ {
                    // busywait by reading the first byte in secondary OAM
                    // We don't need to waste CPU by reading data, so just return Ok
                    return Ok(())
                }
                return Ok(())
            } else /* if scanline >= 240 */ {

                return Ok(())
            }
        } else {
            return Err("PAL not yet implemented".to_owned())
        }
    }
    pub fn process_sprites(&mut self) -> Result<(), String> {
        if self.scanline >= 240 {
            return Ok(())
        }
        if self.scanline_cycle < 257 {
            // Pixel rendering
            // write pixel to buffer here
            //console_log!("Entering pixel rendering code...");
            
            let mut spr_pixel: Option<u8> = None;
            let mut spr_attrs = 0u8;
            for i in 0..8 {
                if self.regsprx[i] > 0 {
                    self.regsprx[i] -= 1;
                }
                if self.regsprx[i] == 0 {
                    //console_log!("Found sprite {} with x=0",i);
                    if spr_pixel.is_none() {
                        let pixel =((self.regsprp[i][0]&0x80)>>7) + 
                            ((self.regsprp[i][1]&0x80)>>6);
                        if pixel != 0 {
                            spr_attrs = self.regspra[i];
                            spr_pixel = Some(pixel + ((spr_attrs & 0x03)<<2));
                        }
                    }
                    // shift out bits from this sprite
                    self.regsprp[i][0]<<=1;
                    self.regsprp[i][1]<<=1;
                }
            }
            // BG pixel exists and is not transparent
            if self.pixel.is_some() {
                if spr_pixel.is_some() && (spr_attrs & 0x20) == 0 {
                    self.pixel = spr_pixel;
                }
            } else if spr_pixel.is_some() {
                self.pixel = spr_pixel;
            }
            //console_log!("Writing pixel s:{},p:{} with {}",scanline,scanline_cycle,color);
        } 
        return Ok(())
    }
    pub fn process_bg_tile(&mut self) -> Result<(), String> {
        if self.scanline < 240 {
            if self.scanline_cycle < 257 {
                let mut bus = self.ppubus.borrow_mut();
                let pixel = (((self.regtilel&0x80)>>7) + ((self.regtileh&0x80)>>6)) as u8;
                if pixel != 0 {
                    self.pixel = Some(pixel);
                }
                self.finex += 1;
                if self.finex >= 0x08 {
                    self.finex %= 0x08;
                    if self.vram & 0x001F == 0x001F {
                        self.vram &= !(0x001F);
                        self.vram ^= 0x0400;
                    } else {
                        self.vram += 1;
                    }
                    let tile_address = (bus.get(0x2000 | (self.vram & 0x0FFF))? as u16)<<8;
                    let attr_address = (bus.get(0x2000 | (self.vram & 0x0FFF))? as u16)<<8;
                    let tile_pattern_addr = (bus.get(tile_address)? as u16)<<3;
                    self.regtilel |= bus.get(tile_pattern_addr)? as u16;
                    self.regtileh |= bus.get(tile_pattern_addr+8)? as u16;
                    self.regpala = bus.get(attr_address)?;
                    self.regpalb = bus.get(attr_address+1)?;
                }

            }
            if self.scanline_cycle == 256 {
                // Y position increment @ last pixel
                self.vram += 0b1000000000000;
                if self.vram & 0x8000 != 0 {
                    let mut cy = (self.vram & 0x03E0) >> 5;
                    self.vram &= !0x83E0;
                    if self.vram & 0x0800 == 0 && cy == 0x1D {
                        cy = 0;
                        self.vram ^= 0x0800;
                    } else if cy < 0x1F {
                        cy += 1;
                    } 
                    self.vram |= cy << 5;
                }
            }
        }
        else if self.scanline == 261 && self.scanline_cycle == 340 {
            // Retrieve new frame tiles
            self.tvram = ((self.ppuctrl as u16 & 0x03)<<10) | 
                (self.ppuscroll_x as u16 & 0xF8)        | 
                ((self.ppuscroll_y as u16 & 0x03)<<12)  |
                ((self.ppuscroll_y as u16 & 0x1C)<<5)   |
                ((self.ppuscroll_y as u16 & 0xC0)<<8);
            self.vram = self.tvram;
        }
    
        return Ok(())
    }
    pub fn clock_tick(&mut self) -> Result<(), String> {
        if self.scanline_cycle != 0 {
            self.fetch_data()?;
            self.process_bg_tile()?;
            self.process_sprites()?;

            // Use BG color if no tile or sprite
            if self.pixel.is_none() {
                self.pixel = Some(self.ppubus.borrow_mut().get(0x3F00)?);
            }
            if self.ntsc {

                if self.scanline == 241 && self.scanline_cycle == 1 {
                    self.set_vblank();
                }
            }
            self._clock_tick()?;
        }
        self.increment_clock();
        Ok(())
    }
    pub fn clock_modulo(&self) -> usize {
        if self.ntsc {
            89341 + 89342
        } else {
            106392
        }
    }
    pub fn increment_clock(&mut self) {
        self.clock += 1;
        self.clock %= self.clock_modulo();
        self.scanline = self.get_scanline();
        self.scanline_cycle = self.get_scanline_cycle();
    }
    fn increment_vmemaddr(&mut self) {
        if self.ctrl_flag(PPUCtrlFlag::INCREMENT_MODE) {
            self.vmemaddr += 32;
            self.vmemaddr %= 0x4000;
        } else {
            self.vmemaddr += 1;
            self.vmemaddr %= 0x4000;
        }
    }
}
pub trait PPU_Renderer {
    fn _clock_tick(&mut self) -> Result<(), String>;
}
// Addresses are relative to addr start (0x2000)
impl AddressObject for PPU<'_> {
    fn get(&mut self, addr: u16) -> Result<u8, String> {
        match addr {
            0x0002 => {
                let ret = self.ppustatus;
                self.clear_latches();
                // clear address latch for PPUSCROLL and PPUADDR
                Ok(ret)
            },
            0x0004 => {
                Ok(self.oamdata[self.oamaddr as usize])
            },
            0x0007 => {
                let ret: u8;
                if self.vmemaddr < 0x3F00 {
                    ret = self.ppudata;
                    self.ppudata = self.ppubus.borrow_mut().get(self.vmemaddr)?;
                } else {
                    ret = self.ppubus.borrow_mut().get(self.vmemaddr)?;
                    self.ppudata = self.ppubus.borrow_mut().get(self.vmemaddr - 0x1000)?;
                }
                self.increment_vmemaddr();
                return Ok(ret)
            },
            _ => Err(format!("PPU: attempt to get mmio @{:04X} failed (not owned)",addr))
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
                if !self.ppuscroll_y_write {
                    self.ppuscroll_y_write = true;
                    self.ppuscroll_x = val;
                } else {
                    self.ppuscroll_y = val;
                }
                Ok(())
            }
            0x0006 => {
                if !self.ppuaddr_byte2 {
                    self.ppuaddr_byte2 = true;
                    self.ppuaddr = val;
                } else {
                    self.vmemaddr = (((self.ppuaddr as u16)<<8) + (val as u16))%0x4000;
                    self.ppuaddr_byte2 = false;
                }
                Ok(())
            },
            0x0007 => {
                console_log!("PPU: Wrote {:02X} to {:04X}",val, self.vmemaddr);
                let ret = self.ppubus.borrow_mut().set(self.vmemaddr,val);
                self.increment_vmemaddr();
                return ret
            },
            0x2014 => {
                console_log!("OAM bulk copy");
                let mut bus_ref = self.mainbus.borrow_mut();
                for i in 0..=255 {
                    self.oamdata[i] = bus_ref.get(((val as u16) << 8) + (i as u16)).unwrap();
                };
                Ok(())
            }
            _ => {
                return Err(format!("PPU: attempt to set mmio @{:04X}={:02X} failed (not owned)",addr,val));
            }
        }
    }
    fn len(&self) -> usize {
        9 // External access is limited to 0x2000..0x2007 or 0x4014
    }
}