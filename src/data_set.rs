//trait Dataset {
//    fn get_size(&self);
//
//    fn apportion(&mut self, validation_ratio: f32);
//    fn get_training()
//
//}
use rand::prelude::*;
use std::marker::PhantomData;

pub struct Dataset <D: Datum<T, K, SIZE>, T, K, const SIZE: usize> {
    data : Vec<D>,
    validation : Vec<usize>,
    training: Vec<usize>,
    cur_val: usize,
    cur_training: usize,
    use_t: PhantomData<T>,
    use_k: PhantomData<K>
}
impl <'a, D, T, K, const SIZE: usize> Dataset<D, T, K, SIZE> where D: Datum<T, K, SIZE> {
    pub fn new<L : DatasetLoader<D, T, K, SIZE>>(mut loader: L, share : f32) -> Dataset<D, T, K, SIZE> {
        let mut data = vec!();
        let mut val = vec!();
        let mut train = vec!();
        let mut rand = thread_rng();
        let mut i = 0;
        while loader.has_next() {
            let a = loader.next();
            if let Some(x) = a{
                data.push(x);

                if rand.gen_range(0.0..1.0) < share{
                    val.push(i);
                }else {
                    train.push(i);
                }
                i += 1;
            }
        }
        Dataset { data, validation: val, training: train, cur_val: 0, cur_training: 0, use_t: PhantomData, use_k: PhantomData }
    }
    pub fn get_validation(&'a mut self) -> Option<&'a D> {
        match self.validation.get(self.cur_val) {
            Some(value) => Some(&self.data[value.clone()]),
            None => None
        }
    }
    pub fn get_training(&'a mut self) -> Option<&'a D> {
        match self.training.get(self.cur_training) {
            Some(value) => Some(&self.data[value.clone()]),
            None => None
        }
    }
}

pub trait DatasetLoader<D: Datum<T,K, DATA_SIZE>, T,K,  const DATA_SIZE: usize> {
    fn next(&mut self) -> Option<D>;
    fn has_next(&self) -> bool;
}

pub trait Datum <T,K, const SIZE: usize> {
    fn get_data(&self) -> [T; SIZE];
    fn from(data : Vec<u8>) -> Option<Self> where Self: Sized;
    fn seed(&self, receiver : &mut [T; SIZE]);
    fn get_classification(&self) -> K;
}

pub struct FileSystemLoader {
    paths: Vec<String>,
    current: usize
}
impl <D: Datum<T,K, SIZE>, T, K, const SIZE: usize> DatasetLoader<D, T,K, SIZE> for FileSystemLoader {
    fn next(&mut self) -> Option<D> {
        let c = self.current;
        self.current +=1;
        let mut file = match File::open(&self.paths[c]) {
            Ok(value) => value,
            Err(_) => return None
        };
        let mut bytes = vec!();
        if let Err(_) = file.read_to_end(&mut bytes) {
            return None
        }

        D::from(bytes)
    }

    fn has_next(&self) -> bool {
        self.current < self.paths.len()
    }
}

use std::fs::{File, read_dir};
use std::io::prelude::*;

impl FileSystemLoader {
    pub fn new(path : &str ) -> Result<FileSystemLoader, FileError>{
        let md = match std::fs::metadata(path) {
            Ok(value) => {
                value
            },
            Err(_) => {
                return Result::Err(FileError::PathNotFound(String::from(path)));
            },
        };
        return if md.is_dir() {
            // processes all files in the directory
            let dir = match read_dir(path) {
                Ok(value) => value,
                Err(_) => return Result::Err(FileError::DirectoryNotReadable(path.into()))
            };
            let mut paths = vec!();
            for path in dir {
                // if error, ignore the file
                match path {
                    Ok(value) => paths.push(value.path().display().to_string()),
                    Err(_) =>()
                };
            }
            Result::Ok(FileSystemLoader {
                paths,
                current: 0
            })
        } else if md.is_file() {
            // assume path points to json or csv containing paths
            if path.ends_with(".csv") {
                let paths = match FileSystemLoader::read_csv(path) {
                    Ok(value) => value,
                    Err(_) => return Result::Err(FileError::IncorrectFormat(path.into(), "csv".into()))
                };
                Result::Ok(FileSystemLoader {
                    paths,
                    current: 0
                })
            }else if path.ends_with(".json") {
                let paths = match FileSystemLoader::read_json(path) {
                    Ok(value) => value,
                    Err(_) => return Result::Err(FileError::IncorrectFormat(path.into(), "json".into()))
                };
                Result::Ok(FileSystemLoader{
                    paths,
                    current: 0
                })
            }
            else {
                Result::Err(FileError::IncorrectFormat(path.into(), "unknown".into()))
            }
        } else {
            Result::Err(FileError::PathNotDirectoryOrFile(path.into()))
        }
    }
    pub fn read_csv(path: &str)-> Result<Vec<String>, FileError>{
        let file = match File::open(path) {
            Ok(value) => value,
            Err(_) => return Result::Err(FileError::FileNotReadable(path.into()))
        };
        let mut reader = csv::Reader::from_reader(file);
        let mut paths : Vec<String> = vec!();
        for result in reader.records() {
            match result {
                Ok(value) => {
                    paths.push(value.get(0).unwrap().into());
                },
                Err(_) => ()
            }
        }
        Result::Ok(paths)
    }
    pub fn read_json(path : &str) -> Result<Vec<String>, FileError> {
        let mut file = match File::open(path) {
            Ok(value) => value,
            Err(_) => return Result::Err(FileError::FileNotReadable(path.into()))
        };
        let mut json_cache = String::new();
        if let Err(_) =  file.read_to_string(&mut json_cache) {
            return Err(FileError::FileNotReadable(path.into()))
        }

        let object : _JsonObject = match serde_json::from_str(&json_cache) {
            Ok(value) => value,
            Err(_) => return Err(FileError::IncorrectFormat(path.into(), "json".into()))
        };
        Result::Ok(object.names)
    }
}
#[derive(serde::Deserialize)]
struct _JsonObject {
    names : Vec<String>
}

#[derive(Debug)]
pub enum FileError {
    PathNotFound(String),
    PathNotDirectoryOrFile(String),
    DirectoryNotReadable(String),
    FileNotReadable(String),
    IncorrectFormat(String, String)
}
