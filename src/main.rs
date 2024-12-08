mod initial_text_constants;
mod number_profiles;
mod pairwise_sum;
mod pangram_machine;

use initial_text_constants::*;
use number_profiles::*;
use pangram_machine::*;
use std::io::{stdin, stdout, Write};
use std::process::exit;

fn main() {
    print!("Enter intial text: ");
    let mut initial_text = String::new();
    stdout().flush().unwrap();
    stdin().read_line(&mut initial_text).unwrap();

    let profiles = number_profiles();
    let constants = initial_text_constants(&initial_text, &profiles);
    println!("Initial constants: {:?}", constants);
    println!("\nRunning Pangram Machine Mark II...");

    let mut num_solutions = 0;
    pangram_machine(&constants, &profiles, |solution| {
        println!("EUREKA! {:?}", solution);
        num_solutions += 1;
    });

    if num_solutions == 0 {
        println!("Unfortunately, no solutions were found.");
        exit(1);
    }
}
