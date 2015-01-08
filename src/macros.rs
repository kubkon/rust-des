macro_rules! assert_almost_eq {
    ($cond1:expr, $cond2:expr, $err:expr) => ({
        use std::num::Float;
        let c1 = $cond1;
        let c2 = $cond2;
        assert!((c1 - c2).abs() < $err, "{} is not almost equal to {}", c1, c2);
    })
}
