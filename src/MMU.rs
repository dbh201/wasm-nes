struct MMU {
    mmio_table: Vec<MMIO_node>,
    mem: [u8;64*1024],
}
impl MMU {
    new() -> Result<MMU,String> {
        mem = [u8;64*1024];
        mmio_table = new Vec();
        Ok({ mem, mmio_table })
    }
    register_mmio_node(&mut self, node: MMIO_node) -> Result<(),String> {
        self.mmio_table.append( node );
        Ok(())
    }
    unregister_mmio_node(&mut self, name: String) -> Result<(),String> {
        let index = self.mmio_table.iter().position(|&x| x.name == name);
        if index.is_some() {
            self.mmio_table.remove(index);
            Ok(())
        }
        Err(format!("No such MMIO node: {}",name))
    }

    // TODO: efficiency improvements to MMIO reading and writing
    get(&self,addr: u16) -> Result<u8,String> {
        for node in &self.mmio_table {
            if node.owns_addr(addr) {
                node.get(addr)
            }
        }
        Ok(mem[addr])
    }
    set(&mut self, addr: u16, value: u8) -> Result<(),String> {
        for node in &self.mmio_table {
            if node.owns_addr(addr) {
                node.set(addr, value)
            }
        }
        Ok(mem[addr])
    }
}
