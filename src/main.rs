use std::f64::consts::E;
use rand::Rng;

fn main() {
    let population_count: usize = 10;
    let crossover_constant = 87.0; //percent
    let mutation_constant = 1.0; //percent
    let vec = generate_initial_gen(population_count);
    for elem in &vec {
        println!("{elem} : {}", fitness(*elem))
    }
}

fn fitness(s: i64) -> f64 {
    let s = conv_int_to_bstring(s);
    let x: i64 = conv_bstring_to_int(&s[0..8]);
    let y: i64 = conv_bstring_to_int(&s[8..16]);

    let fitness_level: f64 = (((1-x)^2) as f64) * E.powi((-(x^2) - (y+1)^2) as i32) - ((x - x^3 - y^3) as f64) * E.powi((-(x^2) -(y^2)) as i32);
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
