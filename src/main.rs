#![feature(portable_simd)]

mod decode_solution;
mod initial_text_constants;
mod number_profiles;
mod pangram_machine;
mod profile;

use decode_solution::*;
use initial_text_constants::*;
use number_profiles::*;
use pangram_machine::*;
use std::io::{stdin, stdout, Write, IsTerminal};
use std::time::Instant;

fn main() {
    print!("Enter intial text: ");
    let mut initial_text = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut initial_text).unwrap();
    if !stdin().is_terminal() { print!("{}", initial_text); }

    let profiles = number_profiles();
    let constants = initial_text_constants(&initial_text, &profiles);
    println!("Initial constants: {:?}", constants);
    println!("\nRunning Pangram Machine Mark II...");

    let just_before = Instant::now();
    let num_iterations = pangram_machine(&constants, &profiles, |solution| {
        println!("EUREKA! {:?}", solution);
        println!("{}\n", decode_solution(&initial_text, &solution));
    });

    let time_taken = just_before.elapsed().as_secs_f64();
    let megahertz = num_iterations as f64 / time_taken / 1_000_000.0;
    println!("Finished {} iterations in {:.2} seconds which is equivalent to {:.1} MHz.", num_iterations, time_taken, megahertz);
}
