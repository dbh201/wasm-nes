use std::{cell::RefCell, rc::Rc};

use js_sys::{Date, Uint8Array};
use web_sys::XmlHttpRequest;
use wasm_bindgen::prelude::*;

use super::NES::NES;

// We want to still give output from this function, even
// if console_log is disabled for the other files.

//use super::console_log;

use crate::{SoftwareRenderer::WebGl2DSoftwareRenderer, real_console_log as console_log, MOS_6502::{Cartridge::Cartridge}};

#[wasm_bindgen(start)]
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
    //let onload_callback = Rc::new(RefCell::new(None));
    //*onload_callback.borrow_mut() = None;
    let cb = Closure::<dyn FnMut(_)>::new(move |e: web_sys::EventTarget| {
        console_log!("ONLOAD!");
        let resp: Vec<u8> = Uint8Array::new(&onload_buf.borrow_mut().response().ok().unwrap()).to_vec();
        console_log!("Loading cart...");
        let mut cart = Cartridge::new("Super Mario Bros".to_owned()).unwrap();
        cart.load_mapper(&resp,0);
        let r = onload_nes.borrow_mut().insert_cart(cart);
        if r.is_err() {
            console_log!("Couldn't insert cart: {}", r.unwrap_err());
        }
        let length = resp.len(); 
        console_log!("Response length: {}", length);
        console_log!("NES hardware initialized.");
    });
    request.borrow_mut().set_onload(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    console_log!("Sending request...");
    request.borrow_mut().send()?;
    let mut start = Date::now();
    let mut elapsed: f64 = 0.0;
    console_log!("Initializing SoftwareRenderer...");
    let mut rnd = WebGl2DSoftwareRenderer::new("nes-canvas",320,240)?;
    console_log!("SoftwareRenderer initialized.");
    console_log!("Preparing animation closure...");
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let n = nes.clone();
    let mut frames = 0;
    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let mut nr = n.borrow_mut();
        if nr.cart_inserted {
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
        }
        request_animation_frame(f.borrow().as_ref().unwrap())
    }) as Box<dyn FnMut()>));
    console_log!("Calling animation closure...");
    request_animation_frame(g.borrow().as_ref().unwrap());
    console_log!("Finishing init...");
    return Ok(())
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window().expect("Failed to get window")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("requestAnimationFrame failed");
}