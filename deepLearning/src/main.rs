mod timer;
mod linear_NN;

use burn::backend::NdArray;
use burn::tensor::{Tensor, TensorData};
use ndarray::{Array, Order, Zip, array, concatenate, s};
type Backend = NdArray;
fn main() {
    let x = Array::range(0.0, 12.0, 1.0);
    println!("x:{}", x);
    println!("x's size:{}", x.len());
    //let x = x.into_shape_with_order((3, 4)).unwrap();
    let x = x.to_shape(((3, 4), Order::RowMajor)).unwrap();
    println!("x:{}", x);
    println!("x's shape:{:?}", x.shape());
    let x = Array::<f32, _>::zeros((4, 5, 3));
    println!("x:{}", x);
    let tensor = Array::from_shape_fn((3, 4), |_| {
        let rand: f64 = rand::random::<f64>();
        // 使用Box-Muller变换生成标准正态分布
        let u1 = rand;
        let u2 = rand::random::<f64>();
        (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos()
    });

    println!("Generated tensor:\n{}", tensor);

    let x = array![[1f64, 2.], [3., 4.]];
    println!("x:{}", x.exp());

    let y = array![[4., 5.], [6., 7.]];

    let pow = Zip::from(&x).and(&y).map_collect(|a, b| a == b);

    println!("x **y = {}", pow);

    let concat = concatenate(ndarray::Axis(1), &[x.view(), y.view()]).unwrap();
    println!("concat:{}", concat);
    println!("{}", x.sum());

    let x = Array::range(0., 3., 1.);
    let x = x.to_shape((3, 1)).unwrap();
    let y = Array::range(0., 2., 1.);
    let y = y.to_shape((1, 2)).unwrap();
    println!("x:{}\ny:{}", x, y);
    println!("x+y= {}", &x + &y);
    println!("{}\t{}\t{}", x.row(1), x.row(2), x.slice(s![0..2, -1]));

    let x = Array::range(0.0, 9., 1.);
    // let mut x = x.to_shape((3, 3)).unwrap();
    // {
    //     let mut sub = x.slice_mut(s![0..2, ..]);
    //     sub.fill(0.);
    // }
    println!("x:{}", x);

    println!("Array type: {}", std::any::type_name_of_val(&x));

    let device = Default::default();
    let x = x.to_owned().into_raw_vec_and_offset().0;
    let data = TensorData::new(x, [3, 3]);

    let tensor_1 = Tensor::<Backend, 2>::from_data(data, &device);
    let tensor_2 = tensor_1.clone();

    println!("tensor_1:{}\n tensor_2:{}", tensor_1, tensor_2);
    //println!("{}", tensor_1 - tensor_2);
    let mut timer = timer::Timer::new();
    println!("{}", tensor_1.powi(tensor_2));
    println!("{}",timer.stop());
    
}
