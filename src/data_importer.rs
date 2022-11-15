/// Represents an object that consumes an input and generates a ConsumableType.
/// 
/// DataReader is generally used to read input from files, such as in BinaryFileReader
/// and PNGFileReader. They can also be set up to read data from memory, or a network, although
/// those should be optimized by storing a buffer. 
/// 
/// The conversion of types is abstracted by the ConsumableType trait.
/// 
pub trait DataReader {
    fn consume<T : ConsumableType<SIZE>, const SIZE: usize>(&mut self) -> Vec<T>;
}

/// Represents a type that can be created from a vector or array with a certain size.
/// 
/// ConsumableType represents a type that can be constructed from a stream of bytes. A lot of the 
/// integral and float types implement this trait. 
/// 
/// ```
/// f32::from_arr([0x0, 0x0, 0x0, 0x0]);
/// u32::from_vec(vec![0x0, 0x1, 0x2, 0x3]);
/// ```
pub trait ConsumableType<const SIZE: usize> {
    fn from_arr(data_in : [u8; SIZE]) -> Self;
    fn from_vec(data_in: Vec<u8>) -> Self;
}


pub enum ReadError {
    FormatException
}

use std::fs::File;
use std::io::prelude::*;
/// Represents a packed binary file.
/// 
/// Consumes data from a file, then returns the ConsumableType requested.
/// 
/// ``` 
/// let mut reader = BinaryFileReader::new(...);
/// reader.consume::<f32>::();
/// let a = reader.consume();
/// if a > 0.0 {
///     println!("Wow!");
/// }
/// ```
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
    /// Creates a new BinaryFileReader from the passed file.
    fn new(f : &'a mut File) -> BinaryFileReader {
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