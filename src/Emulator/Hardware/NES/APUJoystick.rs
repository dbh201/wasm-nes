use crate::Emulator::Common::AddressNode::AddressObject;

pub struct APUJoystick {
    //TODO: sound support
    status: u8,
    joy1: u8,
    joy2: u8,
    out: u8,
}
impl APUJoystick {
    pub fn new() -> Result<APUJoystick,String> {
        Ok(APUJoystick { status: 0, joy1: 0, joy2: 0, out: 0 })
    }
}
impl AddressObject for APUJoystick {
    fn get(&mut self, addr: u16) -> Result<u8, String> {
        if addr == 0x0016 {
            // Read port 1
            return Ok(0b00011001)
        }
        if addr == 0x0017 {
            // Read port 2
            return Ok(0b00011001)
        }
        Ok(0)
    }
    fn set(&mut self, addr: u16, val: u8) -> Result<(), String> {
        if addr == 0x0016 {
            // Retrieve joystick input
            self.out = val;
        }
        Ok(())
    }
    fn len(&self) -> usize {
        0x17
    }
}