#![allow(non_snake_case)]
mod MmioNode;
mod AddressBus;
mod RamBank;
mod PPU;
mod APUJoystick;
pub mod Cartridge;
pub mod NES;
pub mod Mos6502Debug;
pub mod Mos6502Isa;
pub mod Mos6502;


mod tests;
mod NES_wasm;

//pub use crate::dummy_console_log as console_log;
pub use crate::test_console_log as console_log;