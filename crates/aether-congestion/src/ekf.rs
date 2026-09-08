//! Extended Kalman Filter (EKF) tracking queuing delay and delay gradient.

#[derive(Clone, Debug)]
pub struct DelayKalmanFilter {
    // State: [queuing_delay, delay_gradient]
    delay: f64,
    gradient: f64,

    // Error covariance matrix P (2x2)
    p00: f64,
    p01: f64,
    p10: f64,
    p11: f64,

    // Process noise variance
    q_delay: f64,
    q_gradient: f64,

    // Measurement noise variance
    r_measure: f64,

    min_rtt_sec: f64,
    last_update_sec: f64,
}

impl DelayKalmanFilter {
    pub fn new(initial_rtt_sec: f64) -> Self {
        Self {
            delay: 0.0,
            gradient: 0.0,
            p00: 0.01,
            p01: 0.0,
            p10: 0.0,
            p11: 0.01,
            q_delay: 1e-4,
            q_gradient: 1e-3,
            r_measure: 1e-4,
            min_rtt_sec: initial_rtt_sec.max(0.001),
            last_update_sec: 0.0,
        }
    }

    pub fn min_rtt(&self) -> f64 {
        self.min_rtt_sec
    }

    pub fn delay(&self) -> f64 {
        self.delay
    }

    pub fn gradient(&self) -> f64 {
        self.gradient
    }

    /// Ingests a new RTT measurement at time `now_sec`.
    pub fn update(&mut self, rtt_sec: f64, now_sec: f64) {
        if rtt_sec < self.min_rtt_sec {
            self.min_rtt_sec = rtt_sec;
        }

        let dt = (now_sec - self.last_update_sec).clamp(0.001, 1.0);
        self.last_update_sec = now_sec;

        // 1. Predict state: x = F * x
        // delay_new = delay + dt * gradient
        let pred_delay = self.delay + dt * self.gradient;
        let pred_gradient = self.gradient;

        // 2. Predict covariance: P = F * P * F^T + Q
        // F = [1, dt; 0, 1]
        let f_p00 = self.p00 + dt * self.p10;
        let f_p01 = self.p01 + dt * self.p11;
        let f_p10 = self.p10;
        let f_p11 = self.p11;

        let pred_p00 = f_p00 + dt * f_p01 + self.q_delay;
        let pred_p01 = f_p01;
        let pred_p10 = f_p10 + dt * f_p11;
        let pred_p11 = f_p11 + self.q_gradient;

        // 3. Measurement innovation: z = observed queuing delay
        let z = (rtt_sec - self.min_rtt_sec).max(0.0);
        let y = z - pred_delay; // H = [1, 0]

        // 4. Innovation covariance: S = H * P * H^T + R = pred_p00 + R
        let s = pred_p00 + self.r_measure;

        // 5. Kalman gain: K = P * H^T / S
        let k0 = pred_p00 / s;
        let k1 = pred_p10 / s;

        // 6. State update: x = x_pred + K * y
        self.delay = pred_delay + k0 * y;
        self.gradient = pred_gradient + k1 * y;

        // 7. Covariance update: P = (I - K * H) * P_pred
        self.p00 = (1.0 - k0) * pred_p00;
        self.p01 = (1.0 - k0) * pred_p01;
        self.p10 = pred_p10 - k1 * pred_p00;
        self.p11 = pred_p11 - k1 * pred_p01;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ekf_tracking_gradient() {
        let mut ekf = DelayKalmanFilter::new(0.040); // 40ms min RTT

        // Feed increasing RTT to simulate queue buildup
        let mut time = 0.0;
        for i in 0..10 {
            time += 0.01; // every 10ms
            let observed_rtt = 0.040 + (i as f64) * 0.005; // climbing by 5ms
            ekf.update(observed_rtt, time);
        }

        // Gradient must be strictly positive
        assert!(
            ekf.gradient() > 0.0,
            "Expected positive gradient, got {}",
            ekf.gradient()
        );
        assert!(ekf.delay() > 0.0);
    }
}
