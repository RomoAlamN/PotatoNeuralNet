use crate::activation::ActivationFunction;
use crate::model_info::ModelInformation;
//use crate::optimizer::Optimizer;
use std::marker::PhantomData;

use rand::prelude::*;

pub trait Layer {
    fn calculate_state(&mut self);
    fn get_value(&self, idx :usize) -> Option<f32>;
    // TODO: figure out how to pass the data needed to update smartly
    fn update(&mut self, info : ModelInformation);
}
use std::cell::{RefCell};
use std::rc::Rc;

#[derive(Clone)]
pub struct InputLayer <const SIZE: usize>{
    data : Rc<RefCell<[f32; SIZE]>>,
}
impl <'a, const SIZE: usize> Layer for InputLayer<SIZE> {
    fn calculate_state(&mut self) {
        //do nothing
    }
    fn get_value(&self, idx: usize) -> Option<f32> {
        if idx > SIZE {
            Option::None
        }else {

            Option::Some(self.data.borrow()[idx].clone())
        }
    }

    fn update(&mut self, _info: ModelInformation) {
        // do nothing (terminal layer)
    }
}
impl <'a, const SIZE: usize> InputLayer< SIZE> {
    pub fn new(data_in : Rc<RefCell<[f32; SIZE]>>) -> InputLayer<SIZE>{
        InputLayer { data: data_in }
    }
}
pub struct ConnectedGenericLayer <L : Layer,A: ActivationFunction, const SIZE: usize, const PREV_SIZE: usize> {
    prev_layer : Rc<RefCell<L>>,
    cache_data : [f32; SIZE],
    fibers: Vec<Vec<f32>>,
    a : PhantomData<A>
}

impl <'a, L, A, const SIZE: usize, const PREV_SIZE: usize> Layer for ConnectedGenericLayer<L, A, SIZE, PREV_SIZE> where
    L : Layer,
    A: ActivationFunction,
{
    fn calculate_state(&mut self) {
        self.prev_layer.borrow_mut().calculate_state();
        for i in 0..SIZE {
            let mut sum = 0.0;
            for j in 0..PREV_SIZE {
                sum += self.prev_layer.borrow_mut().get_value(j).unwrap() * self.fibers[i][j];
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
        self.prev_layer.borrow_mut().update(info);

        let learning_rate = info.get_lr();
        let mut rng = thread_rng();
        for i in 0..SIZE {
            for j in 0..PREV_SIZE {
                self.fibers[i][j] += rng.gen_range(-1.0..1.0) * learning_rate;
            }
        }
    }
}
impl <'a, L, A, const SIZE: usize, const PREV_SIZE: usize> ConnectedGenericLayer< L, A, SIZE, PREV_SIZE> where
    L: Layer,
    A: ActivationFunction {
    pub fn new(prev_layer : Rc<RefCell<L>>) -> ConnectedGenericLayer<L, A, SIZE, PREV_SIZE>{
        ConnectedGenericLayer { prev_layer, cache_data: [0.0; SIZE], fibers: vec![vec![1.0;PREV_SIZE];SIZE], a: PhantomData}
    }
}
impl <L, A, const SIZE: usize, const PREV_SIZE : usize > Clone for ConnectedGenericLayer<L, A, SIZE, PREV_SIZE>
where 
    L : Layer + Clone,
    A : ActivationFunction {
        fn clone(&self) -> Self {
        Self { prev_layer: Rc::new(RefCell::new(self.prev_layer.borrow().clone())), cache_data: self.cache_data.clone(), fibers: self.fibers.clone(), a: self.a.clone() }
    }
    }