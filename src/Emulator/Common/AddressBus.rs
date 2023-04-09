use super::AddressNode::{AddressNode, AddressType};
use super::super::console_log;

pub struct AddressBus<'a> {
    pub name: String,
    pub mmio_table: Vec<AddressNode<'a>>
}
impl<'a> AddressBus<'a> {
    pub fn new(name: String) -> Result<AddressBus<'a>,String> {
        let mmio_table = Vec::new();
        Ok(AddressBus { name, mmio_table })
    }
    pub fn register_AddressNode(&mut self, node: AddressNode<'a>) -> Result<(),String> {
        console_log!("Registering {}...", node.name);
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
    pub fn unregister_AddressNode_match(&mut self, prefix: String) ->Option<AddressNode<'a>> {
        let index = self.mmio_table.iter().position(|x| x.name.starts_with(&prefix));
        if index.is_some() {
            return Some(self.mmio_table.remove(index.unwrap()));
        }
        return None
    }
    pub fn unregister_AddressNode(&mut self, name: String) -> Option<AddressNode<'a>> {
        let index = self.mmio_table.iter().position(|x| x.name == name);
        if index.is_some() {
            return Some(self.mmio_table.remove(index.unwrap()));
        }
        return None
    }

    // TODO: efficiency improvements to MMIO reading and writing
    pub fn get(&mut self,addr: u16) -> Result<u8,String> {
        console_log!("{}: get {:04X} from {} nodes", self.name, addr, self.mmio_table.len());
        for node in self.mmio_table.iter_mut() {
            let final_addr = node.resolve_addr(addr);
            if final_addr != None {
                console_log!("{}: Final address: {:04X}", self.name, final_addr.clone().unwrap());
                let val = node.get(final_addr.unwrap())?;
                console_log!("Value: {:02X}", val);
                return Ok(val)
            } else {
                console_log!("{}: {:04X} not owned by {}",self.name, addr, node.name);
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
    pub fn _clock_tick(&mut self, obj_type: AddressType, name: Option<String> ) -> Result<(), String> {
        for node in self.mmio_table.iter_mut() {
            if node.obj_type == obj_type {
                if name.is_none() || (name.is_some() && &node.name == name.as_ref().unwrap()) {
                    return node._clock_tick()
                } 
            }
        }
        Err(format!("{}: Cannot tick for mmiotype with name {} (not found)",self.name, obj_type))
    }
}
pub trait MemRW {
    fn setmem(&mut self, addr: u16, val: u8) -> Result<(), String>;
    fn getmem(&self, addr: u16) -> Result<u8, String>;
}