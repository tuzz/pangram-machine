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

    pangram_machine(&constants, &profiles, |solution| {
        println!("EUREKA! {:?}", solution);
        println!("{}\n", decode_solution(&initial_text, &solution));
    });

    println!("Finished running.");
}
