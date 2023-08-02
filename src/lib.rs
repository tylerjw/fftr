pub fn arange(start: f32, stop: f32, step: f32) -> Vec<f32> {
    let n_items = ((stop - start) / step) as usize;
    (0..n_items).map(|x| start + (x as f32) * step).collect()
}

#[test]
fn test_arange() {
    assert_eq!(arange(0., 2., 1.), vec![0., 1.]);
    assert_eq!(arange(0., 2., 0.5), vec![0.0, 0.5, 1.0, 1.5]);
    assert_eq!(arange(0.2, 2., 0.5), vec![0.2, 0.7, 1.2]);
}
