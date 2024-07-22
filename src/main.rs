use std::time::Instant;
use string_alloc::FastRS;
use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;

fn new_string(s: &str) -> String {
    let alloc_start = Instant::now();
    let result = String::from(s);
    let alloc_end = alloc_start.elapsed().as_nanos();
    println!("[stdlib] alloc: {:?}", alloc_end);
    result
}

fn main() {
    for _ in 0..10 {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(15000..20000);
        let sample = Alphanumeric.sample_string(&mut rng, size);
        let fs = FastRS::from(sample.as_str());
        let s = new_string(sample.as_str());
        assert_eq!(fs.len(), s.len())
    }
}