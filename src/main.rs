mod activation;
mod layers;
mod model_info;
mod data_set;
mod optimizer;
mod data_importer;

use layers::{InputLayer, ConnectedGenericLayer, Layer};
use activation::ActivationFunction;
use model_info::ModelInformation;
use data_set::{Dataset, FileSystemLoader, FileError, Datum};

struct LinearActivation {

}
impl ActivationFunction for LinearActivation {
    fn activate(f_in : f32) -> f32 {
        f_in
    }
}
#[derive(Copy, Clone)]
enum Classification{
    Triangle, NotTriangle
}
struct MatrixData {
    data : [f32; 64],
    classification: Classification
}
impl Datum<f32, Classification, 64> for MatrixData {
    fn get_data(&self) -> [f32; 64] {
        self.data
    }

    fn from(data : Vec<u8>) -> Option<Self> {
        if data.len() < 64 * 4 + 1{
            Option::None
        }else {
            let v = data.to_vec();
            let mut data = [0.0; 64 ];
            for i in 0..64{
                let idx = i * 4;
                let arr = [v[idx], v[idx + 1], v[idx+2], v[idx+3]];
                data[i] = f32::from_le_bytes(arr);
            }
            let class_id = v.last().unwrap();
            let class_enum = if class_id.clone() > 0 {
                Classification::NotTriangle
            }else {
                Classification::Triangle
            };
            Some(MatrixData{data, classification: class_enum})
        }
    }

    fn seed(&self, receiver : &mut [f32; 64]) {
        for i in 0..64 {
            receiver[i] = self.data[i];
        }
    }

    fn get_classification(&self) -> Classification {
        self.classification
    }
}

fn main() {
    let mut input = [0.0; 64];
    let info = ModelInformation::new(1.0, 0.9);
    let mut data = Dataset::<MatrixData, _, _,  64>::new(FileSystemLoader::new("data").unwrap(), 0.8);

    let seed = data.get_validation().unwrap();
    seed.seed(&mut input);
    let seed = data.get_training().unwrap();
    seed.seed(&mut input);

//    let loss_fn = |out, expected| {out - expected};

    let mut input_layer = InputLayer::new(&input);
    let mut layer1: ConnectedGenericLayer<_, LinearActivation, 128, 64> = ConnectedGenericLayer::new(&mut input_layer);
    let mut output_layer : ConnectedGenericLayer<_, LinearActivation, 1, 128> = ConnectedGenericLayer::new(&mut layer1);




}
