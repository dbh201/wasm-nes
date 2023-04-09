//TODO: This needs to be a ROM + mappers and address spaces
// mappers should Have their own MmioObjects...
// should they be managed by another bus? 

use std::{rc::Rc, cell::RefCell};

use super::{AddressBus::AddressBus, MmioNode::MmioNode};
use super::console_log;

pub struct Cartridge<'a> {
    name: String,
    mainbus: Rc<RefCell<AddressBus<'a>>>,
    mainNodes: Vec<MmioNode<'a>>,
    ppu_bus: Rc<RefCell<AddressBus<'a>>>,
    ppuNodes: Vec<MmioNode<'a>>,
}

#[allow(dead_code)]
struct NESHeader {
    pub prg_rom_size: u8,
    pub chr_rom_size: u8,
    flags_6: u8,
    flags_7: u8,
    flags_8: u8,
    flags_9: u8,
    flags_10: u8,
    padding_11: u8,
    padding_12: u8,
    padding_13: u8,
    padding_14: u8,
    padding_15: u8,
}

impl NESHeader {
    pub fn new(data: &Vec<u8>) -> Result<NESHeader,String> {
        let magicnum = [ 0x4E, 0x45, 0x53, 0x1A ];
        for i in 0..4 {
            if data[i] != magicnum[i] {
                return Err(format!("MAGICNUM failure byte {}: {:02X} != {:02X}",i,data[i],magicnum[i]))
            }
        }
        Ok(NESHeader {
            prg_rom_size: data[4],
            chr_rom_size: data[5],
            flags_6: data[6],
            flags_7: data[7],
            flags_8: data[8],
            flags_9: data[9],
            flags_10: data[10],
            padding_11: data[11],
            padding_12: data[12],
            padding_13: data[13],
            padding_14: data[14],
            padding_15: data[15],
        })
    }
    pub fn prgsize(&self) -> usize {
        return (self.prg_rom_size as usize)<<14
    }
    pub fn chrsize(&self) -> usize {
        return (self.chr_rom_size as usize)<<13
    }
    pub fn mapper(&self) -> u8 {
        return (self.flags_6 >> 4) + (self.flags_7 & 0xF0)
    }
    pub fn prg_rom_start(&self) -> usize {
        return 0x10;
    }
    pub fn prg_rom_end(&self) -> usize {
        return self.prg_rom_start() + self.prgsize() - 1;
    }
    pub fn chr_rom_start(&self) -> usize {
        return self.prg_rom_end() + 1;
    }
    pub fn chr_rom_end(&self) -> usize {
        return self.prg_rom_end() + self.chrsize();
    }
}
impl<'a> Cartridge<'a> {
    pub fn new(name: String, mainbus: Rc<RefCell<AddressBus<'a>>>, ppubus: Rc<RefCell<AddressBus<'a>>>) -> Result<Cartridge<'a>,String> {
        console_log!("Init Cartridge");
        Ok(Cartridge { name, mainbus, mainNodes: Vec::new(), ppu_bus: ppubus, ppuNodes: Vec::new() })
    }
    pub fn name(&self) -> String {
        self.name.to_string()
    }
    pub fn clock_tick(&mut self) -> Result<(), String> {
        Ok(())
    }
    pub fn register(&mut self) -> Result<(), String> {
        while self.mainNodes.len() > 0 {
            self.mainbus.borrow_mut().register_MmioNode(self.mainNodes.pop().unwrap())?;
        }
        while self.ppuNodes.len() > 0 {
            self.ppu_bus.borrow_mut().register_MmioNode(self.ppuNodes.pop().unwrap())?;
        }
        Ok(())
    }
    pub fn unregister(&mut self) -> Result<(), String> {
        let mut node = self.mainbus.borrow_mut().unregister_MmioNode_match(("[".to_string() + &self.name + "]").to_string());
        while node.is_some() {
            self.mainNodes.push(node.unwrap());
            node = self.mainbus.borrow_mut().unregister_MmioNode_match(("[".to_string() + &self.name + "]").to_string());
        }
        node = self.ppu_bus.borrow_mut().unregister_MmioNode_match(("[".to_string() + &self.name + "]").to_string());        
        while node.is_some() {
            self.ppuNodes.push(node.unwrap());
            node = self.ppu_bus.borrow_mut().unregister_MmioNode_match(("[".to_string() + &self.name + "]").to_string());
        }
        Ok(())
    }
    pub fn load_nes_file(&mut self, data: &Vec<u8>) -> Result<(), String> {
        let header = NESHeader::new(data)?;
        if header.mapper() == 0 {
            let mut prgram = MmioNode::new(format!("[{}].PRGRAM",self.name).to_owned());
            prgram.make_ram(0x1FFF)?;
            prgram.add_addr_range_mirrored(0x6000, 0x7FFF, 0)?;
            self.mainbus.borrow_mut().register_MmioNode(prgram)?;

            let mut prgrom = MmioNode::new(format!("[{}].PRGROM",self.name).to_owned());
            if header.prg_rom_size == 1 {
                prgrom.make_ram(0x3FFF)?;
                prgrom.add_addr_range_mirrored(0x8000,0xFFFF,16*1024)?;
                console_log!("Addr range PRG ROM 16k...");
            } else if header.prg_rom_size == 2 {
                prgrom.make_ram(0x7FFF)?;
                prgrom.add_addr_range(0x8000, 0xFFFF)?;
                console_log!("Addr range PRG ROM 32k...");
            } else {
                return Err(format!("Corrupt NES 2.0 header: PRG size was {} for mapper 0",header.prgsize()))
            }
            prgrom.bulk_set(0,data[header.prg_rom_start()..=header.prg_rom_end()].to_vec())?;
            self.mainbus.borrow_mut().register_MmioNode(prgrom)?;

            console_log!("Addr range CHR ROM 8k...");
            let mut chrrom = MmioNode::new(format!("[{}].CHRROM",self.name).to_owned());
            chrrom.make_ram(header.chrsize() as u16)?;
            chrrom.bulk_set(0,data[header.chr_rom_start()..=header.chr_rom_end()].to_vec())?;
            self.ppu_bus.borrow_mut().register_MmioNode(chrrom)?;
            
            return Ok(());
        }
        Err(format!("Mapper not implemented: {}", header.mapper()))
    }
}