extern crate ndarray;

use ndarray::{aview1, aview2, Array};

fn main() {

    let values1 = vec![1; 8];
    let values2 = vec![2; 2];

    let array1 = Array::from_shape_vec((2,2,2), values1).unwrap();
    let array2 = Array::from_shape_vec((1,2,1), values2).unwrap();

    println!("Array1: {}", array1);
    println!("Array2: {}", array2);
    println!("Mul: {}", array1 * array2);

    println!("{}", aview2(&[[1., 0.]; 10]));
    assert!(
        aview1(&[1., 0.]).broadcast((10, 2)).unwrap()
            == aview2(&[[1., 0.]; 10])
    );
}