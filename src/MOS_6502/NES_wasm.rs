use std::{cell::RefCell, rc::Rc};

use js_sys::Date;
use wasm_bindgen::prelude::*;

use super::NES::NES;
use crate::{SoftwareRenderer::WebGl2DSoftwareRenderer, real_console_log as console_log, MOS_6502::{Cartridge::Cartridge, MmioNode::MmioNode, Mos6502Debug::AddrMode}};

#[wasm_bindgen(start)]
pub fn run_nes() -> Result<(), JsValue> {
    console_log!("Entering run_nes...");
    let nes = NES::<'static>::new();
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
    let mut elapsed: f64 = 0.0;
    console_log!("Initializing SoftwareRenderer...");
    let mut rnd = WebGl2DSoftwareRenderer::new("nes-canvas",320,240)?;
    console_log!("SoftwareRenderer initialized.");
    console_log!("Preparing animation closure...");
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let n = Rc::new(RefCell::new(nes));
    let mut frames = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut nr = n.borrow_mut();
        while nr.clock % 89341 != 0 && nr.clock % (89341+89342) != 0 {
            nr.clock_tick();
        }
        nr.clock_tick();
        rnd.draw();
        frames += 1;
        elapsed = Date::now();
        if elapsed - start > 1000.0 {
            console_log!("{} ticks, {} fps", nr.clock, (frames as f64) * 1000.0 / (elapsed - start));
            start = elapsed;
            frames = 0;
        }
        request_animation_frame(f.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut()>));
    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window().expect("Failed to get window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requestAnimationFrame failed");
}