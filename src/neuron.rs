pub struct Neuron<'a> {
    pub source: ValueSource<'a>
}

pub struct Fiber<'a> {
    source : &'a Neuron<'a>,
    weight : f32
}

pub enum ValueSource<'a>  {
    FiberConnection(Vec<Fiber<'a>>),
    ConstantConnection(&'a f32)
}
impl <'a> Neuron<'a> {
    pub fn empty_connected() -> Neuron<'a>{
        Neuron { source: ValueSource::FiberConnection(vec!()) }
    }
    pub fn empty_unconnected(f : &'a f32) -> Neuron<'a> {
        Neuron { source: ValueSource::ConstantConnection(f)}
    }
}

impl <'a> Fiber<'a> {
    pub fn new(from : &'a Neuron<'a>, weight :f32) -> Fiber<'a> {
        Fiber {
            source: from,
            weight,
        }
    }
}