mod initial_text_constants;
mod number_profiles;
mod pairwise_sum;
mod pangram_machine;
mod decode_solution;

use initial_text_constants::*;
use number_profiles::*;
use pangram_machine::*;
use decode_solution::*;
use std::io::{stdin, stdout, Write};

fn main() {
    print!("Enter intial text: ");
    let mut initial_text = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut initial_text).unwrap();

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
