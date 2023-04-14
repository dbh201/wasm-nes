use std::{cell::RefCell, rc::Rc};

use js_sys::{Date, Uint8Array};
use web_sys::{XmlHttpRequest, EventTarget};
use wasm_bindgen::prelude::*;

// We want to still give output from this function, even
// if console_log is disabled for the other files.

use crate::Emulator::Common::AddressNode::{AddressType, AddressNode};
//use super::console_log;
use crate::Emulator::System::NES::NES;
pub use crate::Emulator::Hardware::NES::PPU::{PPU, PPU_Renderer};
use crate::Renderer::{NES_wasm, WebGL};
use crate::real_console_log as console_log;
use crate::Emulator::Hardware::NES::Cartridge::Cartridge;
use crate::Renderer::WebGL::WebGl2DSoftwareRenderer;

#[wasm_bindgen]
pub fn run_nes() -> Result<(), JsValue> {
    console_log!("Entering run_nes...");
    let nes = NES::<'static>::new();
    if nes.is_err() {
        return Err(nes.err().into())
    }
    let nes = Rc::new(RefCell::new(nes.unwrap()));
    console_log!("Preparing request...");
    let request = Rc::new(RefCell::new(XmlHttpRequest::new()?));

    request.borrow_mut().open("GET","http://localhost:5173/super-mario-bros.nes")?;
    request.borrow_mut().set_response_type(web_sys::XmlHttpRequestResponseType::Arraybuffer);
    let onload_buf = request.clone();
    let onload_nes = nes.clone();
    let nes_frame_rc = nes.clone();
    //let onload_callback = Rc::new(RefCell::new(None));
    //*onload_callback.borrow_mut() = None;
    let cb = Closure::<dyn FnMut(_)>::new(move |e: web_sys::EventTarget| {
        console_log!("ONLOAD!");
        let resp: Vec<u8> = Uint8Array::new(&onload_buf.borrow_mut().response().ok().unwrap()).to_vec();
        console_log!("Loading cart...");
        let mut cart = Cartridge::new(
            "Super Mario Bros".to_owned(),
            onload_nes.borrow().get_mainbus(),
            onload_nes.borrow().get_ppu_bus()
        ).unwrap();
        console_log!("Mapping cart of len {:08X}...",&resp.len());
        let ret = cart.load_nes_file(&resp).err();
        if ret.is_some() {
            console_log!("{}",ret.unwrap());
        }
        console_log!("Inserting cart...");
        let r = onload_nes.borrow_mut().insert_cart(cart);
        if r.is_err() {
            console_log!("Couldn't insert cart: {}", r.unwrap_err());
        }
        let length = resp.len(); 
        console_log!("Response length: {}", length);
        let r = onload_nes.borrow_mut().reset();
        if r.is_err() {
            console_log!("Couldn't reset NES: {}", r.unwrap_err());
        }
        onload_nes.borrow_mut().clock_tick();
        onload_nes.borrow_mut().clock_tick();
        console_log!("NES hardware initialized.");
    });
    request.borrow_mut().set_onload(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    console_log!("Sending request...");
    request.borrow_mut().send()?;
    let mut start = Date::now();
    let mut elapsed: f64 = 0.0;
    let mut frame_start: f64 = 0.0;
    let mut idle: f64 = 0.0;
    let mut clocks = 0;
    console_log!("Initializing SoftwareRenderer...");
    let mut rnd = WebGl2DSoftwareRenderer::new("nes-canvas", 256, 240)?;
    console_log!("SoftwareRenderer initialized.");
    console_log!("Preparing animation closure...");
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let n = nes.clone();
    let mut frames = 0;
    let pause = Rc::new(RefCell::new(true));
    let pause_rc = pause.clone();
    let pause_anim_rc = pause.clone();
    let pause_step_rc = pause.clone();
    let pause_frame_rc = pause.clone();
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut nr = n.borrow_mut();
        console_log!("Checking for cart...");
        if nr.cart.is_some() && !*pause_anim_rc.borrow(){
            console_log!("Cart found...");
            idle += elapsed - frame_start;
            frame_start = Date::now();

            clocks = nr.clock;
            console_log!("Stepping frame...");
            let ret = nr.step_frame();
            if ret.is_err() {
                console_log!("{}", ret.err().unwrap());
            }
            clocks = nr.clock - clocks;
            console_log!("Cycles this frame: {} ({})", clocks, nr.clock);
            //console_log!("Updating clock...");
            console_log!("Generating frame...");
            let ret = rnd.generate_frame(nr.get_framebuffer()).err();
            //console_log!("POST");
            if ret.is_some() {
                console_log!("{}", ret.unwrap());
            }
            console_log!("Drawing frame...");
            let ret = rnd.draw().err();
            if ret.is_some() {
                console_log!("Some kind of draw error...");
                console_log!("{}", ret.unwrap().as_string().unwrap());
            }
            frames += 1;
            elapsed = Date::now();
            if elapsed - start > 1000.0 {
                console_log!("{} ticks per frame [@{:04X} now], {} fps ({} ms idle)", clocks, nr.cpu.pc, (frames as f64) * 1000.0 / (elapsed - start), idle);
                /*
                let mut palstr = String::new();
                let mainbus = nr.get_mainbus();
                let mut ppu_mut = mainbus.borrow_mut();
                let ppu = ppu_mut.
                mmio_table.iter_mut().
                find(|x| x.obj_type == AddressType::PPU
                );
                let ppu = ppu.unwrap();
                let mut bus = ppu.ppu.as_mut().unwrap().ppubus.borrow_mut();
                for x in(0..0x20).step_by(0x08) {
                    for y in 0..0x08 {
                        let v = bus.get(0x3F00+x+y).unwrap();
                        palstr.extend(
                            format!("{:02X} ",v).chars()
                        )
                    }
                    palstr.extend("\n".chars())
                }
                console_log!("Bgcol: {:02X}",bus.get(0x3F00).unwrap());
                console_log!("--Palette--\n{}",palstr);
                */
                start = elapsed;
                frames = 0;
                idle = 0.0;
            }
        }
        request_animation_frame(f.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut()>));
    console_log!("Calling animation closure...");
    request_animation_frame(g.borrow().as_ref().unwrap());
    console_log!("Preparing pause/resume button...");

    let document = web_sys::window().ok_or("Couldn't get window")?.document().ok_or("Couldn't get document")?;
    let pause_button = document.get_element_by_id("pause-button").ok_or("Couldn't get pause-button")?;
    let pause_button_closure = Closure::wrap(Box::new(move |e: EventTarget| {
        let mut newp = pause_rc.borrow_mut();
        *newp = !(*newp);
    }) as Box<dyn FnMut(_)>);
    
    pause_button.add_event_listener_with_callback("click", pause_button_closure.as_ref().unchecked_ref())?;
    pause_button_closure.forget();
    console_log!("Preparing single-step button...");
    let nes_step_rc = nes.clone();

    let step_button = document.get_element_by_id("step-button").ok_or("Couldn't get step-button")?;
    let step_button_closure = Closure::wrap(Box::new(move |e: EventTarget| {
        if *pause_step_rc.borrow() {
            console_log!("Executing:\n{}\n...",nes_step_rc.borrow().cpu);
            let ret = nes_step_rc.borrow_mut().step_cpu().err();
            if ret.is_some() {
                console_log!("Step error: {}",ret.unwrap());
            } else {
                console_log!("Clock ticks: {}",nes_step_rc.borrow().clock);
            }
        } else {
            console_log!("Cannot step!");
        }
    }) as Box<dyn FnMut(_)>);
    step_button.add_event_listener_with_callback("click",step_button_closure.as_ref().unchecked_ref())?;
    step_button_closure.forget();

    let frame_button = document.get_element_by_id("frame-button").ok_or("Couldn't get frame-button")?;

    let frame_button_closure = Closure::wrap(Box::new(move |e: EventTarget| {
        if *pause_frame_rc.borrow() {
            
            let ret = nes_frame_rc.borrow_mut().step_frame().err();
            if ret.is_some() {
                console_log!("Step error: {}",ret.unwrap());
            } else {
                console_log!("Executing:\n{}\n...",nes_frame_rc.borrow().cpu);
                console_log!("Clock ticks: {}",nes_frame_rc.borrow().clock);
            }
        } else {
            console_log!("Cannot step frame!");
        }
    }) as Box<dyn FnMut(_)>);
    frame_button.add_event_listener_with_callback("click",frame_button_closure.as_ref().unchecked_ref())?;
    frame_button_closure.forget();
    console_log!("Finishing init...");

    return Ok(())
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window().expect("Failed to get window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requestAnimationFrame failed");
}
trait NES_framebuffer {
    fn generate_frame(&mut self, nes: Rc<RefCell<Vec<u8>>>) -> Result<(), String>;
}
impl NES_framebuffer for WebGl2DSoftwareRenderer {
    fn generate_frame(&mut self, nes: Rc<RefCell<Vec<u8>>>) -> Result<(), String> {
        //console_log!("Getting mutref for framebuffer");
        let nesref = nes.borrow_mut();
        //console_log!("Replacing pixel buffer");
        let mut newbuf = Vec::<u8>::new();
        for x in 0..256 {
            for y in 0..240 {
                let pixel = nesref[x + y*256];
                let val: &[u8;3] = &WebGL::PALETTE[(pixel & 0x3F) as usize];
                newbuf.push(val[0]);
                newbuf.push(val[1]);
                newbuf.push(val[2]);
                newbuf.push(255);
            }
        }
        self.replace_pixel_buffer_vec(newbuf)?;
        Ok(())
    }
}
// According to NES DEV:
// pre-render scanline 261
// visible 0-239
// idle 240
// vblank 241-260

// 262 scanlines in total, for 89342 cycles (NTSC)

// clock reset on tick 2 of scanline 241, with the vblank nmi

#[allow(non_camel_case_types)]
impl PPU_Renderer for PPU<'_> {
    fn _clock_tick(&mut self) -> Result<(), String> {
        if self.scanline < 240 && self.scanline_cycle < 257 {
            //console_log!("Pixel {},{}: {:02X}",self.scanline, self.scanline_cycle, self.pixel.as_ref().unwrap());
            self.framebuffer.borrow_mut()[self.scanline_cycle + self.scanline*256 - 1] = self.pixel.unwrap();
        }
        return Ok(())
    }
}
