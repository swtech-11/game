use std::time::{SystemTime, UNIX_EPOCH};

struct LCG {
    state: u64,
}

impl LCG {
    fn new(seed: u64) -> Self {
        LCG { state: seed }
    }

    fn next(&mut self) -> u64 {
        // Constants for the LCG algorithm
        const A: u64 = 6364136223846793005;
        const C: u64 = 1;

        self.state = self.state.wrapping_mul(A).wrapping_add(C);
        self.state
    }

    fn next_float(&mut self) -> f32 {
        const MAX_U64: f32 = u64::MAX as f32;
        self.next() as f32 / MAX_U64
    }
}

pub fn rand_float(min: f32, max: f32) -> f32 {
    // Using the current time as a seed with nanoseconds included
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let seed = duration.as_secs() ^ duration.subsec_nanos() as u64;

    let mut lcg = LCG::new(seed);
    let random_fraction = lcg.next_float();
    min + random_fraction * (max - min)
}
