//! `aether-telemetry`: Cross-layer physical RF modem telemetry interceptor.

pub mod rf;

pub use rf::{CellularTelemetry, TelemetryError};
