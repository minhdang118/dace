use std::collections::HashMap;

pub struct Cache {
    pub(crate) status: HashMap<String, usize>,
    pub(crate) size: usize,
    pub(crate) miss: i32,
}

impl Cache {
    pub fn access(&mut self, new_addr: String, new_lease: usize) {
        let mut to_remove = vec![];

        for (addr, lease) in self.status.iter_mut() {
            *lease -= 1;
            if *lease == 0 {
                to_remove.push(addr.clone());
            }
        }
        for addr in to_remove.iter() {
            self.status.remove(addr);
        }

        if !self.status.contains_key(&new_addr) && (self.status.len() == self.size) {
            self.miss += 1;
            // TODO: Random eviction
        } else {
            // Insert or update the lease
            self.status.insert(new_addr, new_lease);
        }
    }
}
