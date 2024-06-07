use std::time::{SystemTime, UNIX_EPOCH};

use bevy::transform::{components::Transform, TransformBundle};

use crate::config::{BOUNDS_X, BOUNDS_Y};

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

fn generate_random_float_in_range(min: f32, max: f32) -> f32 {
    // Using the current time as a seed
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let mut lcg = LCG::new(seed);
    let random_fraction = lcg.next_float();
    min + random_fraction * (max - min)
}

pub fn in_bounds_rng() -> TransformBundle {
    let x = generate_random_float_in_range(0.0, BOUNDS_X);
    let y = generate_random_float_in_range(0.0, BOUNDS_Y);
    TransformBundle {
        local: Transform::from_xyz(x, y, 0.0),
        ..Default::default()
    }
}
