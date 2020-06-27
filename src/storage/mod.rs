use sys_info;
use std::error::Error;

const INNER_ALLOC_PERCENTAGE: f64 = 0.8;

pub struct Store {
    pub inner: Vec<u8>,
    pub offset: u64,
    insert_position: usize
}

impl Store {
    fn get_current_offset() -> u64 {
        let timestamp_as_u64 = std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis() as u64;
        let mut timestamp = timestamp_as_u64.to_le_bytes();
        for i in 0..4 { timestamp[i] = 0 }

        u64::from_le_bytes(timestamp)
    }
    pub fn new() -> Store {
        let memory = sys_info::mem_info().unwrap();
        let inner_size = ((memory.avail + memory.swap_free) as f64 * INNER_ALLOC_PERCENTAGE) as usize;
        println!("Total memory: {} MB, Available memory: {} MB, Available swap: {} MB, Percentage for Store: {}, Reserved: {} MB", memory.total / 1024, memory.avail / 1024, memory.swap_free / 1024,  INNER_ALLOC_PERCENTAGE, inner_size / 1024);

        let offset = Store::get_current_offset();
        println!("Initial memory offset: {}", offset);

        Store {
            inner: vec![0u8; inner_size * 1024],
            offset,
            insert_position: 0
        }
    }
    pub fn append(&mut self, message: &[u8]) -> Result<usize, Box<dyn Error>> {
        if self.insert_position + message.len() > self.inner.capacity() {
            let first_slice_size = self.inner.capacity() - self.insert_position;
            let second_slice_size = message.len() - first_slice_size;

            {
                let first_slice: &mut [u8] = &mut self.inner[self.insert_position..self.insert_position + first_slice_size];
                first_slice.copy_from_slice(&message[0..first_slice_size]);
            }

            {
                let second_slice: &mut [u8] = &mut self.inner[0..second_slice_size - 1];
                second_slice.copy_from_slice(&message[first_slice_size..first_slice_size + second_slice_size - 1]);
            }

            self.insert_position = (self.insert_position + message.len()) % self.inner.capacity();
            return Ok(self.insert_position)
        }

        let slice: &mut [u8] = &mut self.inner[self.insert_position..self.insert_position + message.len()];
        slice.copy_from_slice(message);


        self.insert_position = self.insert_position + message.len();
        Ok(self.insert_position)
    }
    pub fn pipe(self, offset: u64) {
        
    }
}