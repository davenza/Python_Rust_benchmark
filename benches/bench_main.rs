#[macro_use]
extern crate criterion;
extern crate ndarray;

use criterion::Criterion;
use ndarray::Array;

fn product(c: &mut Criterion) {
    c.bench_function_over_inputs("product",  |b, &size| {
        let shape = vec![2; size];
        let zeros = Array::<f64,_>::zeros(shape.clone());
        let ones = Array::<f64,_>::ones(shape);
        b.iter_with_setup(|| (zeros.clone(), ones.clone()),
                          |(zeros, ones)| zeros*ones)
    },vec![2usize, 4, 6, 8, 10, 12, 14]);
}

criterion_group!(product_group, product);

criterion_main!(product_group);
