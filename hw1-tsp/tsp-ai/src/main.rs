use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref GRAPH: HashMap<char, HashMap<char, u32>> = HashMap::from([
        ('A', HashMap::from([('A', 00), ('B', 36), ('C', 32), ('D', 54), ('E', 20), ('F', 40)])),
        ('B', HashMap::from([('A', 36), ('B', 00), ('C', 22), ('D', 58), ('E', 54), ('F', 67)])),
        ('C', HashMap::from([('A', 32), ('B', 22), ('C', 00), ('D', 36), ('E', 42), ('F', 71)])),
        ('D', HashMap::from([('A', 54), ('B', 58), ('C', 36), ('D', 00), ('E', 50), ('F', 92)])),
        ('E', HashMap::from([('A', 20), ('B', 54), ('C', 42), ('D', 50), ('E', 00), ('F', 45)])),
        ('F', HashMap::from([('A', 40), ('B', 67), ('C', 71), ('D', 92), ('E', 45), ('F', 00)])),
    ]);
}

fn fitness(ind: &str) -> u32 {
    let mut res = 0;
    for i in 0..(ind.len() - 1) {
        let curr    = &ind.chars().nth( i ).unwrap();
        let next    = &ind.chars().nth( i+1 ).unwrap();
        res += GRAPH[ curr ][ next ];
    }

    let last    = &ind.chars().last().unwrap();
    let first   = &ind.chars().next().unwrap();
    res + GRAPH[ last ][ first ]
}

fn main() {
    
    println!("{}", fitness("ABCDEF"));

}
