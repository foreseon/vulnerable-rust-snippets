use std::collections::HashMap;

struct StorageUnit {
    id: u32,
    data: Vec<u8>,
}

struct StorageManager {
    storage_units: HashMap<u32, StorageUnit>,
}

impl StorageManager {
    pub fn new() -> Self {
        StorageManager {
            storage_units: HashMap::new(),
        }
    }

    pub fn create_storage_unit(&mut self, id: u32) {
        self.storage_units.insert(id, StorageUnit { id, data: Vec::new() });
    }

    pub fn allocate(&mut self, id: u32, size: usize) {
        if let Some(storage_unit) = self.storage_units.get_mut(&id) {
            storage_unit.data = vec![0; size];
        }
    }

    pub fn print_storage_info(&self, id: u32) {
        if let Some(storage_unit) = self.storage_units.get(&id) {
            println!("Storage Unit {}: size {}", storage_unit.id, storage_unit.data.len());
        } else {
            println!("Storage Unit {} not found", id);
        }
    }
}

fn main() {
    let mut storage_manager = StorageManager::new();

    storage_manager.create_storage_unit(1);
    storage_manager.create_storage_unit(2);

    storage_manager.allocate(1, 1024);
    storage_manager.allocate(2, 2048);

    storage_manager.print_storage_info(1);
    storage_manager.print_storage_info(2);
}
