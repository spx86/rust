mod abi;
pub use abi::*;

use base64::{alphabet, engine, read, write};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

impl ImageSpec {
    pub fn new(spec: Vec<Spec>) -> Self {
        Self { specs }
    }
}

impl From<&ImageSpec> for String {
    fn from(spec: &ImageSpec) -> Self {
        let data = spec.encode_to_vec().expect("Failed to encode ImageSpec");
        write::base64(alphabet::URL_SAFE_NO_PAD, &data).expect("Failed to encode ImageSpec to base64")
    }
}

