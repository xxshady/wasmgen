use std::collections::HashMap;

// TEST
const SIMULTANEOUS_IDS: usize = 4 as usize;
// const SIMULTANEOUS_IDS: usize = u8::MAX as usize;

pub(crate) struct MemoryBufferManager {
    free_ids: [bool; SIMULTANEOUS_IDS],
    buffers: HashMap<u8, Vec<u8>>,
}

impl MemoryBufferManager {
    pub(crate) fn new() -> Self {
        Self {
            free_ids: [true; SIMULTANEOUS_IDS],
            buffers: HashMap::with_capacity(u8::MAX.into()),
        }
    }

    pub(crate) fn alloc(&mut self, size: u16) -> u8 {
        let free_id = self.free_ids.iter().enumerate().find(|(_, free)| **free);
        let Some((index, _)) = free_id else {
            return 0;
        };
        self.free_ids[index] = false;

        let id = (index as u8) + 1;
        self.buffers.insert(id, vec![0; size as usize]);

        println!(
            "after alloc id: {id} free_ids: {:?} buffers: {:?}",
            self.free_ids, self.buffers
        );
        id
    }

    pub(crate) fn dealloc(&mut self, id: u8) -> Vec<u8> {
        let index = (id - 1) as usize;
        self.free_ids[index] = true;
        let content = self.buffers.remove(&id).unwrap();

        println!(
            "after dealloc id: {id} free_ids: {:?} buffers: {:?}",
            self.free_ids, self.buffers
        );

        content
    }

    pub(crate) fn read(&mut self, id: u8) -> Vec<u8> {
        self.dealloc(id)
    }

    pub(crate) fn get_mut_ptr(&mut self, id: u8) -> *mut u8 {
        self.buffers.get_mut(&id).unwrap().as_mut_ptr()
    }
}
