use std::{fs, path::PathBuf, time::Instant};

#[fncli::cli]
fn main(file: PathBuf) {
    let source = fs::read_to_string(&file).unwrap();

    let instants = (0..1000).map(|_| {
        let start = Instant::now();
        endron::tokenizer::tokenize(&source);
        let end = Instant::now();

        end.duration_since(start).as_millis()
    });

    let total: u128 = instants.sum();
    let average = total / 1000;

    println!("{}ms", average);
}
