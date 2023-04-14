use super::*;
use std::fs;
use std::env::current_dir;

#[test]
fn full_SuperMarioBros() {
    let mut nes = NES::new().unwrap();
    println!("{}",current_dir().ok().unwrap().to_string_lossy());
    let data = fs::read("super-mario-bros.nes");
    assert!(data.is_ok(),"Couldn't load file: {}",data.err().unwrap());
    let mut cart = Cartridge::Cartridge::new("SMBTEST".to_owned(),nes.get_mainbus(), nes.get_ppu_bus()).expect("Cartridge::new failed");
    cart.load_nes_file(&data.ok().unwrap());
    nes.insert_cart(cart);

    // Run a million instructions to see if it crashes
    for _ in 0..1_000_000 {
        assert!(nes.clock_tick().is_ok());
    }
    assert!(true);
}