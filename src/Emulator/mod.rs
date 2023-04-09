pub mod Common {
    pub mod AddressBus;
    pub mod AddressNode;
    pub mod RamBank;
}
pub mod CPU {
    pub mod MOS_6502 {
        pub mod Mos6502;
        pub mod Mos6502Debug;
        pub mod Mos6502Isa;
    }
}
pub mod Hardware {
    pub mod NES {
        pub mod APUJoystick;
        pub mod Cartridge;
        pub mod PPU;
    }
}
pub mod System {
    pub mod NES;
}

pub mod tests;

use crate::dummy_console_log as console_log;