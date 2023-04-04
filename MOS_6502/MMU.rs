use crate::MmioNode::MmioNode;
pub struct MMU {
    mmio_table: Vec<MmioNode>,
}
impl MMU {
    pub fn new() -> Result<MMU,String> {
        let mmio_table = Vec::new();
        Ok(MMU { mmio_table })
    }
    pub fn register_MmioNode(&mut self, node: MmioNode) -> Result<(),String> {
        for iter in &self.mmio_table {
            if iter.addr <= node.addr && iter.addr + iter.len >= node.addr {
                return Err(format!("Cannot register node {}: interferes with {}",node.name, iter.name));
            }
            if node.name == iter.name {
                return Err(format!("Cannot register node {} at {}: node name already exists at {}",node.name, node.addr, iter.addr));
            }
        }
        self.mmio_table.push( node );
        Ok(())
    }
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
        for node in self.mmio_table.iter() {
            if node.owns_addr(addr) {
                return node.get(addr)
            }
        }
        Err(format!("Addr {} not owned",addr))
    }
    pub fn set(&mut self, addr: u16, value: u8) -> Result<(),String> {
        for node in self.mmio_table.iter_mut() {
            if node.owns_addr(addr) {
                //println!("{} owns {}",node.name,addr);
                return node.set(addr, value);
            }
        }
        Err(format!("Addr {} not owned",addr))
    }
}
