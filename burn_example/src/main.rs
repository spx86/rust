#![recursion_limit = "256"]
mod data;
mod inference;
mod model;
mod training;


use crate::{model::ModelConfig, training::TrainingConfig};
use burn::{
    backend::{Autodiff, NdArray},
    data::dataset::Dataset,
    optim::AdamConfig,
};

type Backend = NdArray;
type AutodiffBackend = Autodiff<Backend>;
fn main() {
    let device = burn::backend::ndarray::NdArrayDevice::Cpu;
    let artifact_dir = "/home/spengxu/rust/burn_example/model";
    crate::training::train::<AutodiffBackend>(
        artifact_dir,
        TrainingConfig::new(ModelConfig::new(10, 512), AdamConfig::new()),
        device.clone(),
    );

    crate::inference::infer::<Backend>(
        artifact_dir,
        device,
        burn::data::dataset::vision::MnistDataset::test()
            .get(42)
            .unwrap(),
    );
}
