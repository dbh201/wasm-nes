use crate::MmioNode::MmioObject;

pub struct RamBank {
    data: Vec<u8>
}


impl RamBank {
    pub fn new(len: u16) -> RamBank {
        let mut data = Vec::with_capacity(len.into());
        // Add an extra byte so we can get a full 64k if we want it
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
            Ok(self.data[addr as usize])
        }
    }

    fn set(&mut self, addr: u16, val: u8) -> Result<(),String> {
        if addr as usize > self.data.len() {
            Err(format!("set addr {} out of range", addr))
        } else {
            self.data[addr as usize] = val;
            Ok(())
        }
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}