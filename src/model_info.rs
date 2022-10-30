#[derive(Copy, Clone)]
pub struct ModelInformation {
    //implementation detail
    learning_rate_initial : f32,
    learning_rate_current : f32,
    learning_rate_rate: f32
}
impl ModelInformation {
    pub fn new(lr : f32, dlr : f32) -> ModelInformation {
        ModelInformation { learning_rate_initial: lr, learning_rate_current: lr, learning_rate_rate: dlr }
    }
    pub fn update(self) -> ModelInformation {
        ModelInformation {
            learning_rate_initial: self.learning_rate_initial,
            learning_rate_current: self.learning_rate_current * self.learning_rate_rate,
            learning_rate_rate: self.learning_rate_rate
        }
    }
    pub fn get_lr(&self)->f32 {
        self.learning_rate_current
    }
}