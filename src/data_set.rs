//trait Dataset {
//    fn get_size(&self);
//
//    fn apportion(&mut self, validation_ratio: f32);
//    fn get_training()
//
//}
use rand::prelude::*;

pub struct Dataset <D: Datum> {
    data : Vec<D>,
    validation : Vec<usize>,
    training: Vec<usize>
}
impl <D> Dataset<D> where D: Datum {
    pub fn new<L : DatasetLoader<D>>(loader: L, share : f32) -> Dataset<D> {
        let mut data = vec!();
        let mut val = vec!();
        let mut train = vec!();
        let mut rand = thread_rng();
        let mut i = 0;
        while loader.has_next() {
            data.push(loader.next());
            if rand.gen_range(0.0..1.0) < share{
                val.push(i);
            }else {
                train.push(i);
            }
            i += 1;
        }
        Dataset { data, validation: val, training: train }
    }
}

pub trait DatasetLoader<D: Datum> {
    fn next(&mut self) -> D;
    fn has_next(&self) -> bool;
}

pub trait Datum {

}

struct FolderLoader {

}
impl <D: Datum> DatasetLoader<D> for FolderLoader {
    fn next(&mut self) -> D {
        todo!()
    }

    fn has_next(&self) -> bool {
        todo!()
    }
}