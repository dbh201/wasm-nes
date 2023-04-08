//TODO: This needs to be a ROM + mappers and address spaces
// mappers should Have their own MmioObjects...
// should they be managed by another bus? 

use super::{AddressBus::AddressBus, MmioNode::{MmioObject, MmioNode}};
use super::console_log;

pub struct Cartridge<'a> {
    pub data: AddressBus<'a>
}
struct NESHeader {
    prg_rom_size: u8,
    chr_rom_size: u8,
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
    pub fn prgsize(&self) -> u8 {
        return self.prg_rom_size
    }
    pub fn chrsize(&self) -> u8 {
        return self.chr_rom_size
    }
    pub fn mapper(&self) -> u8 {
        return (self.flags_6 >> 4) + (self.flags_7 & 0xF0)
    }
    pub fn prg_rom_start(&self) -> usize {
        return 0x10;
    }
    pub fn prg_rom_end(&self) -> usize {
        return self.prg_rom_start() + ((self.prg_rom_size as usize)<<14) - 1;
    }
}
impl<'a> Cartridge<'a> {
    pub fn new(name: String) -> Result<Cartridge<'a>,String> {
        console_log!("Init Cartridge");
        let data = AddressBus::new(name)?;
        Ok(Cartridge { data })
    }
    pub fn name(&self) -> String {
        self.data.name.to_string()
    }
    pub fn clock_tick(&mut self) -> Result<(), String> {
        Ok(())
    }
    pub fn load_nes_file(&mut self, data: &Vec<u8>) -> Result<(), String> {
        let header = NESHeader::new(data)?;
        if header.mapper() == 0 {
            let mut prgram = MmioNode::new("PRG RAM".to_owned());
            prgram.make_ram(0x1FFF)?;
            prgram.add_addr_range_mirrored(0x6000, 0x7FFF, 0)?;
            self.data.register_MmioNode(prgram)?;
            let mut prgrom = MmioNode::new("PRG ROM".to_owned());

            if header.prgsize() == 1 {
                prgrom.make_ram(0x3FFF)?;
                prgrom.add_addr_range_mirrored(0x8000,0xFFFF,16*1024)?;
                console_log!("Addr range PRG ROM 16k...");
            } else if header.prgsize() == 2 {
                prgrom.make_ram(0x7FFF)?;
                prgrom.add_addr_range(0x8000, 0xFFFF)?;
                console_log!("Addr range PRG ROM 32k...");
            } else {
                return Err(format!("Corrupt NES 2.0 header: PRG size was {} for mapper 0",header.prgsize()))
            }
            console_log!("Loading...");
            for i in header.prg_rom_start()..=header.prg_rom_end() {
                prgrom.set((i - header.prg_rom_start()) as u16,data[ i as usize ])?;
            } 
            console_log!("Pre-register");
            self.data.register_MmioNode(prgrom)?;
            return Ok(());
        }
        Err(format!("Mapper not implemented: {}", header.mapper()))
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