extern crate metrohash;

use std::u64;
use std::hash::Hasher;

const NUM_REGISTERS: usize = 16384;
const ALPHA: f64 = (0.7213 / (1.0 + 1.079 / 16384.0));
const MAX_X: u64 = u64::MAX >> 50;
const SEED: u64 = 46769;
const PRECISION: u64 = 14;

fn beta(ez: f64) -> f64 {
    let zl = (ez + 1.0).ln();
    -0.370393914 * ez
        + 0.070471823 * zl
        + 0.17393686 * zl.powi(2)
        + 0.16339839 * zl.powi(3)
        - 0.09237745 * zl.powi(4)
        + 0.03738027 * zl.powi(5)
        - 0.005384159 * zl.powi(6)
        + 0.00042419 * zl.powi(7)
}

fn main() {
    let mut hashf = metrohash::MetroHash64::with_seed(SEED);
    let mut registers = [0u8; NUM_REGISTERS];
    let stdin = std::io::stdin();
    let mut line = String::new();

    while stdin.read_line(&mut line).unwrap() > 0 {
        let l2 = line.clone();
        let data: &[u8] = l2.as_bytes();
        hashf.write(&data);
        let hash = hashf.finish();

        let k = hash >> 50;
        let val = ((hash << PRECISION) ^ MAX_X).leading_zeros() as u8 + 1;
        if registers[k as usize] < val {
            registers[k as usize] = val;
        }
        line.clear();
    }

    let mut sum = 0.0;
    let mut ez = 0.0;
    let m = 16384.0;

    for i in 0..NUM_REGISTERS {
        if registers[i] == 0 {
            ez += 1.0;
        }
        sum += 1.0 / 2.0_f64.powf(registers[i] as f64);
    }

    println!("{}", (ALPHA * m * (m - ez) / (beta(ez) + sum)) as u64);
}
