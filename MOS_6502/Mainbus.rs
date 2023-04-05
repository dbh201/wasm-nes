use crate::MmioNode::MmioNode;

pub struct Mainbus<'a> {
    name: String,
    pub mmio_table: Vec<MmioNode<'a>>
}
impl<'a> Mainbus<'a> {
    pub fn new(name: String) -> Result<Mainbus<'a>,String> {
        let mmio_table = Vec::new();
        Ok(Mainbus { name, mmio_table })
    }
    pub fn register_MmioNode(&mut self, node: MmioNode<'a>) -> Result<(),String> {
        //println!("Registering {}...", node.name);
        for iter in &self.mmio_table {
            if node.name == iter.name {
                return Err(format!("MMU {}: Cannot register node {}: node name already exists", self.name, node.name));
            }
            for node_iter in node.ownership.iter() {
                let lower = iter.get_addr_range(node_iter[0]);
                if lower.is_some() {
                    let lower = lower.unwrap();
                    return Err(format!("MMU {}: Cannot register node {} [{:04X}..{:04X}]: interferes with {} [{:04X}..{:04X}]",
                    self.name, node.name, node_iter[0], node_iter[1], iter.name, lower[0], lower[1]));
                }
                let upper = iter.get_addr_range(node_iter[1]);
                if upper.is_some() {
                    let upper = upper.unwrap();
                    return Err(format!("MMU {}: Cannot register node {} [{:04X}..{:04X}]: interferes with {} [{:04X}..{:04X}]",
                    self.name, node.name, node_iter[0], node_iter[1], iter.name, upper[0], upper[1]));
                }
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
        Err(format!("MMU {}: No such MMIO node: {}",self.name, name))
    }

    // TODO: efficiency improvements to MMIO reading and writing
    pub fn get(&self,addr: u16) -> Result<u8,String> {
        //println!("{}: get {:04X} from {} nodes", self.name, addr, self.mmio_table.len());
        for node in self.mmio_table.iter() {
            let final_addr = node.resolve_addr(addr);
            //println!("Final address: {:04X}",final_addr.clone().unwrap());
            if final_addr != None {
                let val = node.get(final_addr.unwrap());
                //println!("Value: {:02X}", val.clone().unwrap());
                return val
            }
        }
        Err(format!("MMU {}: get @{:04X} failed (addr not owned)", self.name, addr))
    }
    pub fn set(&mut self, addr: u16, value: u8) -> Result<(),String> {
        for node in self.mmio_table.iter_mut() {
            let final_addr = node.resolve_addr(addr);
            if final_addr != None {
                return node.set(final_addr.unwrap(), value);
            }
        }
        Err(format!("MMU {}: set @{:04X}={:02X} failed (addr not owned)", self.name, addr, value))
    }
}
pub trait MemRW {
    fn setmem(&mut self, addr: u16, val: u8);
    fn getmem(&self, addr: u16) -> u8;
}