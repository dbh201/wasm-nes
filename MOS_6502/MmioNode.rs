use crate::RamBank::RamBank;
#[derive(PartialEq)]
enum MmioType {
    UNSET,
    RAM,
}
pub trait MmioObject {
    fn get(&self, addr: u16) -> Result<u8,String>;
    fn set(&mut self, addr: u16, val: u8) -> Result<(),String>;
}
pub struct MmioNode {
    pub(crate) name: String,
    pub(crate) addr: u16,
    pub(crate) len: u16,
    ram: Option<RamBank>,
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
            ram: None,
            obj_type: MmioType::UNSET
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
    pub fn owns_addr(&self, addr: u16) -> bool {
        addr >= self.addr && addr <= self.addr + self.len
    }

    pub fn get(&self, addr: u16) -> Result<u8, String> {
        match self.obj_type {
            MmioType::UNSET => Err(format!("MmioNode: {} backing store uninitialized; owns {}-{} [get {}]",self.name,self.addr,self.len,addr)),
            MmioType::RAM => return self.ram.as_ref().unwrap().get(addr)
        }
    }

    pub fn set(&mut self, addr: u16, val: u8) -> Result<(), String> {
        match self.obj_type {
            MmioType::UNSET => Err(format!("MmioNode: {} backing store uninitialized; owns {}-{} [get {}]",self.name,self.addr,self.len,addr)),
            MmioType::RAM => return self.ram.as_mut().unwrap().set(addr, val)
        }
    }
}
