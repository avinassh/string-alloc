use rand::distributions::{Alphanumeric, DistString};
use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::time::Instant;
use string_alloc::FastRS;

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
    let alloc_start = Instant::now();
    let result = String::from(s);
    let _alloc_end = alloc_start.elapsed().as_nanos();
    // println!("[stdlib] alloc: {:?}", alloc_end);
    result
}
fn main() {
    let file = File::open("random_strings.txt").unwrap();
    let reader = BufReader::new(file);

    for (_index, line) in reader.lines().enumerate() {
        // if index >= 1000 {
        //     break;
        // }
        let s = line.unwrap();
        let fs = FastRS::from(s.as_str());
        let old = new_string(s.as_str());
        assert_eq!(fs.len(), old.len())
    }
}
