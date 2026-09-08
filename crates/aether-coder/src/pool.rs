//! Zero-allocation contiguous symbol memory pool.

#[derive(Debug)]
pub struct SymbolPool {
    capacity: usize,
    symbol_size: usize,
    storage: Vec<u8>,
    free_indices: Vec<usize>,
}

impl SymbolPool {
    /// Pre-allocates a pool of `capacity` symbols, each of `symbol_size` bytes.
    pub fn new(capacity: usize, symbol_size: usize) -> Self {
        let storage = vec![0u8; capacity * symbol_size];
        let mut free_indices = Vec::with_capacity(capacity);
        for i in (0..capacity).rev() {
            free_indices.push(i);
        }

        Self {
            capacity,
            symbol_size,
            storage,
            free_indices,
        }
    }

    /// Acquires a slot index from the pool without runtime allocations.
    pub fn acquire(&mut self) -> Option<usize> {
        self.free_indices.pop()
    }

    /// Returns a slot index back to the pool.
    pub fn release(&mut self, slot: usize) {
        assert!(slot < self.capacity);
        self.free_indices.push(slot);
    }

    /// Returns a slice to the symbol data at `slot`.
    pub fn get(&self, slot: usize) -> &[u8] {
        let start = slot * self.symbol_size;
        &self.storage[start..start + self.symbol_size]
    }

    /// Returns a mutable slice to the symbol data at `slot`.
    pub fn get_mut(&mut self, slot: usize) -> &mut [u8] {
        let start = slot * self.symbol_size;
        &mut self.storage[start..start + self.symbol_size]
    }

    pub fn symbol_size(&self) -> usize {
        self.symbol_size
    }

    pub fn available(&self) -> usize {
        self.free_indices.len()
    }
}
