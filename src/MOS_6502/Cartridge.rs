//TODO: This needs to be a ROM + mappers and address spaces
// mappers should Have their own MmioObjects...
// should they be managed by another bus? 

use super::{AddressBus::AddressBus, MmioNode::MmioObject};

pub struct Cartridge<'a> {
    pub data: AddressBus<'a>
}
impl<'a> Cartridge<'a> {
    pub fn new(name: String) -> Result<Cartridge<'a>,String> {
        let mut data = AddressBus::new(name)?;
        Ok(Cartridge { data })
    }
    pub fn name(&self) -> String {
        self.data.name.to_string()
    }
    pub fn clock_tick(&mut self) -> Result<(), String> {
        Ok(())
    }
}
impl<'a> MmioObject for Cartridge<'a> {
    fn get(&mut self, addr: u16) -> Result<u8,String> {
        if addr > 0xBFDF {
            return Err(format!("{} get @{:04X}: Cartridge address space ends at 0xBFDF",self.data.name, addr).to_string())
        }
        self.data.get(addr)
    }

    fn set(&mut self, addr: u16, val: u8) -> Result<(),String> {
        if addr > 0xBFDF {
            return Err(format!("{} set @{:04X}={:02X}: Cartridge address space ends at 0xBFDF",self.data.name, addr, val).to_string())
        }
        self.data.set(addr, val)
    }
    fn len(&self) -> usize {
        0xBFE0
    }
}