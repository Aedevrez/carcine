use std::f64::consts::E;
use rand::Rng;
use std::collections::HashMap;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::thread_rng;

fn main() {
    let population_count: usize = 10;
    let crossover_constant = 87.0; //percent
    let mutation_constant = 1.0; //percent
    let population = generate_initial_gen(population_count);
    let population_fitness = determine_fitness_of_gen(&population);
    for (elem, elem_fitness) in &population_fitness {
        println!("{elem} : {elem_fitness}")
    }
    for _ in 0..5 {
        let (first, second) = pick_candidates_for_breeding(&population_fitness);
        println!("{first} {second}");
    }
}

fn fitness(s: i64) -> f64 {
    let s = conv_int_to_bstring(s);
    let x: i64 = conv_bstring_to_int(&s[0..8]);
    let y: i64 = conv_bstring_to_int(&s[8..16]);

    let mut fitness_level: f64 = (((1-x)^2) as f64) * E.powi((-(x^2) - (y+1)^2) as i32) - ((x - x^3 - y^3) as f64) * E.powi((-(x^2) -(y^2)) as i32);

    if fitness_level < 0.0 {fitness_level = 0.0;}
    fitness_level
}

fn generate_initial_gen(population_count: usize) -> Vec<i64> {
    let mut rng = rand::thread_rng();
    
    let mut vec = Vec::with_capacity(population_count as usize);
    for _ in 0..population_count {
        let random = rng.gen_range(0..131072);
        vec.push(random);
    }
    vec
}

fn conv_bstring_to_int(bin: impl Into<String>) -> i64 {
    i64::from_str_radix(&bin.into(), 2).unwrap()
}

fn conv_int_to_bstring(int: i64) -> String {
    format!("{:016b}", int)
}

fn determine_fitness_of_gen(population: &Vec<i64>) -> HashMap<i64, f64> {
    let mut population_fitness: HashMap<i64, f64> = HashMap::new();
    for elem in population {
        population_fitness.insert(*elem, fitness(*elem));
    }
    population_fitness
}

fn pick_candidates_for_breeding(population: &HashMap<i64, f64>) -> (i64, i64) {
    let mut rng = thread_rng();
    
    let keys: Vec<i64> = population.keys().cloned().collect();
    let weights: Vec<f64> = population.values().cloned().collect();
    
    let dist = WeightedIndex::new(&weights).unwrap();
    
    let first = keys[dist.sample(&mut rng)];
    let mut second;
    /*loop {
        second = keys[dist.sample(&mut rng)];
        if second != first {
            break;
        }
    }*/
    second = keys[dist.sample(&mut rng)];
    
    (first, second)
}
