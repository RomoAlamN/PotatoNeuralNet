use crate::neuron::{Neuron, Fiber, ValueSource};
pub struct Layer <'a, const SIZE: usize> {
    neurons : [Neuron<'a>; SIZE],
    fibers_in : Vec<Fiber<'a>>,
}

impl <'a, const SIZE: usize> Layer<'a, SIZE> {
    pub fn new_connected<const IN_SIZE: usize>(layer_in: &'a Layer<'a, IN_SIZE>) -> Layer<'a, SIZE> {
        // create an array of neurons, since they cannot be copied safely
        let mut layer: Layer<'a, SIZE> = Layer {
            neurons: [(); SIZE].map(|_| Neuron::empty_connected()),
            fibers_in: Vec::with_capacity(SIZE * IN_SIZE),
        };
        // holy mother of connections this is O(n^2) so buckle up buddy
        for i in 0..IN_SIZE {
            for j in 0..SIZE {
                layer.fibers_in.push(Fiber::new(&layer_in.neurons[i], 1.0));
                match &mut layer.neurons[j].source {
                    ValueSource::FiberConnection(fibers) => {
                        //TODO: check if differente performance for pushing same fiber vs diff fiber with same references
                        fibers.push(Fiber::new(&layer_in.neurons[i], 1.0))
                    },
                    ValueSource::ConstantConnection(_) => panic!("I just made these, there is something wrong"),
                }
            }
        }
        layer
    }
    pub fn new_unconnected(values : &'a[f32; SIZE]) -> Layer<'a, SIZE> {
        let mut vals_array = [0; SIZE];
        for i in 0..SIZE {
            vals_array[i] = i;
        }
        let neurons_array = vals_array.map(|idx| Neuron::empty_unconnected(&values[idx]));
        Layer {
            neurons: neurons_array,
            fibers_in: vec![],
        }
    }
}