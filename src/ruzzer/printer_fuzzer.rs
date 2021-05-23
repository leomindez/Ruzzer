use core::panic;

use crate::ruzzer::fuzzer::Fuzzer;
use rand::distributions::Alphanumeric;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;

#[derive(Debug)]
pub struct PrinterFuzzer {
    seed_path: &'static str,
    iterations: i32,
}

impl PrinterFuzzer {
    pub fn create_mutation_string(&self, prng_seed: u64, mut content_string: Vec<u8>) {
        let mut prng = Pcg32::seed_from_u64(prng_seed);
        println!("{:?}", content_string);
        let num_mutations = (0.13 * content_string.len() as f64) as i32;
        for index in 0..self.iterations {
            if self.iterations >= 500 && (index % 500 == 0) {
                let mut ten_chars = rand::thread_rng()
                    .sample_iter(Alphanumeric)
                    .take(10)
                    .map(|e| e as u8)
                    .collect::<Vec<u8>>();
                content_string.append(&mut ten_chars);
            }
            for _ in 0..num_mutations {
                let random_char = prng.sample(Alphanumeric) as u8;
                let index = prng.gen_range(0, content_string.len());
                content_string[index] = random_char;
            }
            println!("{:?}", content_string);
        }
    }
}

impl Fuzzer for PrinterFuzzer {
    fn new(seed_path: &'static str, iterations: i32) -> PrinterFuzzer {
        PrinterFuzzer {
            seed_path: seed_path,
            iterations: iterations,
        }
    }

    fn read_seed(&self) -> String {
        match std::fs::read_to_string(&self.seed_path) {
            Ok(content) => content,
            Err(_) => panic!("There is no the correct path to get seed string"),
        }
    }

    fn fuzz(&self, prng_seed: u64) {
        let content = self.read_seed().into_bytes();
        PrinterFuzzer::create_mutation_string(self, prng_seed, content);
    }
}
