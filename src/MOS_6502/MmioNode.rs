

use super::AddressBus::AddressBus;
use super::RamBank::RamBank;
use super::PPU::PPU;
use super::APUJoystick::APUJoystick;
use super::Cartridge::Cartridge;
use super::console_log;

use std::fmt;
use std::{cell::RefCell};
use std::{rc::Rc};
#[derive(PartialEq)]
pub enum MmioType {
    UNSET,
    RAM,
    PPU,
    APU,
    CART
}
impl fmt::Display for MmioType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MmioType::UNSET => write!(f, "UNSET"),
            MmioType::RAM => write!(f, "RAM"),
            MmioType::PPU => write!(f, "PPU"),
            MmioType::APU => write!(f, "APU"),
            MmioType::CART => write!(f, "CART"),
        }
    }
}
pub trait MmioObject {
    fn get(&mut self, addr: u16) -> Result<u8,String>; // some gets trigger changes in the Mmio backing store
    fn set(&mut self, addr: u16, val: u8) -> Result<(),String>;
    fn len(&self) -> usize;
}
pub struct MmioNode<'a> {
    pub(super) name: String,
    pub(super) ownership: Vec<[u16;3]>,
    ram: Option<RamBank>,
    ppu: Option<PPU<'a>>,
    apu: Option<APUJoystick>,
    cart: Option<Cartridge<'a>>,
    pub obj_type: MmioType,
    tick_error: usize,

}
impl<'a> MmioNode<'a> {
    pub fn new(
        name: String
    ) -> MmioNode<'a> {
        let res = MmioNode { 
            name,
            ownership: Vec::new(),
            obj_type: MmioType::UNSET,
            ram: None,
            ppu: None,
            apu: None,
            cart: None,
            tick_error: 0,
        };
        res
    }
    pub fn make_ram(&mut self, last_addr: u16) -> Result<(),String> {
        if self.obj_type == MmioType::UNSET {
            self.ram = Some(RamBank::new(last_addr));
            self.obj_type = MmioType::RAM;
            Ok(())
        } else {
            Err(format!("MmioNode {}: cannot be made RAM because it already has a type", self.name))
        }
    }
    pub fn make_ppu(&mut self, bus: Rc<RefCell<AddressBus<'a>>>, ppu_mmu: Rc<RefCell<AddressBus<'a>>>) -> Result<(),String> {
        if self.obj_type == MmioType::UNSET {
            let ppu = PPU::new(bus, ppu_mmu);
            if ppu.is_err() {
                return Err(format!("MmioNode {}: PPU init failed: {}",self.name,ppu.err().unwrap()));
            }
            self.ppu = ppu.ok();
            self.obj_type = MmioType::PPU;
            Ok(())
        } else {
            Err(format!("MmioNode {}: cannot be made PPU because it already has a type", self.name))
        }        
    }
    pub fn make_apu(&mut self) -> Result<(),String> {
        if self.obj_type == MmioType::UNSET {
            let apu = APUJoystick::new();
            if apu.is_err() {
                return Err(format!("MmioNode {}: APU init failed: {}",self.name,apu.err().unwrap()));
            }
            self.apu = apu.ok();
            self.obj_type = MmioType::APU;
            Ok(())
        } else {
            Err(format!("MmioNode {}: cannot be made APU because it already has a type", self.name))
        }        
    }
    pub fn insert_cart(&mut self, cart: Cartridge<'a>) -> Result<(),String> {
        if self.obj_type == MmioType::UNSET {      
            self.cart = Some(cart);
            self.obj_type = MmioType::CART;
            Ok(())
        } else {
            Err(format!("MmioNode {}: cannot be made CART because it already has a type", self.name))
        }        
    }
    // might use this for cartridges later
    #[allow(dead_code)]
    pub fn rem_addr_range(&mut self, addr: u16) -> Result<(),String> {
        for i in 0..self.ownership.len() {
            if self.ownership[i][0] == addr {
                self.ownership.remove(i);
                return Ok(());
            }
        }
        Err(format!("MmioNode {}: Could not remove ownership @{:04X} (does not exist)",self.name,addr))
    }
    pub fn add_addr_range(&mut self, first_addr: u16, last_addr: u16) -> Result<(), String> {
        self.add_addr_range_mirrored(first_addr, last_addr, 0)
    }
    pub fn add_addr_range_mirrored(&mut self, first_addr: u16, last_addr: u16, strobe: u16) -> Result<(),String> {
        if self.resolve_addr(first_addr) != None || self.resolve_addr(last_addr) != None {
            return Err(format!("MmioNode {}: Could not add ownership @{:04X}-{:04X} (overlaps existing ownership)",
                self.name, first_addr, last_addr));
        }
        self.ownership.push([first_addr,last_addr,strobe]);
        Ok(())
    }
    pub fn get_addr_range(&self, addr: u16) -> Option<&[u16;3]> {
        console_log!("Scanning for {:04X} in {}...", addr, self.name);
        for i in self.ownership.iter() {
            if addr >= i[0] && addr <= i[1] {
                console_log!("Test: {:04X} [{:04X}..{:04X}%{:04X}]  OK", addr,i[0],i[1],i[2]);
                return Some(i);
            }
            console_log!("Test: {:04X} [{:04X}..{:04X}%{:04X}] ...", addr,i[0],i[1],i[2]);
        }
        None
    }
    pub fn resolve_addr(&self, addr: u16) -> Option<u16> {
        let i = self.get_addr_range(addr);
        if i.is_none() {
            return None;
        }
        let i = i.unwrap();
        let final_addr: u16;
        if i[2] != 0 {
            final_addr = (addr - i[0]) % i[2];
            console_log!("Mirror: {:04X} = ({:04X} - {:04X} % {:04X}",final_addr, addr, i[0], i[2]);
        } else {
            final_addr = addr - i[0];
        }
        return Some(final_addr);
    }

    pub fn get(&mut self, addr: u16) -> Result<u8, String> {
        match self.obj_type {
            MmioType::UNSET => Err(format!("MmioNode {}: get attempt @{:04X} but backing store uninitialized", self.name, addr)),
            MmioType::RAM => {
                let val = self.ram.as_mut().unwrap().get(addr)?;
                console_log!("MmioNode {}: get {:04X} returned {}", self.name, addr, val);
                return Ok(val)
            },
            MmioType::CART => self.cart.as_mut().unwrap().get(addr),
            _ => Err(format!("MmioNode {}: get attempt @{:04X} but type not yet implemented", self.name, addr))
        }
    }

    pub fn set(&mut self, addr: u16, val: u8) -> Result<(), String> {
        match self.obj_type {
            MmioType::UNSET => Err(format!("MmioNode {}: get attempt @{:04X} but backing store uninitialized", self.name, addr)),
            MmioType::RAM => {
                return self.ram.as_mut().unwrap().set(addr, val)
            },
            MmioType::CART => {
                return self.cart.as_mut().unwrap().set(addr, val)
            }
            _ => Err(format!("MmioNode {}: set attempt @{:04X}={:02X} but type not yet implemented", self.name, addr, val))
        }
    }
    pub fn _clock_tick(&mut self) -> Result<(), String> {
        match self.obj_type {
            MmioType::UNSET => Err(format!("MmioNode {}: Tried ticking but backing store uninitialized", self.name)),
            MmioType::RAM => {
                return Ok(())
            },
            MmioType::PPU => {
                return self.ppu.as_mut().unwrap().clock_tick()
            },
            MmioType::CART => {
                return self.cart.as_mut().unwrap().clock_tick()
            }
            _ => {
                self.tick_error += 1;
                if self.tick_error % 1024 == 1 {
                    return Err(format!("MmioNode {}: Tried ticking but type not yet implemented ({} times)", self.name,self.tick_error))
                } else {
                    return Ok(())
                }
            }
        }        
    }
}
