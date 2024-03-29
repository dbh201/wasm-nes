

use super::AddressNode::AddressObject;
use super::super::console_log;

pub struct RamBank {
    data: Vec<u8>
}


impl RamBank {
    pub fn new(max_addr: u16) -> RamBank {
        // Minimum 1 byte (this allows RamBanks with 65536 bytes)
        let mut data = Vec::with_capacity(max_addr as usize + 1);
        for _ in 0..=max_addr {
            data.push(0);
        };
         RamBank { data }
    }
    pub fn bulk_set(&mut self, addr: u16, data: Vec<u8>) -> Result<(), String> {
        console_log!("Copying {} bytes to {:04X}",data.len(),addr);
        self.data[addr as usize..(addr as usize)+data.len()].copy_from_slice(data.as_slice());
        Ok(())
    }
}
impl AddressObject for RamBank {
    fn get(&mut self, addr: u16) -> Result<u8,String> {
        if addr as usize > self.data.len() {
            Err(format!("get addr @{:X} > max addr {:X}", addr, self.data.len()))
        } else {
            //console_log!("[get {:04X}]:{:02X}", addr, self.data[addr as usize]);
            Ok(self.data[addr as usize])
        }
    }

    fn set(&mut self, addr: u16, val: u8) -> Result<(),String> {
        if addr as usize > self.data.len() {
            Err(format!("set addr @{:X}={:02X} > max addr {:X}", addr, val, self.data.len()))
        } else {
            self.data[addr as usize] = val;
            //console_log!("[set {:04X}]:{:02X}", addr, self.data[addr as usize]);
            Ok(())
        }
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}