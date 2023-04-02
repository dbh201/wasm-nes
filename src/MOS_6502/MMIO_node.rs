pub struct MMIO_node<'a> {
    pub(crate) name: String,
    addr: u16,
    len: u16,
    get_func: Option<&'a dyn Fn(u16) -> Result<u8, String>>,
    set_func: Option<&'a dyn Fn(u16, u8) -> Result<(), String>>,
}
impl<'a> MMIO_node<'a> {
    pub fn new(
        name: String,
        addr: u16,
        len: u16,
        get_func: Option<&'a dyn Fn(u16) -> Result<u8, String>>,
        set_func: Option<&'a dyn Fn(u16, u8) -> Result<(), String>>,
    ) -> MMIO_node<'a> {
        let res = MMIO_node { 
            name, 
            addr, 
            len, 
            get_func,
            set_func
        };
        res
    }

    pub fn use_set_func(
        &mut self,
        new_func: Option<&'a dyn Fn(u16, u8) -> Result<(), String>>,
    ) -> Result<(), String> {
        self.set_func = new_func;
        Ok(())
    }

    pub fn use_get_func(&mut self, new_func: Option<&'a dyn Fn(u16) -> Result<u8, String>>) -> Result<(), String> {
        self.get_func = new_func;
        Ok(())
    }

    pub fn owns_addr(&self, addr: u16) -> bool {
        addr >= self.addr && addr < self.addr + self.len
    }

    pub fn get(&self, addr: u16) -> Result<u8, String> {
        if self.get_func.is_some() {
            (self.get_func.unwrap())(addr - self.addr)
        } else {
            Err(format!(
                "no getter for MMIO node {}, which owns {}-{} (requested {})",
                self.name,
                self.addr,
                self.addr + self.len,
                addr
            ))
        }
        
    }

    pub fn set(&self, addr: u16, val: u8) -> Result<(), String> {
        if self.set_func.is_some() {
            (self.set_func.unwrap())(addr - self.addr, val)
        } else {
            Err(format!(
                "no setter for MMIO node {}, which owns {}-{} (tried {} = {})",
                self.name,
                self.addr,
                self.addr + self.len,
                addr,
                val
            ))
        }
    }
}
