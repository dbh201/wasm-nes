use std::{cell::RefCell, rc::Rc};


use super::AddressBus::AddressBus;
use super::Cartridge::Cartridge;
use super::MmioNode::{MmioNode, MmioType};
use super::Mos6502::Mos6502;
use super::console_log;

pub struct NES<'nes> {
    pub cpu: Mos6502<'nes>,
    mainbus: Rc<RefCell<AddressBus<'nes>>>,
    ppu_bus: Rc<RefCell<AddressBus<'nes>>>,
    pub clock: usize,
    pub cart_inserted: bool,
    NTSC: bool
}
impl<'nes> NES<'nes> {
    pub fn new() -> Result<NES<'nes>,String> {
        console_log!(":::Initializing mainbus...");
        let bus = Rc::new(RefCell::new(AddressBus::new("NES MAINBUS".to_owned()).unwrap()));
        console_log!(":::Initializing ppubus...");
        let ppu_mmu = Rc::new(RefCell::new(AddressBus::new("NES PPUMMU".to_owned()).unwrap()));
        console_log!(":::Initializing cpu...");
        let mut cpu = Mos6502::new("NES CPU".to_owned(),bus.clone()).unwrap();
        
        console_log!(":::Initializing ram...");
        let mut ram = MmioNode::new("2KB RAM".to_owned());
        ram.make_ram(0x07FF)?;
        ram.add_addr_range_mirrored(0,0x1FFF,0x800)?;
        bus.borrow_mut().register_MmioNode(ram)?;

        console_log!(":::Initializing ppu...");
        let mut ppu = MmioNode::new("NES PPU".to_string());
        ppu.make_ppu(bus.clone(),ppu_mmu.clone())?;
        ppu.add_addr_range_mirrored(0x2000, 0x3FFF, 8)?;
        ppu.add_addr_range(0x4014, 0x4014)?; // 1 byte for DMA
        bus.borrow_mut().register_MmioNode(ppu)?;

        console_log!(":::Initializing apu...");
        let mut apu = MmioNode::new("NES APU/Joystick".to_string());
        apu.make_apu()?;
        apu.add_addr_range(0x4000,0x401F)?;
        bus.borrow_mut().register_MmioNode(apu).expect("NES APU:");
        Ok(NES { cpu, mainbus: bus, ppu_bus: ppu_mmu, clock: 0, NTSC: true, cart_inserted: false })
    }
    pub fn insert_cart(&mut self, cart: Cartridge<'nes>) -> Result<(), String> {
        let mut node = MmioNode::new(cart.name());
        node.insert_cart(cart)?;
        node.add_addr_range(0x4020, 0xFFFF)?;
        self.mainbus.borrow_mut().register_MmioNode(node)?;
        self.cart_inserted = true;
        Ok(())
    }
    pub fn remove_cart(&mut self) -> Result<(), String> {
        let mut bus = self.mainbus.borrow_mut();
        let mut name: Option<String> = None;
        for node in bus.mmio_table.iter() {
            if node.obj_type == MmioType::CART {
                name = Some(node.name.clone());
                break;
            }
        }
        if name.is_some() {
            self.cart_inserted = false;
            return bus.unregister_MmioNode(name.unwrap());
        }
        Err("No cart inserted".to_owned())
    }
    pub fn reset(&mut self) -> Result<(), String> {
        self.cpu.reset()
    }
    pub fn clock_tick(&mut self) -> Result<(),String> {
        if !self.cart_inserted {
            return Ok(());
        }
        self.clock += 1;
        if self.NTSC {
            if (self.clock % 12) == 1 {
                return self.cpu._clock_tick()
            };
            if (self.clock % 4) == 1 {
                return self.mainbus.borrow_mut()._clock_tick(MmioType::PPU, None)
            };
            Ok(())
        } else {
            if (self.clock % 12) == 1 {
                return self.cpu._clock_tick()
            };
            if (self.clock % 4) == 1 {
                return self.mainbus.borrow_mut()._clock_tick(MmioType::PPU, None)
            };
            Ok(())
        }
    }
}