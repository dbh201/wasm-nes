struct MMIO_node {
    name: String,
    addr: u16,
    len: u16,
    _set_func: &dyn Fn(u16,u8) -> Result<(),String>,
    _get_func: &dyn Fn(u16) -> Result<(),String>,
}
impl MMIO_node {

    pub fn new(name: String,addr: u16,len: u16,
        get_func: Option(&dyn Fn(u16) -> Result<(), String>),
        set_func: Option(&dyn Fn(u16,u8) -> Result<(), String>)) -> MMIO_node {

            let res = { name, addr, len };
            res.use_set_func(set_func);
            res.use_get_func(get_func);
            res

    }

    pub fn use_set_func(&mut self, new_func: Option(&dyn Fn(u16,u8) -> Result<(), String>) ) -> Result<(), String> {
        self._set_func = new_func.is_none() ? self.default_set : new_func.unwrap();
    }

    pub fn use_get_func(&mut self, new_func: &dyn Fn(u16,u8)) -> Result<(), String> {
        self._get_func = new_func.is_none() ? self.default_get : new_func.unwrap();
    }

    fn default_set(&self, addr: u16, val: u8) -> Result<(), String> {
        Err(format!("no getter for MMIO node {}, which owns {}-{} (tried {} = {})",self.name,self.addr,self.addr+self.len,addr,val))
    }

    fn default_get(&self, addr: u16) -> Result<(), String> {
        Err(format!("no getter for MMIO node {}, which owns {}-{} (requested {})",self.name,self.addr,self.addr+self.len,addr))
    }

    pub fn owns_addr(&self, addr: u16) -> bool {
        addr >= node.addr && addr < node.addr + len 
    }

    pub fn get(&self, addr: u16) -> Result<(), String> {
        if self._get_func == None {
            self.default_get(addr)
        }
        self._get_func(addr - self.addr)
    }

    pub fn set(&self, addr: u16, val: u8) -> Result<(), String> {
        if self._set_func == None {
            self.default_set(addr, val)
        }
        self._set_func(addr - self.addr, val)
    }

}
        
