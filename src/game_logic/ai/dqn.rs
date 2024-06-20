use tch::{nn, nn::Module, Device, Tensor};

pub struct DQN {
    model: nn::Sequential,
    device: Device,
}

impl DQN {
    fn new(vs: &nn::Path, input_size: i64, output_size: i64, device: Device) -> Self {
        let model = nn::seq()
            .add(nn::linear(vs, input_size, 128, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(vs, 128, output_size, Default::default()));
        DQN { model, device }
    }

    fn forward(&self, x: &Tensor) -> Tensor {
        self.model.forward(x)
    }
}
