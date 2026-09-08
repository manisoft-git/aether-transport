//! Bifurcated Congestion Arbiter and Microsecond Pacing Controller.

use crate::ekf::DelayKalmanFilter;
use std::time::{Duration, Instant};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LossClassification {
    PhysicalRandomDrop,
    CongestiveBufferbloat,
}

#[derive(Debug)]
pub struct CongestionArbiter {
    ekf: DelayKalmanFilter,
    base_pacing_interval: Duration,
    current_pacing_interval: Duration,
    redundancy_ratio: f64, // repair packets / source packets (e.g. 0.20 = 20%)
    start_time: Instant,
}

impl CongestionArbiter {
    pub fn new(initial_rtt: Duration, initial_bitrate_mbps: f64) -> Self {
        let packet_bits = 1400.0 * 8.0;
        let packets_per_sec = (initial_bitrate_mbps * 1e6) / packet_bits;
        let interval_micros = (1e6 / packets_per_sec).max(10.0) as u64;

        Self {
            ekf: DelayKalmanFilter::new(initial_rtt.as_secs_f64()),
            base_pacing_interval: Duration::from_micros(interval_micros),
            current_pacing_interval: Duration::from_micros(interval_micros),
            redundancy_ratio: 0.10, // Default 10% coding redundancy
            start_time: Instant::now(),
        }
    }

    pub fn on_rtt_sample(&mut self, rtt: Duration) {
        let now_sec = self.start_time.elapsed().as_secs_f64();
        self.ekf.update(rtt.as_secs_f64(), now_sec);

        // If queue gradient is positive and persistent, moderate pacing rate
        if self.ekf.gradient() > 0.05 {
            // Smoothly back off pacing by 5%
            let new_micros = (self.current_pacing_interval.as_micros() as f64 * 1.05) as u64;
            self.current_pacing_interval = Duration::from_micros(new_micros.min(100_000));
        } else if self.ekf.gradient() <= 0.0 && self.current_pacing_interval > self.base_pacing_interval {
            // Gradually recover towards base pacing rate
            let new_micros = (self.current_pacing_interval.as_micros() as f64 * 0.98) as u64;
            self.current_pacing_interval = Duration::from_micros(new_micros.max(self.base_pacing_interval.as_micros() as u64));
        }
    }

    /// Handles a detected packet drop event.
    pub fn on_packet_loss(&mut self) -> LossClassification {
        if self.ekf.gradient() <= 0.0 {
            // Gradient is non-positive: Queue is not growing.
            // Classification: Physical loss (rain fade / RF attenuation).
            // Action: Maintain pacing rate, increase coding redundancy.
            self.redundancy_ratio = (self.redundancy_ratio + 0.05).min(0.80);
            LossClassification::PhysicalRandomDrop
        } else {
            // Gradient is positive: Bufferbloat / queue saturation.
            // Action: Throttle pacing rate.
            let new_micros = (self.current_pacing_interval.as_micros() as f64 * 1.20) as u64;
            self.current_pacing_interval = Duration::from_micros(new_micros.min(200_000));
            LossClassification::CongestiveBufferbloat
        }
    }

    pub fn pacing_interval(&self) -> Duration {
        self.current_pacing_interval
    }

    pub fn redundancy_ratio(&self) -> f64 {
        self.redundancy_ratio
    }

    pub fn set_redundancy_ratio(&mut self, ratio: f64) {
        self.redundancy_ratio = ratio.clamp(0.0, 1.0);
    }
}
