
use clap::{App, Arg};
use std::path::Path;

use crate::ruzzer::Fuzzer;
use crate::ruzzer::PrinterFuzzer;
mod ruzzer;

fn main() {
    let matches = App::new("Ruzzer")
    .version("0.1.0")
    .author("Leonel Mendez Jimenez. <lmendezj@asu.edu>")
    .about("CLI App to generate random strings for fuzzing testing")
    .arg(Arg::with_name("iterations")
         .short("it")
         .long("iter")
         .value_name("iter")
         .help("Num of iterations to create strings")
         .takes_value(true)
         .required(true))
    .arg(Arg::with_name("prng_seed")
         .short("ps")
         .long("prng_seed")
         .value_name("prng")
         .help("Seed to pseudo random numbers")
         .takes_value(true)
         .required(true))
    .get_matches();

    let iterations = matches.value_of("iterations").unwrap_or("10").parse().unwrap_or(10);
    let prng_seed = matches.value_of("prng_seed").unwrap_or("10").parse().unwrap_or(10);
    let seed_path = Path::new("src/seed").to_str().unwrap_or("src/seed");
    PrinterFuzzer::new(seed_path, iterations).fuzz(prng_seed);
}
