//TODO: This needs to be a ROM + mappers and address spaces
// mappers should Have their own MmioObjects...
// should they be managed by another bus? 

use super::{AddressBus::AddressBus, MmioNode::{MmioObject, MmioNode}};
use super::console_log;

pub struct Cartridge<'a> {
    pub data: AddressBus<'a>
}
impl<'a> Cartridge<'a> {
    pub fn new(name: String) -> Result<Cartridge<'a>,String> {
        console_log!("Init Cartridge");
        let mut data = AddressBus::new(name)?;
        Ok(Cartridge { data })
    }
    pub fn name(&self) -> String {
        self.data.name.to_string()
    }
    pub fn clock_tick(&mut self) -> Result<(), String> {
        Ok(())
    }
    pub fn load_mapper(&mut self, data: &Vec<u8>, mapper: u16) {
        if mapper == 0 {
            let mut prgram = MmioNode::new("PRG ROM".to_owned());
            prgram.add_addr_range_mirrored(0x6000, 0x7FFF, 0);
            prgram.make_ram(0x1FFF);
            self.data.register_MmioNode(prgram);
            let mut prgrom = MmioNode::new("PRG ROM".to_owned());
            if data.len() <= 16*1024 {
                prgrom.add_addr_range_mirrored(0x8000,0xFFFF,16*1024);
                for i in 0x8000..0xBFFF {
                    prgrom.set(i,data[ (i-0x8000) as usize ]);
                }
            } else {
                prgrom.add_addr_range(0x8000, 0xFFFF);
                for i in 0x8000..0xFFFF {
                    prgrom.set(i,data[ (i-0x8000) as usize ]);
                }
            }
            self.data.register_MmioNode(prgrom);
        }
    }
}
impl<'a> MmioObject for Cartridge<'a> {
    fn get(&mut self, addr: u16) -> Result<u8,String> {
        if addr < 0x4020 {
            return Err(format!("{} get @{:04X}: Cartridge address space begins at 0x4020",self.data.name, addr).to_string())
        }
        self.data.get(addr)
    }

    fn set(&mut self, addr: u16, val: u8) -> Result<(),String> {
        if addr < 0x4020 {
            return Err(format!("{} set @{:04X}={:02X}: Cartridge address space begins at 0x4020",self.data.name, addr, val).to_string())
        }
        self.data.set(addr, val)
    }
    fn len(&self) -> usize {
        0xBFE0
    }
}