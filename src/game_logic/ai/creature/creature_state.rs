pub struct CreatureState {
    pub distance_to_fruit: f32,
    pub creature_pos_x: f32,
    pub creature_pos_y: f32,
    pub creature_rot: f32,
}

impl CreatureState {
    pub fn to_vec(&self) -> Vec<f32> {
        vec![
            self.distance_to_fruit,
            self.creature_pos_x,
            self.creature_pos_y,
            self.creature_rot,
        ]
    }
}
