use crate::Emulator::Common::AddressNode::AddressObject;

pub struct APUJoystick {
    //TODO: sound support
    status: u8
}
impl APUJoystick {
    pub fn new() -> Result<APUJoystick,String> {
        Ok(APUJoystick { status: 0 })
    }
}
impl AddressObject for APUJoystick {
    fn get(&mut self, addr: u16) -> Result<u8, String> {
        Ok(0)
    }
    fn set(&mut self, addr: u16, val: u8) -> Result<(), String> {
        Ok(())
    }
    fn len(&self) -> usize {
        0x17
    }
}