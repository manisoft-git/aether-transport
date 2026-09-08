//! `aether-coder`: Concatenated Fulcrum Network Coder and symbol pool management.

pub mod fulcrum;
pub mod pool;
pub mod sliding_window;

pub use fulcrum::{CoderError, FulcrumDecoder, FulcrumEncoder};
pub use pool::SymbolPool;
pub use sliding_window::SlidingWindowCoder;
