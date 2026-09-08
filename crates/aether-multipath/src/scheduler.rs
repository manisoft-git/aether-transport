//! Delay-aware asymmetric multipath packet scheduler.

use aether_wire::{FLAG_MULTIPATH_PRIMARY, FLAG_MULTIPATH_SECONDARY};
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct NetworkPath {
    pub id: u8,
    pub name: String,
    pub rtt: Duration,
    pub capacity_mbps: f64,
    pub loss_rate: f64,
    pub in_flight: usize,
}

impl NetworkPath {
    pub fn new(id: u8, name: &str, rtt: Duration, capacity_mbps: f64) -> Self {
        Self {
            id,
            name: name.to_string(),
            rtt,
            capacity_mbps,
            loss_rate: 0.05,
            in_flight: 0,
        }
    }

    /// Score combining capacity and latency.
    pub fn efficiency_score(&self) -> f64 {
        let rtt_sec = self.rtt.as_secs_f64().max(0.005);
        self.capacity_mbps / rtt_sec
    }
}

#[derive(Debug)]
pub struct MultipathScheduler {
    pub primary: NetworkPath,
    pub secondary: NetworkPath,
    packet_counter: usize,
}

impl MultipathScheduler {
    pub fn new(primary: NetworkPath, secondary: NetworkPath) -> Self {
        Self {
            primary,
            secondary,
            packet_counter: 0,
        }
    }

    /// Determines which path should carry the next symbol.
    /// Returns (target_path_id, wire_flag).
    pub fn schedule_next(&mut self, is_systematic: bool) -> (u8, u16) {
        self.packet_counter += 1;

        // If systematic, prioritize lower latency path
        if is_systematic {
            if self.primary.rtt <= self.secondary.rtt {
                (self.primary.id, FLAG_MULTIPATH_PRIMARY)
            } else {
                (self.secondary.id, FLAG_MULTIPATH_SECONDARY)
            }
        } else {
            // Repair packets sprayed based on relative capacity
            let p_score = self.primary.efficiency_score();
            let s_score = self.secondary.efficiency_score();
            let total = p_score + s_score;

            let p_ratio = if total > 0.0 { p_score / total } else { 0.5 };
            let sample = (self.packet_counter % 100) as f64 / 100.0;

            if sample < p_ratio {
                (self.primary.id, FLAG_MULTIPATH_PRIMARY)
            } else {
                (self.secondary.id, FLAG_MULTIPATH_SECONDARY)
            }
        }
    }

    pub fn update_rtt(&mut self, path_id: u8, rtt: Duration) {
        if self.primary.id == path_id {
            self.primary.rtt = rtt;
        } else if self.secondary.id == path_id {
            self.secondary.rtt = rtt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multipath_scheduling_systematic_low_latency() {
        let p1 = NetworkPath::new(1, "MTN_4G", Duration::from_millis(60), 30.0);
        let p2 = NetworkPath::new(2, "Airtel_4G", Duration::from_millis(140), 20.0);

        let mut sched = MultipathScheduler::new(p1, p2);

        // Systematic packets must pick p1 (lower latency)
        let (path_id, flag) = sched.schedule_next(true);
        assert_eq!(path_id, 1);
        assert_eq!(flag, FLAG_MULTIPATH_PRIMARY);
    }
}
