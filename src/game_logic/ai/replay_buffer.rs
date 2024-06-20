use crate::game_logic::entities::creature::CreatureAction;

use super::CreatureState;

pub struct ReplayBuffer {
    buffer: Vec<(CreatureState, CreatureAction, f32, CreatureState)>,
    capacity: usize,
}

impl ReplayBuffer {
    fn new(capacity: usize) -> Self {
        ReplayBuffer {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }

    fn add(&mut self, experience: (CreatureState, CreatureAction, f32, CreatureState)) {
        if self.buffer.len() >= self.capacity {
            self.buffer.remove(0);
        }
        self.buffer.push(experience);
    }

    fn sample(
        &self,
        batch_size: usize,
    ) -> Vec<(CreatureState, CreatureAction, f32, CreatureState)> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.buffer
            .choose_multiple(&mut rng, batch_size)
            .cloned()
            .collect()
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }
}
