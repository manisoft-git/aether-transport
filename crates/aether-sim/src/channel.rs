//! Gilbert-Elliot 2-State Markov Channel Model.

use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ChannelState {
    Good,
    Bad,
}

#[derive(Clone, Debug)]
pub struct GilbertElliotChannel {
    state: ChannelState,
    p_good_to_bad: f64, // Transition prob G -> B
    p_bad_to_good: f64, // Transition prob B -> G
    loss_rate_good: f64,
    loss_rate_bad: f64,
    rtt_ms: f64,
    jitter_ms: f64,
}

impl GilbertElliotChannel {
    pub fn new(p: f64, q: f64, loss_good: f64, loss_bad: f64, rtt_ms: f64, jitter_ms: f64) -> Self {
        Self {
            state: ChannelState::Good,
            p_good_to_bad: p,
            p_bad_to_good: q,
            loss_rate_good: loss_good,
            loss_rate_bad: loss_bad,
            rtt_ms,
            jitter_ms,
        }
    }

    /// Simulates transit of one packet through the channel.
    /// Returns (dropped: bool, delay_ms: f64).
    pub fn transit<R: Rng>(&mut self, rng: &mut R) -> (bool, f64) {
        // 1. Update Markov state
        match self.state {
            ChannelState::Good => {
                if rng.gen::<f64>() < self.p_good_to_bad {
                    self.state = ChannelState::Bad;
                }
            }
            ChannelState::Bad => {
                if rng.gen::<f64>() < self.p_bad_to_good {
                    self.state = ChannelState::Good;
                }
            }
        }

        // 2. Sample loss based on current state
        let loss_threshold = match self.state {
            ChannelState::Good => self.loss_rate_good,
            ChannelState::Bad => self.loss_rate_bad,
        };

        let dropped = rng.gen::<f64>() < loss_threshold;

        // 3. Compute delay with jitter
        let jitter = (rng.gen::<f64>() * 2.0 - 1.0) * self.jitter_ms;
        let delay_ms = (self.rtt_ms + jitter).max(1.0);

        (dropped, delay_ms)
    }

    #[allow(dead_code)]
    pub fn state(&self) -> ChannelState {
        self.state
    }
}
