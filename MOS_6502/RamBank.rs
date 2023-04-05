use crate::MmioNode::MmioObject;

pub struct RamBank {
    data: Vec<u8>
}


impl RamBank {
    pub fn new(len: u16) -> RamBank {
        // Minimum 1 byte (this allows RamBanks with 65536 bytes)
        let mut data = Vec::with_capacity(len as usize + 1);
        for _ in 0..=len {
            data.push(0);
        };
         RamBank { data }
    }
}
impl MmioObject for RamBank {
    fn get(&self, addr: u16) -> Result<u8,String> {
        if addr as usize > self.data.len() {
            Err(format!("get addr {} out of range", addr))
        } else {
            //println!("[get {:04X}]:{:02X}", addr, self.data[addr as usize]);
            Ok(self.data[addr as usize])
        }
    }

    fn set(&mut self, addr: u16, val: u8) -> Result<(),String> {
        if addr as usize > self.data.len() {
            Err(format!("set addr {} out of range", addr))
        } else {
            self.data[addr as usize] = val;
            //println!("[set {:04X}]:{:02X}", addr, self.data[addr as usize]);
            Ok(())
        }
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}