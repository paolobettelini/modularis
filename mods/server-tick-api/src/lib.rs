use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Copy)]
pub struct ServerTickRate {
    pub target_tps: f64,
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct ServerTickMetrics {
    pub measured_tps: f64,
    sample_ticks: u64,
    sample_seconds: f64,
}

impl ServerTickMetrics {
    pub fn new(initial_tps: f64) -> Self {
        Self {
            measured_tps: initial_tps,
            sample_ticks: 0,
            sample_seconds: 0.0,
        }
    }

    pub fn record_tick(&mut self, delta_seconds: f64) {
        if !delta_seconds.is_finite() || delta_seconds <= 0.0 {
            return;
        }
        self.sample_ticks += 1;
        self.sample_seconds += delta_seconds;
        if self.sample_seconds >= 1.0 {
            self.measured_tps = self.sample_ticks as f64 / self.sample_seconds;
            self.sample_ticks = 0;
            self.sample_seconds = 0.0;
        }
    }
}

pub trait ServerTickApi: Send + Sync + 'static {
    fn ticks_per_second() -> f64;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn measures_twenty_ticks_per_second() {
        let mut metrics = ServerTickMetrics::new(0.0);
        for _ in 0..20 {
            metrics.record_tick(0.05);
        }
        assert!((metrics.measured_tps - 20.0).abs() < 0.001);
    }
}
