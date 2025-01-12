#[allow(unused_imports)]
use std::f64::consts::E;
#[allow(unused_imports)]
use std::collections::HashMap;
use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::thread_rng;

struct Candidate {
    value: i64,
    fitness_level: f64,
}

impl Candidate {
    pub fn new(value: i64) -> Candidate {
        Candidate {
            value,
            fitness_level: Candidate::fitness(value),
        }
    }

    pub fn fitness(s: i64) -> f64 {
        let s = conv_int_to_bstring(s);
        let x: i64 = conv_bstring_to_int(&s[0..8]);
        let y: i64 = conv_bstring_to_int(&s[8..16]);
    
        //let mut fitness_level: f64 = (((1-x)^2) as f64) * E.powi((-(x^2) - (y+1)^2) as i32) - ((x - x^3 - y^3) as f64) * E.powi((-(x^2) -(y^2)) as i32);
    
        let mut fitness_level: f64 = (1000 - x - y + x * y) as f64;
    
        if fitness_level < 0.0 {fitness_level = 0.0;}
        fitness_level
    }
}

fn main() {
    let population_count: usize = 10;
    let crossover_constant = 0.87; //percent
    let mutation_constant = 0.1; //percent
    let population = generate_initial_gen(population_count);
    //let population_fitness = determine_fitness_of_gen(&population);
    let generation_count = 20;
    for candidate in &population {
        println!("{:?} : {}", split_bstring_to_two(conv_int_to_bstring(candidate.value)), candidate.fitness_level);
    }

    let mut new_generation= generate_new_generation(&population, population_count, crossover_constant, mutation_constant);

    for i in 0..generation_count {
        println!("Generation {i}");
        new_generation = generate_new_generation(&new_generation, population_count, crossover_constant, mutation_constant);
    }

}



fn generate_initial_gen(population_count: usize) -> Vec<Candidate> {
    let mut rng = rand::thread_rng();
    
    let mut vec = Vec::with_capacity(population_count as usize);
    for _ in 0..population_count {
        let random = rng.gen_range(0..131072);
        vec.push(Candidate::new(random));
    }
    vec
}

fn conv_bstring_to_int(bin: impl Into<String>) -> i64 {
    i64::from_str_radix(&bin.into(), 2).unwrap()
}

fn conv_int_to_bstring(int: i64) -> String {
    format!("{:016b}", int)
}

/*fn determine_fitness_of_gen(population: &Vec<i64>) -> HashMap<i64, f64> {
    let mut population_fitness: HashMap<i64, f64> = HashMap::new();
    for elem in population {
        population_fitness.insert(*elem, fitness(*elem));
    }
    population_fitness
}*/

fn pick_candidates_for_breeding(population: &Vec<Candidate>) -> (Candidate, Candidate) {
    let mut rng = thread_rng();
    
    let keys: Vec<i64> = (&population).into_iter().map(|c| c.value).collect();
    let weights: Vec<f64> = (&population).into_iter().map(|c| c.fitness_level).collect();
    
    let dist = WeightedIndex::new(&weights).unwrap();
    
    let first = Candidate::new(keys[dist.sample(&mut rng)]);

    #[allow(unused_mut)]
    let mut second;
    /*loop {
        second = keys[dist.sample(&mut rng)];
        if second != first {
            break;
        }
    }*/
    second = Candidate::new(keys[dist.sample(&mut rng)]);
    
    (first, second)
}

fn breed(first: Candidate, second: Candidate, crossover_constant: f64, mutation_constant: f64) -> (Candidate, Candidate) {
    let mut rng = rand::thread_rng();

    let binary_first = conv_int_to_bstring(first.value);
    let binary_second = conv_int_to_bstring(second.value);
    let mut binary_couple = format!("{binary_first}{binary_second}");
    let mut first_child = first;
    let mut second_child= second;

    if rng.gen_range(0.0..1.0) < mutation_constant {
        let pivot = rng.gen_range(0..31);
        let mutated_char = match &binary_couple.chars().nth(pivot).unwrap() {
            '0' => "1",
            '1' => "0",
            _ => panic!("Unexpected character during mutation")
        };
        binary_couple.replace_range(pivot..pivot+1, mutated_char);
    }

    if rng.gen_range(0.0..1.0) < crossover_constant {
        let pivot = rng.gen_range(0..32);
        let part_one = &binary_couple[0..pivot];
        let part_two = &binary_couple[pivot..32];
        first_child = Candidate::new(conv_bstring_to_int(format!("{part_one}{part_two}")));
        second_child = Candidate::new(conv_bstring_to_int(format!("{part_two}{part_one}")));
    }

    println!("{:?} , {:?} => {:?} {:?}", split_bstring_to_two(binary_first), split_bstring_to_two(binary_second), split_bstring_to_two(conv_int_to_bstring(first_child.value)), split_bstring_to_two(conv_int_to_bstring(second_child.value)));
    
    (first_child, second_child)
}

fn generate_new_generation(population: &Vec<Candidate>, population_count: usize, crossover_constant: f64, mutation_constant: f64) -> Vec<Candidate> {
    //let old_population_with_fitness = determine_fitness_of_gen(population);
    let mut new_generation = Vec::new();

    for _ in 0..population_count/2 {
        let (first, second) = pick_candidates_for_breeding(&population);
        
        let (first_child, second_child) = breed(first, second, crossover_constant, mutation_constant);

        new_generation.push(first_child);
        new_generation.push(second_child);
    }

    new_generation
}

fn split_bstring_to_two(s: String) -> (i64, i64) {
    let x: i64 = conv_bstring_to_int(&s[0..8]);
    let y: i64 = conv_bstring_to_int(&s[8..16]);
    (x,y)
}