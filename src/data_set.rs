use std::cell::RefCell;
//trait Dataset {
//    fn get_size(&self);
//
//    fn apportion(&mut self, validation_ratio: f32);
//    fn get_training()
//
//}
use std::ffi::OsStr;
use std::marker::PhantomData;
use std::ops::AddAssign;
use std::rc::Rc;
use rand::prelude::*;

#[allow(dead_code)]
pub struct Dataset<D: Datum<SIZE>, L, const SIZE: usize>
where
    L: DatasetLoader<D, SIZE>,
{
    data: Vec<ClassifiedData<D, SIZE>>,
    validation: Vec<usize>,
    training: Vec<usize>,
    cur_val: Rc<RefCell<usize>>,
    cur_training: Rc<RefCell<usize>>,
    ld : PhantomData<L>
}
impl<'a, D, L, const SIZE: usize> Dataset<D, L, SIZE>
where
    D: Datum<SIZE>,
    L: DatasetLoader<D, SIZE>,
{
    pub fn new(mut loader: L, share: f32) -> Dataset<D, L, SIZE> {
        let mut data = vec![];
        let mut val = vec![];
        let mut train = vec![];
        let mut rand = thread_rng();
        let mut i = 0;
        while loader.has_next() {
            let a = loader.next();
            if let Some(x) = a {
                data.push(x);

                if rand.gen_range(0.0..1.0) < share {
                    val.push(i);
                } else {
                    train.push(i);
                }
                i += 1;
            }
        }
        Dataset {
            data,
            validation: val,
            training: train,
            cur_val: Rc::new(RefCell::new(0)),
            cur_training: Rc::new(RefCell::new(0)),
            ld : PhantomData
        }
    }
    #[allow(dead_code)]
    pub fn get_validation(&'a self) -> Option<&'a ClassifiedData<D, SIZE>> {
        let a = match self.validation.get(*self.cur_val.borrow()) {
            Some(value ) => Some(&self.data[*value]),
            None => None,
        };
        self.cur_val.borrow_mut().add_assign(1);
        a
    }
    pub fn get_training(&'a self) -> Option<&'a ClassifiedData<D, SIZE>> {
        let a = match self.training.get(*self.cur_training.borrow()) {
            Some(value) => Some(&self.data[*value]),
            None => None,
        };
        self.cur_training.borrow_mut().add_assign(1);
        a
    }
    pub fn has_training(&self) -> bool {
        return *self.cur_training.borrow() < self.training.len();
    }
    pub fn reset(&self) {
        *self.cur_training.borrow_mut() = 0;
        *self.cur_val.borrow_mut() = 0;
    }
}

pub trait DatasetLoader<D : Datum<SIZE>, const SIZE: usize> {

    fn next(&mut self) -> Option<ClassifiedData<D, SIZE>>;
    fn has_next(&self) -> bool;
}

pub struct ClassifiedData<D : Datum<SIZE>, const SIZE : usize> {
    data : D,
    classification : f32
}
impl < D: Datum<SIZE> , const SIZE : usize> ClassifiedData<D, SIZE> {
    pub fn get_data(&self) -> &D {
        &self.data
    }
    pub fn get_class(&self) -> f32 {
        self.classification
    }
}
pub trait Datum<const SIZE: usize> : Copy {
    type DataType;
    type ReceiverType;
    fn from(data: Vec<u8>) -> Option<Self>
    where
        Self: Sized;
    fn seed(&self, receiver: Self::ReceiverType);
}
use std::path::PathBuf;
use crate::data_importer::{PNGFileReader, DataReader, BinaryFileReader};


pub struct FileSystemLoader {
    paths: Vec<DataItem>,
    current: usize,
    root: PathBuf
}
impl<D: Datum<SIZE>, const SIZE: usize> DatasetLoader<D, SIZE>
    for FileSystemLoader
{
    fn next(&mut self) -> Option<ClassifiedData<D, SIZE>> {
        let c = self.current;
        self.current += 1;
        let mut true_path = self.root.clone();
        true_path.push(&self.paths[c].path);

        let mut file = match File::open(true_path.clone()) {
            Ok(value) => value,
            Err(_) => return None,
        };
        
        if let Some("png") = true_path.extension().and_then(OsStr::to_str) {
            let _reader = PNGFileReader::new(&mut file);
            if let Ok(mut reader) = _reader {
                let img = reader.consume();
                if let Some(value) = D::from(img) {
                    Some (
                        ClassifiedData {
                            data : value,
                            classification: self.paths[c].classification
                        }
                    )
                }else {
                    None
                }
            }else {
                None
            }
        }else {
            let mut reader = BinaryFileReader::new(&mut file);
            let img = reader.consume();
            if let Some(value) = D::from(img) {
                Some (
                    ClassifiedData {
                        data: value,
                        classification: self.paths[c].classification
                    }
                )
            }else {
                None
            }
        }

    }

    fn has_next(&self) -> bool {
        self.current < self.paths.len()
    }
}

use std::fs::File;
use std::io::prelude::*;

impl FileSystemLoader {
    pub fn new(path: &str) -> Result<FileSystemLoader, FileError> {
        let md = match std::fs::metadata(path) {
            Ok(value) => value,
            Err(_) => {
                return Result::Err(FileError::PathNotFound(String::from(path)));
            }
        };
        let mut a = std::path::PathBuf::new();
        a.push(path);
        a.pop();
        return if md.is_file() {
            // assume path points to json or csv containing paths
            if path.ends_with(".json") {
                let paths = match FileSystemLoader::read_json(path) {
                    Ok(value) => value,
                    Err(_) => {
                        return Result::Err(FileError::IncorrectFormat(path.into(), "json".into()))
                    }
                };
                Result::Ok(FileSystemLoader { paths, current: 0, root: a })
            } else {
                Result::Err(FileError::IncorrectFormat(path.into(), "unknown".into()))
            }
        } else {
            Result::Err(FileError::PathNotDirectoryOrFile(path.into()))
        };
    }
    pub fn read_json(path: &str) -> Result<Vec<DataItem>, FileError> {
        let mut file = match File::open(path) {
            Ok(value) => value,
            Err(_) => return Result::Err(FileError::FileNotReadable(path.into())),
        };
        let mut json_cache = String::new();
        if let Err(_) = file.read_to_string(&mut json_cache) {
            return Err(FileError::FileNotReadable(path.into()));
        }

        let object: JsonDataset = match serde_json::from_str(&json_cache) {
            Ok(value) => value,
            Err(_) => return Err(FileError::IncorrectFormat(path.into(), "json".into())),
        };
        Result::Ok(object.data_items)
    }
}
use serde::Deserialize;
#[derive(Deserialize)]
pub struct DataItem {
    path: String,
    classification: f32,
}
#[derive(Deserialize)]
struct JsonDataset {
    data_items: Vec<DataItem>,
}

#[derive(Debug)]
pub enum FileError {
    PathNotFound(String),
    PathNotDirectoryOrFile(String),
    FileNotReadable(String),
    IncorrectFormat(String, String),
}
