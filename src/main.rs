mod candidate;
mod generation;

use crate::candidate::*;
use crate::generation::*;

fn main() {
    let population_count: usize = 10;
    let crossover_constant = 0.87; //percent
    let mutation_constant = 0.1; //percent
    let generation = Generation::initial_gen(population_count);
    //let population_fitness = determine_fitness_of_gen(&population);
    let generation_count = 20;
    for candidate in &generation.population {
        println!("{:?} : {}", Candidate::split_bstring_to_two(Candidate::conv_int_to_bstring(candidate.value)), candidate.fitness_level);
    }

    let mut new_generation= Generation::generate_new(&generation, crossover_constant, mutation_constant);

    for i in 0..generation_count {
        println!("Generation {i}");
        new_generation = Generation::generate_new(&new_generation, crossover_constant, mutation_constant);
    }

}
