pub trait DataReader {
    fn consume<T : ConsumableType<SIZE>, const SIZE: usize>(&mut self) -> Vec<T>;
}

pub trait ConsumableType<const SIZE: usize> {
    fn from_arr(data_in : [u8; SIZE]) -> Self;
    fn from_vec(data_in: Vec<u8>) -> Self;
}


pub enum ReadError {
    FormatException
}

use std::fs::File;
use std::io::prelude::*;
pub struct BinaryFileReader<'a> {
    the_file : &'a mut File
}
impl <'a>  DataReader for BinaryFileReader<'a>{
    fn consume<T : ConsumableType<SIZE>, const SIZE: usize>(&mut self) -> Vec<T> {
        let mut ret = vec![];
        let mut buf = [0u8; SIZE];
        loop {
            if let Err(_) = self.the_file.read_exact(&mut buf) {
                break;
            }else {
                ret.push(T::from_arr(buf));
            }
        }
        ret
    }
}
impl <'a> BinaryFileReader<'a> {
    pub fn new(f : &'a mut File) -> BinaryFileReader {
        BinaryFileReader { the_file: f }
    }
}
pub struct PNGFileReader {
    buffer : Vec<u8>,
    current : usize
}

impl PNGFileReader {
    pub fn new (f: &mut File) -> Result<PNGFileReader, ReadError> {
        let decoder = png::Decoder::new(f);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        if let Err(_) = reader.next_frame(&mut buf) {
            return Err(ReadError::FormatException)
        }
        Ok(PNGFileReader {
            buffer : buf,
            current: 0
        })
    }
}
impl DataReader for PNGFileReader {
    fn consume<T : ConsumableType<SIZE>, const SIZE: usize>(&mut self) -> Vec<T> {
        let mut ret = vec![];
        let mut buf = [0u8; SIZE];
        loop {
            if self.buffer.len() - self.current >= SIZE {
                for i in 0..SIZE {
                    buf[i] = self.buffer[i + self.current]
                }
                self.current += SIZE;
                ret.push(T::from_arr(buf))
            }else {
                break;
            }
        }
        ret
    }
}

impl ConsumableType<4> for f32 {
    fn from_arr(data_in : [u8; 4]) -> Self {
        f32::from_be_bytes(data_in)
    }

    fn from_vec(data_in: Vec<u8>) -> Self {
        let mut arr = [0;4];
        arr[0] = data_in[0];
        arr[1] = data_in[1];
        arr[2] = data_in[2];
        arr[3] = data_in[4];
        Self::from_arr(arr)
    }
}
impl ConsumableType<1> for u8 {
    fn from_arr(data_in : [u8; 1]) -> Self {
        data_in[0]
    }

    fn from_vec(data_in: Vec<u8>) -> Self {
        data_in[0]
    }
}