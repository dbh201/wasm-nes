use crate::MmioNode::MmioNode;
pub struct MMU<'a> {
    mmio_table: Vec<MmioNode<'a>>,
    mem: [u8;64*1024],
}
impl<'a> MMU<'a> {
    pub fn new() -> Result<MMU<'a>,String> {
        let mem: [u8;64*1024] = [0;64*1024];
        let mmio_table = Vec::new();
        Ok(MMU { mem, mmio_table })
    }
    #[allow(non_snake_case)]
    pub fn register_MmioNode(&mut self, node: MmioNode<'a>) -> Result<(),String> {
        self.mmio_table.push( node );
        Ok(())
    }
    #[allow(non_snake_case)]
    pub fn unregister_MmioNode(&mut self, name: String) -> Result<(),String> {
        let index = self.mmio_table.iter().position(|x| x.name == name);
        if index.is_some() {
            self.mmio_table.remove(index.unwrap());
            return Ok(());
        }
        Err(format!("No such MMIO node: {}",name))
    }

    // TODO: efficiency improvements to MMIO reading and writing
    pub fn get(&self,addr: u16) -> Result<u8,String> {
        for node in &self.mmio_table {
            if node.owns_addr(addr) {
                return node.get(addr)
            }
        }
        Ok(self.mem[addr as usize])
    }
    pub fn set(&mut self, addr: u16, value: u8) -> Result<(),String> {
        for node in &self.mmio_table {
            if node.owns_addr(addr) {
                println!("{} owns {}",node.name,addr);
                return node.set(addr, value);
            }
        }
        self.mem[addr as usize] = value;
        Ok(())
    }
}
