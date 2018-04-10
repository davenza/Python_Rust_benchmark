//use criterion::Criterion;
//use test_utils;
//
//fn product(c: &mut Criterion) {
//    c.bench_function_over_inputs("TabularFactor_product",  |b, &size| {
//        let nodes_left = test_utils::create_vec_node_index(0, size);
//        let factor_left = test_utils::create_factor::<f64, u32>(nodes_left);
//
//        let nodes_right =  test_utils::create_vec_node_index(size/2, size);
//        let factor_right = test_utils::create_factor::<f64, u32>(nodes_right);
//        b.iter(|| factor_left.product(&factor_right))
//    },vec![2usize, 4, 6, 8, 10, 12, 14]);
//}
//
//criterion_group!(product_group, product);
//
//criterion_main!(product_group);
