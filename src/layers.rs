use crate::activation::ActivationFunction;
use crate::model_info::ModelInformation;
use crate::optimizer::Optimizer;
use std::marker::PhantomData;

use rand::prelude::*;

pub trait Layer {
    fn calculate_state(&mut self);
    fn get_value(&self, idx :usize) -> Option<f32>;
    // TODO: figure out how to pass the data needed to update smartly
    fn update(&mut self, info : ModelInformation);
}

pub struct InputLayer <'a, const SIZE: usize>{
    data : [Option<&'a f32>; SIZE]
}
impl <'a, const SIZE: usize> Layer for InputLayer<'a, SIZE> {
    fn calculate_state(&mut self) {
        //do nothing
    }
    fn get_value(&self, idx: usize) -> Option<f32> {
        if idx > SIZE {
            Option::None
        }else {
            Option::Some(self.data[idx].unwrap().clone())
        }
    }

    fn update(&mut self, info: ModelInformation) {
        // do nothing (terminal layer)
    }
}
impl <'a, const SIZE: usize> InputLayer<'a, SIZE> {
    pub fn new(data_in : &'a [f32; SIZE]) -> InputLayer<SIZE>{
        let mut data_out = [Option::None; SIZE];
        for i in 0..SIZE {
            data_out[i] = Option::Some(&data_in[i]);
        }

        InputLayer { data: data_out }
    }
}

pub struct ConnectedGenericLayer <'a, L : Layer,A: ActivationFunction, const SIZE: usize, const PREV_SIZE: usize> {
    prev_layer : &'a mut L,
    cache_data : [f32; SIZE],
    fibers: [[f32; PREV_SIZE]; SIZE],
    a : PhantomData<A>
}

impl <'a, L, A, const SIZE: usize, const PREV_SIZE: usize> Layer for ConnectedGenericLayer<'a, L, A, SIZE, PREV_SIZE> where
    L : Layer,
    A: ActivationFunction,
{
    fn calculate_state(&mut self) {
        for i in 0..SIZE {
            let mut sum = 0.0;
            for j in 0..PREV_SIZE {
                sum += self.prev_layer.get_value(j).unwrap() * self.fibers[i][j];
            }
            self.cache_data[i] = A::activate(sum);
        }
    }
    fn get_value(&self, idx : usize) -> Option<f32> {
        if idx > SIZE {
            Option::None
        }else{
            Option::Some(self.cache_data[idx])
        }
    }

    fn update(&mut self, info: ModelInformation) {
        self.prev_layer.update(info);

        let learning_rate = info.get_lr();
        let mut rng = thread_rng();
        for i in 0..SIZE {
            for j in 0..PREV_SIZE {
                self.fibers[i][j] += rng.gen_range(-1.0..1.0) * learning_rate;
            }
        }
    }
}
impl <'a, L, A, const SIZE: usize, const PREV_SIZE: usize> ConnectedGenericLayer<'a, L, A, SIZE, PREV_SIZE> where
    L: Layer,
    A: ActivationFunction {
    pub fn new(prev_layer : &'a mut L) -> ConnectedGenericLayer<'a, L, A, SIZE, PREV_SIZE>{
        ConnectedGenericLayer { prev_layer, cache_data: [0.0; SIZE], fibers: [[1.0;PREV_SIZE];SIZE], a: PhantomData}
    }
}