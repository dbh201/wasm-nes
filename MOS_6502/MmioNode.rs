use crate::Mainbus::Mainbus;
use crate::RamBank::RamBank;
use crate::PPU::PPU;
use crate::APUJoystick::APUJoystick;
use crate::Cartridge::Cartridge;
use std::cell::RefMut;
#[derive(PartialEq)]
enum MmioType {
    UNSET,
    RAM,
    PPU,
    APU,
    CART
}
pub trait MmioObject {
    fn get(&self, addr: u16) -> Result<u8,String>;
    fn set(&mut self, addr: u16, val: u8) -> Result<(),String>;
    fn len(&self) -> usize;
}
pub struct MmioNode<'a> {
    pub(crate) name: String,
    pub(crate) ownership: Vec<[u16;3]>,
    ram: Option<RamBank>,
    ppu: Option<PPU<'a>>,
    apu: Option<APUJoystick>,
    cart: Option<Cartridge>,
    obj_type: MmioType,

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
        };
        res
    }
    pub fn make_ram(&mut self, len: u16) -> Result<(),String> {
        if self.obj_type == MmioType::UNSET {
            self.ram = Some(RamBank::new(len));
            self.obj_type = MmioType::RAM;
            Ok(())
        } else {
            Err(format!("MmioNode {}: cannot be made RAM because it already has a type", self.name))
        }
    }
    pub fn make_ppu(&mut self, bus: RefMut<'a, Mainbus<'a>>, ppu_mmu: RefMut<'a, Mainbus<'a>>) -> Result<(),String> {
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
    pub fn insert_cart(&mut self, cart: Cartridge) -> Result<(),String> {
        if self.obj_type == MmioType::UNSET {      
            self.cart = Some(cart);
            self.obj_type = MmioType::CART;
            Ok(())
        } else {
            Err(format!("MmioNode {}: cannot be made CART because it already has a type", self.name))
        }        
    }
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
        self.add_mirror_addr_range(first_addr, last_addr, 0)
    }
    pub fn add_mirror_addr_range(&mut self, first_addr: u16, last_addr: u16, strobe: u16) -> Result<(),String> {
        if self.resolve_addr(first_addr) != None || self.resolve_addr(last_addr) != None {
            return Err(format!("MmioNode {}: Could not add ownership @{:04X}-{:04X} (overlaps existing ownership)",
                self.name, first_addr, last_addr));
        }
        self.ownership.push([first_addr,last_addr,strobe]);
        Ok(())
    }
    pub fn get_addr_range(&self, addr: u16) -> Option<&[u16;3]> {
        //println!("Scanning for {:04X}...", addr);
        for i in self.ownership.iter() {
            //print!("Test: {:04X} [{:04X}..{:04X}%{:04X}]", addr,i[0],i[1],i[2]);
            if addr >= i[0] && addr <= i[1] {
                //println!("  OK");
                return Some(i);
            }
            println!("...");
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
        } else {
            final_addr = addr - i[0];
        }
        return Some(final_addr);
    }

    pub fn get(&self, addr: u16) -> Result<u8, String> {
        match self.obj_type {
            MmioType::UNSET => Err(format!("MmioNode {}: get attempt @{:04X} but backing store uninitialized", self.name, addr)),
            MmioType::RAM => {
                let val = self.ram.as_ref().unwrap().get(addr);
                //let resp = val.clone().ok();
                //println!("MmioNode {}: get {:04X} returned {}",self.name,addr,resp.unwrap());
                return val
            },
            _ => Err(format!("MmioNode {}: get attempt @{:04X} but type not yet implemented", self.name, addr))
        }
    }

    pub fn set(&mut self, addr: u16, val: u8) -> Result<(), String> {
        match self.obj_type {
            MmioType::UNSET => Err(format!("MmioNode {}: get attempt @{:04X} but backing store uninitialized", self.name, addr)),
            MmioType::RAM => {
                return self.ram.as_mut().unwrap().set(addr, val)
            },
            _ => Err(format!("MmioNode {}: set attempt @{:04X}={:02X} but type not yet implemented", self.name, addr, val))
        }
    }
}
