use crate::candidate::*;
use rand::Rng;
use rand::distributions::WeightedIndex;
use rand::prelude::*;
use rand::thread_rng;

pub struct Generation {
    pub population_count: usize,
    pub population: Vec<Candidate>,
}

impl Generation {
    pub fn initial_gen(population_count: usize) -> Generation {
        let mut rng = rand::thread_rng();
        
        let mut vec = Vec::with_capacity(population_count as usize);
        for _ in 0..population_count {
            let random = rng.gen_range(0..131072);
            vec.push(Candidate::new(random));
        }
        Generation{population_count, population : vec}
    }

    pub fn pick_candidates_for_breeding(&self) -> (Candidate, Candidate) {
        let mut rng = thread_rng();
        
        let keys: Vec<i64> = (&self.population).into_iter().map(|c| c.value).collect();
        let weights: Vec<f64> = (&self.population).into_iter().map(|c| c.fitness_level).collect();
        
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

    pub fn generate_new(&self, crossover_constant: f64, mutation_constant: f64) -> Generation {
        //let old_population_with_fitness = determine_fitness_of_gen(population);
        let mut new_generation = Vec::new();
    
        for _ in 0..&self.population_count/2 {
            let (first, second) = Generation::pick_candidates_for_breeding(&self);
            
            let (first_child, second_child) = Candidate::breed(first, second, crossover_constant, mutation_constant);
    
            new_generation.push(first_child);
            new_generation.push(second_child);
        }
    
        Generation{population_count: self.population_count, population: new_generation}
    }

}