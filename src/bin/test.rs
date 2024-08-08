use ndarray::prelude::*;
use ndarray_linalg::Inverse;
use ndarray_linalg::Norm;

fn main() {
    let x = arr1(&[1., 2., 3.]);
    let a = arr2(&[[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]);

    let inverse_matrix = a.inv().unwrap();

    println!("{:?}", x.norm());
    println!("{:?}", inverse_matrix);
    // println!("{:?}", a.inv());
}
