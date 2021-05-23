pub trait Fuzzer {
    fn new(seed_path: &'static str, iterations: i32) -> Self;
    fn read_seed(&self) -> String;
    fn fuzz(&self, prng_seed: u64);
}
