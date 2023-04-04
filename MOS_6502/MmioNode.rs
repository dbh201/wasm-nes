use crate::RamBank::RamBank;
use crate::PPU::PPU;
use crate::APUJoystick::APUJoystick;
use crate::Cartridge::Cartridge;
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
pub struct MmioNode {
    pub(crate) name: String,
    pub(crate) addr: u16,
    pub(crate) len: u16,
    pub(crate) mirrored: bool, // If MmioNode len is greater than backing store len, mirror it
    ram: Option<RamBank>,
    ppu: Option<PPU>,
    apu: Option<APUJoystick>,
    cart: Option<Cartridge>,
    obj_type: MmioType,

}
impl MmioNode {
    pub fn new(
        name: String,
        addr: u16,
        len: u16
    ) -> MmioNode {
        let res = MmioNode { 
            name, 
            addr, 
            len,
            mirrored: false,
            obj_type: MmioType::UNSET,
            ram: None,
            ppu: None,
            apu: None,
            cart: None,
        };
        res
    }
    pub fn make_ram(&mut self) -> Result<(),String> {
        if self.obj_type == MmioType::UNSET {
            self.ram = Some(RamBank::new(self.len));
            self.obj_type = MmioType::RAM;
            Ok(())
        } else {
            Err(format!("MmioNode: {} cannot be made ram because it already has a type", self.name))
        }
    }
    pub fn enable_mirror(&mut self, mirror: bool) {
        self.mirrored = mirror;
    }
    pub fn change_addr_range(&mut self, addr: u16, len: u16) {
        self.addr = addr;
        self.len = len;
    }
    pub fn owns_addr(&self, addr: u16) -> bool {
        addr >= self.addr && addr <= self.addr + self.len
    }

    pub fn get(&self, addr: u16) -> Result<u8, String> {
        if !self.owns_addr(addr) {
            return Err(format!("MmioNode: {} does not own {} [{}-{}]",self.name,addr,self.len,self.addr));
        }
        match self.obj_type {
            MmioType::UNSET => Err(format!("MmioNode: {} backing store uninitialized; owns {}-{} [get {}]",self.name,self.addr,self.len,addr)),
            MmioType::RAM => {
                let mut final_addr = addr - self.addr;
                if self.mirrored {
                    final_addr %= self.ram.as_ref().unwrap().len() as u16;
                }
                return self.ram.as_ref().unwrap().get(final_addr)
            },
            _ => Err("Type not yet implemented".to_string())
        }
    }

    pub fn set(&mut self, addr: u16, val: u8) -> Result<(), String> {
        if !self.owns_addr(addr) {
            return Err(format!("MmioNode: {} does not own {} [{}-{}]",self.name,addr,self.len,self.addr));
        }
        match self.obj_type {
            MmioType::UNSET => Err(format!("MmioNode: {} backing store uninitialized; owns {}-{} [get {}]",self.name,self.addr,self.len,addr)),
            MmioType::RAM => {
                let mut final_addr = addr - self.addr;
                if self.mirrored {
                    final_addr %= self.ram.as_ref().unwrap().len() as u16;
                }
                return self.ram.as_mut().unwrap().set(final_addr, val)
            },
            _ => Err("Type not yet implemented".to_string())
        }
    }
}
