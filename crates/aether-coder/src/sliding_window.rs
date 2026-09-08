//! Sliding-window generation manager for continuous transport stream coding.

use crate::fulcrum::{FulcrumDecoder, FulcrumEncoder};

#[derive(Debug)]
pub struct SlidingWindowCoder {
    k: usize,
    r: usize,
    symbol_size: usize,
    current_window_id: u32,
    encoder: Option<FulcrumEncoder>,
    decoder: Option<FulcrumDecoder>,
}

impl SlidingWindowCoder {
    pub fn new(k: usize, r: usize, symbol_size: usize) -> Self {
        Self {
            k,
            r,
            symbol_size,
            current_window_id: 1,
            encoder: None,
            decoder: Some(FulcrumDecoder::new(k, r, symbol_size)),
        }
    }

    pub fn window_id(&self) -> u32 {
        self.current_window_id
    }

    pub fn load_source_block(&mut self, window_id: u32, symbols: Vec<Vec<u8>>) {
        self.current_window_id = window_id;
        self.encoder = Some(FulcrumEncoder::new(symbols, self.r));
    }

    pub fn encoder(&self) -> Option<&FulcrumEncoder> {
        self.encoder.as_ref()
    }

    pub fn decoder_mut(&mut self) -> Option<&mut FulcrumDecoder> {
        self.decoder.as_mut()
    }

    pub fn advance_decoder_window(&mut self, next_window_id: u32) {
        self.current_window_id = next_window_id;
        self.decoder = Some(FulcrumDecoder::new(self.k, self.r, self.symbol_size));
    }
}
