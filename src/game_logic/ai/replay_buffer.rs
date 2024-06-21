use rand::Rng;

struct ReplayBuffer {
    buffer: Vec<(Vec<f32>, usize, f32, Vec<f32>)>,
}

impl ReplayBuffer {
    fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    fn add(&mut self, experience: (Vec<f32>, usize, f32, Vec<f32>)) {
        self.buffer.push(experience);
        if self.buffer.len() > 1000 {
            self.buffer.remove(0);
        }
    }

    fn sample(&self, batch_size: usize) -> Vec<(Vec<f32>, usize, f32, Vec<f32>)> {
        let mut rng = rand::thread_rng();
        (0..batch_size)
            .map(|_| {
                let index = rng.gen_range(0..self.buffer.len());
                self.buffer[index].clone()
            })
            .collect()
    }
}
