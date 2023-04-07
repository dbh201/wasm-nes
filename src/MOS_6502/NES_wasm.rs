use js_sys::Date;
use wasm_bindgen::prelude::*;

use super::NES::NES;
use crate::{SoftwareRenderer::WebGl2DSoftwareRenderer, console_log, MOS_6502::{Cartridge::Cartridge, MmioNode::MmioNode, Mos6502Debug::AddrMode}};

#[wasm_bindgen(start)]
pub fn run_nes() -> Result<(), JsValue> {
    console_log!("Entering run_nes...");
    let nes = NES::new();
    if nes.is_err() {
        return Err(nes.err().into())
    }
    let mut nes = nes.unwrap();
    let mut cart = Cartridge::new("Dummy Cart".to_owned())?;
    let mut node = MmioNode::new("dummy RAM".to_owned());

    // Offset from 0x4020
    node.make_ram(0xBFE0)?;
    node.add_addr_range(0, 0xBFDF)?;
    node.set(0xBFDC,0x34)?;
    node.set(0xBFDD,0x42)?;
    node.set(0x0214,nes.cpu.debug.get_opcode("JMP", AddrMode::ABSOLUTE))?;
    node.set(0x0215,0x34)?;
    node.set(0x0216,0x42)?;
    cart.data.register_MmioNode(node)?;
    nes.insert_cart(cart)?;
    nes.reset()?;
    console_log!("NES hardware initialized.");
    let mut start = Date::now();
    let mut elapsed: f64;
    console_log!("Initializing SoftwareRenderer...");
    let mut rnd = WebGl2DSoftwareRenderer::new("nes-canvas",320,240)?;
    console_log!("SoftwareRenderer initialized.");
    loop {
        nes.clock_tick()?;
        rnd.draw();
        if nes.clock % 10000 == 0 {
            elapsed = Date::now();
            console_log!("{} ticks in {} ms", nes.clock, elapsed - start);
            start = elapsed;
        }
    }
}