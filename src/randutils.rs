// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use getrandom::getrandom;
/// Utility functions for generating random polynomials.
use rand::distributions::{Distribution, Normal};
use rand::{rngs::StdRng, thread_rng, Rng, SeedableRng};

use super::*;
use crate::rqpoly::RqPolyContext;

pub fn sample_ternary_poly_prng<T>(context: Arc<RqPolyContext<T>>) -> RqPoly<T>
where
    T: SuperTrait<T>,
{
    let mut seed = [0u8; 32];
    getrandom::getrandom(&mut seed);
    let mut rng = StdRng::from_seed(seed);

    let q = context.q.rep();
    let q_minus_one = q - 1 as u64;

    let c = (0..context.n)
        .map(|_| {
            let t = rng.gen_range(-1i32, 2i32) as i64;
            let mut s: u64 = t as u64;
            if t < 0 {
                s = q_minus_one;
            }
            T::from(s)
        })
        .collect::<Vec<T>>();

    RqPoly {
        coeffs: c,
        is_ntt_form: false,
        context: Some(context),
    }
}

/// Sample a polynomial with Gaussian coefficients in the ring Rq.
pub fn sample_gaussian_poly<T>(context: Arc<RqPolyContext<T>>, stdev: f64) -> RqPoly<T>
where
    T: SuperTrait<T>,
{
    let normal = Normal::new(0.0, stdev);

    let mut seed = [0u8; 32];
    getrandom::getrandom(&mut seed);
    let mut rng = StdRng::from_seed(seed);

    let q: f64 = context.q.rep() as f64;

    let c = (0..context.n)
        .map(|_| {
            let mut tmp = normal.sample(&mut rng);
            if tmp < 0.0 {
                tmp += q;
            }
            T::from(tmp as u64)
        })
        .collect::<Vec<T>>();

    RqPoly {
        coeffs: c,
        is_ntt_form: false,
        context: Some(context),
    }
}

/// Sample a uniform polynomial in the ring Rq.
pub fn sample_uniform_poly<T>(context: Arc<RqPolyContext<T>>) -> RqPoly<T>
where
    T: SuperTrait<T>,
{
    let mut seed = [0u8; 32];
    getrandom::getrandom(&mut seed);
    let mut rng = StdRng::from_seed(seed);

    let c: Vec<T> = vec![0; context.n]
        .iter()
        .map(|_| T::sample_below_from_rng(&context.q, &mut rng))
        .collect();
    RqPoly {
        coeffs: c,
        is_ntt_form: false,
        context: Some(context),
    }
}
