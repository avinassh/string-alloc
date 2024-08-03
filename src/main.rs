use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;

use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use string_alloc::StringRS;

static MEDIUM_STRING_MAX: usize = (1 << 14) - 1;
#[allow(dead_code)]
fn generate_and_write_strings() -> std::io::Result<()> {
    let mut file = File::create("random_strings.txt")?;

    for _ in 0..10000 {
        let mut rng = rand::thread_rng();
        let size = rng.gen_range(5000..MEDIUM_STRING_MAX);
        let s = Alphanumeric.sample_string(&mut rng, size);
        writeln!(file, "{}", s)?;
    }

    Ok(())
}

fn new_string(s: &str) -> String {
    let result = String::from(s);
    result
}
fn main() {
    let file = File::open("random_strings.txt").unwrap();
    let reader = BufReader::new(file);
    let mut results = Vec::new();

    for (_index, line) in reader.lines().enumerate() {
        // if index >= 1000 {
        //     break;
        // }
        let s = line.unwrap();
        let alloc_start = Instant::now();
        let fs = StringRS::from(s.as_str());
        let stringrs_elapsed = alloc_start.elapsed().as_nanos() as u64;
        // println!("[stringrs] alloc: {:?}", alloc_end);

        let alloc_start = Instant::now();
        let old = new_string(s.as_str());
        results.push((alloc_start.elapsed().as_nanos() as u64, stringrs_elapsed));
        // println!("[stdlib] alloc: {:?}", alloc_end);
        assert_eq!(fs.len(), old.len())
    }
    println!("results: {}", results.len());
}
