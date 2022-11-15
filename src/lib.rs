//! Simple Neural Network written in rust
//!
//! Allows you to create architectures and train models in a cumbersome and unnatural way
//!

mod activation;
mod data_importer;
mod data_set;
mod layers;
mod model_info;

pub use layers::{Layer, ConnectedGenericLayer, InputLayer};
pub use activation::ActivationFunction;
pub use data_importer::{ConsumableType, DataReader, PNGFileReader, BinaryFileReader, ReadError};
pub use data_set::{DatasetLoader, Dataset, Datum};
pub use model_info::ModelInformation;