extern crate two_sum;
extern crate two_product;

use two_sum::two_sum;
use two_product::two_product;

pub fn robust_scale(e: &[f64], scale: f64) -> Vec<f64> {
    linear_expansion(e, scale)
}

fn linear_expansion(e: &[f64], scale: f64) -> Vec<f64> {
    let n = e.len();
    if n == 1 {
        let ts = two_product(e[0], scale);
        if ts[0] != 0.0 {
            return ts.to_vec();
        }
        return vec!(ts[1]);
    }
    let mut q;
    let mut t;
    let mut g = vec!(0.0; 2 * n);
    let mut count = 0;
    q = two_product(e[0], scale);
    if q[0] != 0.0 {
        g[count] = q[0];
        count += 1;
    }
    for i in 1..n {
        t = two_product(e[i], scale);
        let pq = q[1];
        q = two_sum(pq, t[0]);
        if q[0] != 0.0 {
            g[count] = q[0];
            count += 1;
        }
        let a = t[1];
        let b = q[1];
        let x = a + b;
        let bv = x - a;
        let y = b - bv;
        q[1] = x;
        if y != 0.0 {
            g[count] = y;
            count += 1;
        }
    }
    if q[1] != 0.0 {
        g[count] = q[1];
        count += 1;
    }
    if count == 0 {
        g[count] = 0.0;
        count += 1;
    }
    g[0..count].to_vec()
}


#[cfg(test)]
mod robust_scale {
    use super::robust_scale;

    #[test]
    fn test_robust_scale() {
        assert_eq!(robust_scale(&vec!(4.), 2.), [8.]);
        assert_eq!(robust_scale(&vec!(1., 1e64), 2.), [2., 2e64]);
        assert_eq!(robust_scale(&vec!(1.), 1.), [1.]);
        let s = robust_scale(
            &vec!(-2.4707339790384e-144, -1.6401064715739963e-142, 2e-126),
            -10e-64
        );
        assert!(s[s.len() - 1] < 0.0);

        for i in -50..50 {
            for j in -50..50 {
                assert_eq!(
                    robust_scale(&vec!(i as f64), j as f64),
                    [(i * j) as f64]
                )
            }
        }
    }
}
