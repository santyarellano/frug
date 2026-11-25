/// Fixed-timestep accumulator.
///
/// Feed it the real elapsed time each frame with [`FixedTimestep::advance`]; it returns how
/// many fixed updates to run. At most `max_steps` are returned per frame so a long stall
/// (window drag, breakpoint) does not cause a spiral of catch-up updates.
#[derive(Debug, Clone)]
pub struct FixedTimestep {
    step: f64,
    accumulator: f64,
    max_steps: u32,
}

impl FixedTimestep {
    /// `steps_per_second` fixed updates per second (e.g. 60).
    pub fn new(steps_per_second: u32) -> Self {
        assert!(steps_per_second > 0, "steps_per_second must be > 0");
        Self {
            step: 1.0 / steps_per_second as f64,
            accumulator: 0.0,
            max_steps: 5,
        }
    }

    pub fn with_max_steps(mut self, max_steps: u32) -> Self {
        self.max_steps = max_steps.max(1);
        self
    }

    /// Length of one step in seconds.
    pub fn step_seconds(&self) -> f64 {
        self.step
    }

    /// Add `elapsed` seconds and return the number of whole steps to simulate.
    pub fn advance(&mut self, elapsed: f64) -> u32 {
        self.accumulator += elapsed.max(0.0);
        let mut steps = 0;
        while self.accumulator >= self.step && steps < self.max_steps {
            self.accumulator -= self.step;
            steps += 1;
        }
        if steps == self.max_steps && self.accumulator >= self.step {
            // Drop the backlog we refused to simulate.
            self.accumulator %= self.step;
        }
        steps
    }

    /// Fraction (0..1) of a step left in the accumulator; useful for interpolation.
    pub fn alpha(&self) -> f64 {
        self.accumulator / self.step
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_steps() {
        let mut t = FixedTimestep::new(60);
        assert_eq!(t.advance(1.0 / 60.0), 1);
        assert_eq!(t.advance(2.0 / 60.0), 2);
    }

    #[test]
    fn accumulates_partial_frames() {
        let mut t = FixedTimestep::new(60);
        // 120 Hz display: every other frame runs an update.
        let total: u32 = (0..120).map(|_| t.advance(1.0 / 120.0)).sum();
        assert!((59..=60).contains(&total), "got {total}");
    }

    #[test]
    fn clamps_after_stall() {
        let mut t = FixedTimestep::new(60).with_max_steps(5);
        assert_eq!(t.advance(10.0), 5);
        assert!(t.alpha() < 1.0);
        assert_eq!(t.advance(0.0), 0);
    }

    #[test]
    fn ignores_negative_time() {
        let mut t = FixedTimestep::new(60);
        assert_eq!(t.advance(-1.0), 0);
        assert_eq!(t.alpha(), 0.0);
    }
}
