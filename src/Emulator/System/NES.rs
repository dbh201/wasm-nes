use std::{cell::RefCell, rc::Rc};
use crate::Emulator::Common::AddressNode::{AddressNode,AddressType};

use super::super::Hardware::NES::Cartridge::Cartridge;
use super::super::CPU::MOS_6502::Mos6502::Mos6502;
use super::super::Common::AddressBus::AddressBus;

use super::super::console_log;

pub struct NES<'nes> {
    pub cpu: Mos6502<'nes>,
    mainbus: Rc<RefCell<AddressBus<'nes>>>,
    ppu_bus: Rc<RefCell<AddressBus<'nes>>>,
    pub cart: Option<Cartridge<'nes>>,
    pub clock: usize,
    NTSC: bool
}
impl<'nes> NES<'nes> {
    pub fn get_mainbus(&self) -> Rc<RefCell<AddressBus<'nes>>> {
        self.mainbus.clone()
    }
    pub fn get_ppu_bus(&self) -> Rc<RefCell<AddressBus<'nes>>> {
        self.ppu_bus.clone()
    }
    pub fn new() -> Result<NES<'nes>,String> {
        console_log!(":::Initializing mainbus...");
        let bus = Rc::new(RefCell::new(AddressBus::new("NES MAINBUS".to_owned()).unwrap()));
        console_log!(":::Initializing ppubus...");
        let ppu_mmu = Rc::new(RefCell::new(AddressBus::new("NES PPUMMU".to_owned()).unwrap()));
        console_log!(":::Initializing cpu...");
        let mut cpu = Mos6502::new("NES CPU".to_owned(),bus.clone()).unwrap();
        
        console_log!(":::Initializing ram...");
        let mut ram = AddressNode::new("2KB RAM".to_owned());
        ram.make_ram(0x07FF)?;
        ram.add_addr_range_mirrored(0,0x1FFF,0x800)?;
        bus.borrow_mut().register_AddressNode(ram)?;

        console_log!(":::Initializing ppu...");
        let mut ppu = AddressNode::new("NES PPU".to_string());
        ppu.make_ppu(bus.clone(),ppu_mmu.clone())?;
        ppu.add_addr_range_mirrored(0x2000, 0x3FFF, 8)?;
        ppu.add_addr_range(0x4014, 0x4014)?; // 1 byte for DMA
        bus.borrow_mut().register_AddressNode(ppu)?;

        console_log!(":::Initializing apu...");
        let mut apu = AddressNode::new("NES APU/Joystick".to_string());
        apu.make_apu()?;
        apu.add_addr_range(0x4000,0x401F)?;
        bus.borrow_mut().register_AddressNode(apu).expect("NES APU:");
        Ok(NES { cpu, mainbus: bus, ppu_bus: ppu_mmu, clock: 0, NTSC: true, cart: None })
    }
    pub fn insert_cart(&mut self, cart: Cartridge<'nes>) -> Result<(), String> {
        self.cart = Some(cart);
        self.cart.as_mut().unwrap().register()?;
        self.reset();
        Ok(())
    }
    pub fn remove_cart(&mut self) -> Result<(), String> {
        if self.cart.is_some() {
            self.cart.as_mut().unwrap().unregister()?;
            self.cart = None;
            return Ok(())
        } else {
            Err("No cart inserted".to_owned())
        }
    }
    pub fn reset(&mut self) -> Result<(), String> {
        self.cpu.reset()
    }
    // TODO: maybe wait or check for vblank on the PPU side of things
    pub fn step_frame(&mut self) -> Result<(), String> {
        let cpu_count1: usize;
        let cpu_count2: usize;
        if self.NTSC {
            cpu_count1 = 29780;
            cpu_count2 = 29781;
        } else {
            cpu_count1 = 33247;
            cpu_count2 = 33248;
        }
        while self.clock % cpu_count1 != 0 && self.clock % (cpu_count1 + cpu_count2) != 0 {
            self.clock_tick()?;
        }
        self.clock_tick()
    }
    pub fn clock_tick(&mut self) -> Result<(),String> {
        if self.cart.is_none() {
            return Ok(());
        }
        self.clock += 1;
        if self.NTSC {
            if (self.clock % 12) == 1 {
                return self.cpu._clock_tick()
            };
            if (self.clock % 4) == 1 {
                return self.mainbus.borrow_mut()._clock_tick(AddressType::PPU, None)
            };
            Ok(())
        } else {
            if (self.clock % 16) == 1 {
                return self.cpu._clock_tick()
            };
            if (self.clock % 5) == 1 {
                return self.mainbus.borrow_mut()._clock_tick(AddressType::PPU, None)
            };
            Ok(())
        }
    }
}