#[macro_use]
extern crate dudect_bencher;
extern crate rand;
extern crate orion_dudect;
extern crate constant_time_eq;

use dudect_bencher::{BenchRng, Class, CtRunner};
use rand::Rng;
use orion_dudect::NUMBER_OF_SAMPLES;
use constant_time_eq::constant_time_eq;

// Return a random vector of length len
fn rand_vec(len: usize, rng: &mut BenchRng) -> Vec<u8> {
    let mut arr = vec![0u8; len];
    rng.fill_bytes(&mut arr);
    arr
}

// Based on `dudect-bencher`s examples.
fn test_secure_cmp(runner: &mut CtRunner, rng: &mut BenchRng) {
    // The length of input vectors.
    let vlen = 32;
    
    let mut inputs: Vec<(Vec<u8>, Vec<u8>)> = Vec::new();
    let mut classes = Vec::new();

    for _ in 0..NUMBER_OF_SAMPLES {
        if rng.gen::<bool>() {
            let v1 = rand_vec(vlen, rng);
            let v2 = v1.clone();
            inputs.push((v1, v2));
            classes.push(Class::Left);
        }
        else {
            let v1 = rand_vec(vlen, rng);
            let v2 = vec![0u8; v1.len()];
            inputs.push((v1, v2));
            classes.push(Class::Right);
        }
    }

    // Run timing
    for (class, (u, v)) in classes.into_iter().zip(inputs.into_iter()) {
        runner.run_one(class, || constant_time_eq(&u[..], &v[..]));
    }
}

ctbench_main!(test_secure_cmp);
