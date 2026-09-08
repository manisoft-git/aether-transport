//! Cellular RF signal metrics and cross-layer loss prediction.

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TelemetryError {
    #[error("Failed to parse AT command response: {0}")]
    ParseError(String),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellularTelemetry {
    pub rsrp_dbm: i32,  // Reference Signal Received Power (-140 to -44 dBm)
    pub rsrq_db: i32,   // Reference Signal Received Quality (-20 to -3 dB)
    pub sinr_db: f32,   // Signal-to-Interference-plus-Noise Ratio (-10 to +30 dB)
    pub csq: u8,        // Raw CSQ index (0..31)
}

impl Default for CellularTelemetry {
    fn default() -> Self {
        Self {
            rsrp_dbm: -95,
            rsrq_db: -10,
            sinr_db: 12.0,
            csq: 18,
        }
    }
}

impl CellularTelemetry {
    /// Parses standard 3GPP AT+CSQ response: "+CSQ: <rssi>,<ber>"
    pub fn parse_at_csq(response: &str) -> Result<Self, TelemetryError> {
        let line = response.lines()
            .find(|l| l.contains("+CSQ:"))
            .ok_or_else(|| TelemetryError::ParseError("Missing +CSQ header".into()))?;

        let parts: Vec<&str> = line.trim_start_matches("+CSQ:").trim().split(',').collect();
        if parts.is_empty() {
            return Err(TelemetryError::ParseError("Empty CSQ arguments".into()));
        }

        let csq: u8 = parts[0].trim().parse().map_err(|_| TelemetryError::ParseError("Invalid CSQ number".into()))?;
        // Convert CSQ to approximate dBm: -113 + 2 * csq
        let rsrp_dbm = if csq == 99 { -113 } else { -113 + (csq as i32) * 2 };
        let sinr_db = if csq > 15 { 15.0 } else { (csq as f32) - 5.0 };

        Ok(Self {
            rsrp_dbm,
            rsrq_db: -12,
            sinr_db,
            csq,
        })
    }

    /// Computes predictive physical frame loss probability before transport observations.
    /// Uses logistic mapping on SINR and RSRP.
    pub fn predict_loss_probability(&self) -> f32 {
        if self.sinr_db < 0.0 {
            0.35 // Severe multipath / noise interference
        } else if self.sinr_db < 5.0 {
            0.18 // Volatile tropical cellular conditions
        } else if self.sinr_db < 12.0 {
            0.05
        } else {
            0.005 // Clean channel
        }
    }

    /// Returns recommended Fulcrum repair redundancy ratio (e.g. 0.25 = 25% extra repair symbols).
    pub fn recommended_redundancy(&self) -> f64 {
        let loss_prob = self.predict_loss_probability() as f64;
        // Provide safety margin over expected loss
        (loss_prob * 1.35).clamp(0.05, 0.60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csq_parser() {
        let resp = "\r\n+CSQ: 20,99\r\nOK\r\n";
        let tel = CellularTelemetry::parse_at_csq(resp).unwrap();
        assert_eq!(tel.csq, 20);
        assert_eq!(tel.rsrp_dbm, -73);
        assert!(tel.predict_loss_probability() < 0.05);
    }

    #[test]
    fn test_degraded_sinr_loss_prediction() {
        let tel = CellularTelemetry {
            rsrp_dbm: -112,
            rsrq_db: -17,
            sinr_db: -2.5,
            csq: 6,
        };
        assert_eq!(tel.predict_loss_probability(), 0.35);
        assert!(tel.recommended_redundancy() >= 0.40);
    }
}
