mod activation;
mod layers;
mod model_info;
mod data_set;
mod optimizer;

use layers::{InputLayer, ConnectedGenericLayer, Layer};
use activation::ActivationFunction;
use model_info::ModelInformation;
use data_set::{DatasetLoader, Dataset};

struct LinearActivation {

}
impl ActivationFunction for LinearActivation {
    fn activate(f_in : f32) -> f32 {
        f_in
    }
}

fn main() {
    let mut input = [0.0; 64];
    let info = ModelInformation::new(1.0, 0.9);
    let data = Dataset::new(FolderLoader::new("data"), 0.8);


    let loss_fn = |out, expected| {out - expected};

    let mut input_layer = InputLayer::new(&input);
    let mut layer1: ConnectedGenericLayer<_, LinearActivation, 128, 64> = ConnectedGenericLayer::new(&mut input_layer);
    let mut output_layer : ConnectedGenericLayer<_, LinearActivation, 1, 128> = ConnectedGenericLayer::new(&mut layer1);




}
