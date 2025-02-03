use rand::Rng;

pub struct Candidate {
    pub value: i64,
    pub fitness_level: f64,
}

impl Candidate {
    pub fn new(value: i64) -> Candidate {
        Candidate {
            value,
            fitness_level: Candidate::fitness(value),
        }
    }

    pub fn fitness(s: i64) -> f64 {
        let s = Self::conv_int_to_bstring(s);
        let x: i64 = Self::conv_bstring_to_int(&s[0..8]);
        let y: i64 = Self::conv_bstring_to_int(&s[8..16]);
    
        //let mut fitness_level: f64 = (((1-x)^2) as f64) * E.powi((-(x^2) - (y+1)^2) as i32) - ((x - x^3 - y^3) as f64) * E.powi((-(x^2) -(y^2)) as i32);
    
        let mut fitness_level: f64 = (1000 - x - y + x * y) as f64;
    
        if fitness_level < 0.0 {fitness_level = 0.0;}
        fitness_level
    }

    pub fn breed(first: Candidate, second: Candidate, crossover_constant: f64, mutation_constant: f64) -> (Candidate, Candidate) {
        let mut rng = rand::thread_rng();
    
        let binary_first = Self::conv_int_to_bstring(first.value);
        let binary_second = Self::conv_int_to_bstring(second.value);
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
            first_child = Candidate::new(Self::conv_bstring_to_int(format!("{part_one}{part_two}")));
            second_child = Candidate::new(Self::conv_bstring_to_int(format!("{part_two}{part_one}")));
        }
    
        println!("{:?} , {:?} => {:?} {:?}", Self::split_bstring_to_two(binary_first), Self::split_bstring_to_two(binary_second), Self::split_bstring_to_two(Self::conv_int_to_bstring(first_child.value)), Self::split_bstring_to_two(Self::conv_int_to_bstring(second_child.value)));
        
        (first_child, second_child)
    }

    pub fn conv_bstring_to_int(bin: impl Into<String>) -> i64 {
        i64::from_str_radix(&bin.into(), 2).unwrap()
    }
    
    pub fn conv_int_to_bstring(int: i64) -> String {
        format!("{:016b}", int)
    }
    
    /*fn determine_fitness_of_gen(population: &Vec<i64>) -> HashMap<i64, f64> {
        let mut population_fitness: HashMap<i64, f64> = HashMap::new();
        for elem in population {
            population_fitness.insert(*elem, fitness(*elem));
        }
        population_fitness
    }*/
    
    pub fn split_bstring_to_two(s: String) -> (i64, i64) {
        let x: i64 = Self::conv_bstring_to_int(&s[0..8]);
        let y: i64 = Self::conv_bstring_to_int(&s[8..16]);
        (x,y)
    }
}