mod activation;
mod data_importer;
mod data_set;
mod layers;
mod model_info;
mod optimizer;

use activation::ActivationFunction;
use data_set::{Dataset, Datum, FileSystemLoader, DatasetLoader};
use layers::{ConnectedGenericLayer, InputLayer, Layer};
use model_info::ModelInformation;

use std::cell::RefCell;
use std::fs::{create_dir, metadata, File};
use std::io::Write;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
struct LinearActivation {}
impl ActivationFunction for LinearActivation {
    fn activate(f_in: f32) -> f32 {
        if f_in > 1000.0 {
            1000.0
        } else if f_in < -1000.0 {
            -1000.0
        }else {
            f_in
        }
    }
}
#[derive(Clone)]
struct ActualLinearActivation {}
impl ActivationFunction for ActualLinearActivation {
    fn activate(f_in : f32) -> f32 {
        f_in
    }
}

#[derive(Copy, Clone)]
struct MatrixData {
    data: [f32; 1024],
}
impl Datum<1024> for MatrixData {
    type DataType = f32;
    type ReceiverType = Rc<RefCell<[f32; 1024]>>;
    fn from(data: Vec<u8>) -> Option<Self> {
        if data.len() < 1024 {
            Option::None
        } else {
            let v = data.to_vec();
            let mut data = [0.0; 1024];
            for i in 0..1024 {
                data[i] = v[i] as f32 / 256.0;
            }
            Some(MatrixData { data })
        }
    }

    fn seed(&self, receiver: Rc<RefCell<[f32; 1024]>>) {
        for i in 0..1024 {
            receiver.borrow_mut()[i] = self.data[i];
        }
    }
}
use rand::prelude::*;
fn main() {
    let mut info = ModelInformation::new(1.0, 0.98);
    let data = Dataset::<MatrixData, _, 1024>::new(
        FileSystemLoader::new("./dataset/dataset.json").unwrap(),
        0.5,
    );
    let in_cell = Rc::new(RefCell::new([0.0; 1024]));


    let input_layer = Rc::new(RefCell::new(InputLayer::new(in_cell.clone())));
    let layer1: Rc<RefCell<ConnectedGenericLayer<_, LinearActivation, 128, 1024>>> =
        Rc::new(RefCell::new(ConnectedGenericLayer::new(input_layer)));
    let mut output_layer: ConnectedGenericLayer<_, ActualLinearActivation, 1, 128> =
        ConnectedGenericLayer::new(layer1);

    let mut log = vec![];
    let loss_fn = |out, expected| out - expected;
    let mut fitness = 10000.0;
    let mut rng = thread_rng();
    let mut generation_number =0;

    while fitness > 0.001{

        let mut bif1 = output_layer.clone();
        let mut bif2 = output_layer.clone();
        bif1.update(info);
        bif2.update(info);
        info = info.update();

        let mut f1 = train(&mut bif1, in_cell.clone(), &data, &loss_fn);
        let mut f2 = train(&mut bif2, in_cell.clone(), &data, &loss_fn);

        let chose_best = if rng.gen_range(0.0..1.0) < 0.1 {
            std::mem::swap(&mut f1, &mut f2);
            false
        } else {true};
        let best = if f1 < f2 {
            fitness = f1;
            bif1
        } else {
            fitness = f2;
            bif2
        };
        output_layer = best;

        log.push(LogStructure {
            best_chosen: chose_best,
            loss: fitness,
            learning_rate: info.get_lr(),
            generation: generation_number
        });
        println!("Generation {generation_number}, Loss: {fitness}");
        generation_number += 1;
    }
    if !metadata("./logs").is_ok() {        
        create_dir("./logs").unwrap()
    }

    let log_path = get_log_path();
    let mut file = File::create(log_path).unwrap();
    for item in log {
        writeln!(&mut file, "{} : L={}, R={}, best={}", item.generation, item.loss, item.learning_rate, item.best_chosen).unwrap();
    }

}



fn get_log_path() -> PathBuf {
    let mut cur_id = 0;
    let mut p = PathBuf::new();
    p.push("./logs");
    loop {
        let a = format!("log_{cur_id}.log");
        p.push(a);
        if !p.exists() {
            break;
        }
        else {
            cur_id += 1;
            p.pop();
        }
    }
    p

}

fn train<L : Layer, D : Datum<SIZE, ReceiverType = Rc<RefCell<[f32; SIZE]>>>, Ld : DatasetLoader<D, SIZE>, const SIZE : usize>(layer : &mut L, in_cell : Rc<RefCell<[f32; SIZE]>>, data :& Dataset<D, Ld, SIZE>, loss: &dyn Fn(f32, f32) -> f32) -> f32 {
    let mut cumavg = 0.0;
    let mut amt = 0;

    while data.has_training() {
        let seed = data.get_training().unwrap();
        let class = seed.get_class();
        seed.get_data().seed(in_cell.clone());

        layer.calculate_state();
        let output = layer.get_value(0).unwrap();
        cumavg += loss(output, class as f32);
        amt += 1;
        // println!("Class {class}, Output: {output}")
    }
    data.reset();
    (cumavg / (amt as f32)).abs()
}

struct LogStructure {
    best_chosen: bool,
    loss: f32,
    learning_rate: f32,
    generation : usize
}
