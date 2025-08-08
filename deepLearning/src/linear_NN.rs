

use burn::{
    backend::NdArray, module::Param, tensor::{Shape, Tensor}
};
use burn_autodiff::{grads::Gradients, Autodiff};
use plotters::prelude::*;
use rand::{rng, seq::SliceRandom};

type Backend = Autodiff<NdArray<f64>>;

fn synthetic_data(w: &[f64], b: f64, num: usize) -> (Tensor<Backend, 2>, Tensor<Backend, 2>) {
    let feature_dim = w.len();
    let device = &Default::default();

    // X: [num, feature_dim]
    let x = Tensor::<Backend, 2>::random(
        Shape::new([num, feature_dim]),
        burn::tensor::Distribution::Normal(0.0, 1.0),
        device,
    );

    // w: [feature_dim, 1]
    let w_tensor = Tensor::<Backend, 1>::from_floats(w, device).reshape([feature_dim, 1]);

    // y = Xw + b + noise
    let mut y = x.clone().matmul(w_tensor); // [num, 1]
    y = y + b;

    let noise = Tensor::<Backend, 2>::random(
        Shape::new([num, 1]),
        burn::tensor::Distribution::Normal(0.0, 0.01),
        device,
    );

    y = y + noise;
    (x, y)
}

fn plot_scatter(features: &Tensor<Backend, 2>, labels: &Tensor<Backend, 2>) {
    let x_data = features.to_data().into_vec().unwrap(); // Vec<f64>
    let y_data = labels.to_data().to_vec().unwrap();

    let feature_dim = features.shape().dims[1];

    let x_column_1: Vec<f64> = x_data
        .chunks(feature_dim)
        .map(|row| row[1]) // 使用第 2 个特征
        .collect();

    assert_eq!(x_column_1.len(), y_data.len());

    let root = BitMapBackend::new("scatter.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("Feature[1] vs Label", ("sans-serif", 20))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-3.0..3.0, -10.0..20.0)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for (x, y) in x_column_1.iter().zip(y_data.iter()) {
        chart
            .draw_series(std::iter::once(Circle::new((*x, *y), 2, RED.filled())))
            .unwrap();
    }

    root.present().unwrap();
    println!("Saved to scatter.png");
}

fn data_iter(
    batch_size: usize,
    features: &Tensor<Backend, 2>,
    labels: &Tensor<Backend, 2>,
) -> impl Iterator<Item = (Tensor<Backend, 2>, Tensor<Backend, 2>)> {
    let num_examples = features.shape().dims[0];
    let mut indices: Vec<usize> = (0..num_examples).collect();
    indices.shuffle(&mut rng());

    (0..num_examples).step_by(batch_size).map(move |i| {
        let end = (i + batch_size).min(num_examples);
        let batch_indices = &indices[i..end];

        let device = &Default::default();
        let batch_indices = Tensor::<Backend, 1, burn::tensor::Int>::from_ints(batch_indices, device);
        let batch_features = features.clone().select(0, batch_indices.clone());
        let batch_labels = labels.clone().select(0, batch_indices);

        (batch_features, batch_labels)
    })
}

fn init_params() -> (Param<Tensor<Backend, 2>>, Param<Tensor<Backend, 1>>) {
    let w = Tensor::<Backend, 2>::random(
        Shape::new([2, 1]),
        burn::tensor::Distribution::Normal(0.0, 1.0),
        &Default::default(),
    );
    let b = Tensor::zeros(Shape::new([1]), &Default::default());
    (Param::from_tensor(w), Param::from_tensor(b))
}

fn linreg(x:Tensor<Backend, 2>, w: &Param<Tensor<Backend, 2>>, b: &Param<Tensor<Backend, 1>>) -> Tensor<Backend, 2>{
    x.matmul(w.val()) + b.val().unsqueeze()
}

fn squared_loss(y_hat: Tensor<Backend, 2>, y: Tensor<Backend, 2>) -> Tensor<Backend, 2> {
    // 均方误差: (y_hat - y)^2 / 2
    (y_hat - y).powf_scalar(2.0) / 2.0
}

pub fn sgd<const W:usize, const B:usize>(
    w: Param<Tensor<Backend, W>>, 
    b: Param<Tensor<Backend, B>>,   // 或者你也可以传 Param<Tensor<B, 1>> 类型数组
    grads: Gradients,
    lr: f64,
    batch_size: usize,
) -> (Param<Tensor<Backend, W>>, Param<Tensor<Backend, B>>){
    let tensor_w : Param<Tensor<Backend, W>>  = if let Some(grad) = w.grad(&grads) {
        let update = grad.clone().mul_scalar(lr / batch_size as f64);
        let new_value = w.val().clone().inner() - update;
        let tensor_value = Tensor::from_inner(new_value);
        Param::from_tensor(tensor_value)
    } else {
        w.clone()
    };

    let tensor_b : Param<Tensor<Backend, B>>  = if let Some(grad) = b.grad(&grads) {
        let update = grad.clone().mul_scalar(lr / batch_size as f64);
        let new_value = b.val().clone().inner() - update;
        let tensor_value = Tensor::from_inner(new_value);
        Param::from_tensor(tensor_value)
    } else {
        b.clone()
    };

    (tensor_w, tensor_b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthetic_data_scatter() {
        let true_w = vec![2.0, -3.4];
        let true_b = 4.2;

        let (features, labels) = synthetic_data(&true_w, true_b, 1000);
        for (x, y) in data_iter(64, &features, &labels) {
            println!("x: {}, y: {}", x, y);
        }
        plot_scatter(&features, &labels);
    }

    #[test]
    fn test_param_init() {
        let (w, b) = init_params();
        println!("Weight: {}", w.val().to_data());
        println!("Bias: {}", b.val().to_data());
    }

    #[test]
    fn test_training(){
        let num_epochs = 100 as usize;
        let true_w = vec![2.0, -3.4];
        let true_b = 4.2;
        let batch_size = 64;
        let lr = 0.03;

        let (mut w, mut b) = init_params();
         println!("w:{}, b:{}", &w.val().to_data(), &b.val().to_data());

        let (features, labels) = synthetic_data(&true_w, true_b, 1000);

        for epoch in 0..num_epochs {
            for (x, y) in data_iter(batch_size, &features, &labels){
                let loss = squared_loss(linreg(x, &w, &b), y);
                let grad = loss.sum().backward();
                (w, b) = sgd(w, b, grad, lr, batch_size)
            }

            let loss = squared_loss(linreg(features.clone(), &w, &b), labels.clone());

            println!("eopch:{}, w:{}, b:{},  loss:{:.3}", epoch+1, &w.val().to_data(), &b.val().to_data(), loss.mean().to_data());
        }
    }
}
