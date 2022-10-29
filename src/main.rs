mod neuron;
mod layers;


use layers::Layer;
fn main() {
    let mut input = [0.0; 32];

    let input_layer = Layer::<32>::new_unconnected(&input);
    let layer1 = Layer::<128>::new_connected(&input_layer);
    let _output_layer = Layer::<1>::new_connected(&layer1);



}
