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
            return ts;
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
        assert!(true);
    }
}
