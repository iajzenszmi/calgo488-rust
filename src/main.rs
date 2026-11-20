//! ACM CALGO Algorithm 488 (Richard P. Brent, 1974), independent Rust port.
//! Same single-precision numerical steps and lookup table as GRAND.
//! The original GRAND(N) needs an external RAND routine.  This demonstration
//! supplies the *same* 32-bit LCG to the Rust and Fortran programs.
//!
//! Usage: cargo run --release -- <unsigned-seed> <count>
//! Example: cargo run --release -- 12345 10

use std::env;

const D: [f32; 60] = [
    0.674489750, 0.475589630, 0.383771164, 0.328611323,
    0.291142827, 0.263684322, 0.242508452, 0.225567444,
    0.216346166, 0.199924267, 0.189917580, 0.181925181,
    0.173601400, 0.166841909, 0.160796729, 0.155349717,
    0.150409384, 0.145902577, 0.141770033, 0.137963174,
    0.134441762, 0.131172150, 0.128259650, 0.125279090,
    0.122610883, 0.120350610, 0.117741707, 0.115511892,
    0.113402349, 0.111402720, 0.109503852, 0.107697617,
    0.105976772, 0.104334841, 0.102766212, 0.101265052,
    0.099827234, 0.098448282, 0.097124309, 0.095851178,
    0.094627461, 0.093444847, 0.092311989, 0.091215482,
    0.090156838, 0.089133867, 0.088144619, 0.087187293,
    0.086260215, 0.085361834, 0.084490706, 0.083645487,
    0.082824924, 0.082027847, 0.081253162, 0.080498544,
    0.079769632, 0.078953527, 0.078358781, 0.077681899,
];

/// Portable shared stream: top 24 bits of a wrapping 32-bit LCG.
struct Lcg32 {
    state: u32,
}
impl Lcg32 {
    fn uniform(&mut self) -> f32 {
        self.state = self.state.wrapping_mul(1_664_525)
            .wrapping_add(1_013_904_223);
        ((self.state >> 8) as f32) * (1.0_f32 / 16_777_216.0_f32)
    }
}

/// Brent's GRAND state: U is retained between calls.
struct Grand {
    rng: Lcg32,
    u: f32,
}
impl Grand {
    fn new(seed: u32) -> Self {
        Self { rng: Lcg32 { state: seed }, u: 0.0 }
    }

    fn next(&mut self) -> f32 {
        let mut a = 0.0_f32;
        let mut i = 0_usize;

        // Inspect the successive leading one-bits of the saved uniform U.
        loop {
            self.u += self.u;
            if self.u < 1.0 { break; }
            self.u -= 1.0;
            i += 1;
            a -= D[i - 1];
        }

        // On rejection, return to label 20, not to label 10.
        loop {
            let w = D[i] * self.u;
            let mut v = w * (0.5_f32 * w - a);
            let accepted = loop {
                self.u = self.rng.uniform();
                if v <= self.u { break true; }
                v = self.rng.uniform();
                if self.u > v { continue; }
                self.u = (v - self.u) / (1.0_f32 - self.u);
                break false;
            };
            if !accepted { continue; } // Fortran GO TO 20.
            self.u = (self.u - v) / (1.0_f32 - v);
            self.u += self.u;
            if self.u < 1.0 { return a - w; }
            self.u -= 1.0;
            return w - a;
        }
    }
}

fn usage() -> ! {
    eprintln!("Usage: calgo488 <unsigned-seed-0..4294967295> <count-1..1000000>");
    std::process::exit(2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 { usage(); }
    let seed: u32 = args[1].parse().unwrap_or_else(|_| usage());
    let count: usize = args[2].parse().unwrap_or_else(|_| usage());
    if count == 0 || count > 1_000_000 { usage(); }
    let mut grand = Grand::new(seed);
    for _ in 0..count {
        println!("{:.7}", grand.next());
    }
}
