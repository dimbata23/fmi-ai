use rand::{Rng, prelude::SliceRandom};
use lazy_static::lazy_static;
use std::collections::{HashMap, BTreeSet};

// TODO: impl struct Individual for BTreeSet

fn main() {
    let res_ind = genetic_algorithm( 1000, fitness ).unwrap();
    println!("{:?} with road len: {}", res_ind, fitness( &res_ind ));
}

fn genetic_algorithm(init_size: usize, fitness_func: fn(&Individual) -> u32) -> Option<Individual> {
    let     range       = ['A', 'B', 'C', 'D', 'E', 'F'];
    let mut rng         = rand::thread_rng();
    let mut population  = BTreeSet::new();
    for _ in 0..init_size {
        let mut ind     = range.clone();
        ind.shuffle( &mut rng );
        population.insert( ind );
    }

    // TODO: replace with real end condition
    for _ in 0..100 {
        let mut new_population  = BTreeSet::new();

        for _ in &population {
            let par_i   = rng.gen_range( 0..population.len() );
            let par1    = population.iter().skip( par_i ).next().unwrap();
            let par_j   = rng.gen_range( 0..population.len() );
            let par2    = population.iter().skip( par_j ).next().unwrap();

            let (mut child1, mut child2)    = reproduce( &par1, &par2 );
            if rng.gen_bool( 0.05 ) {
                child1  = mutate( &child1 );
            }

            if rng.gen_bool( 0.05 ) {
                child2  = mutate( &child2 );
            }
            new_population.insert( child1 );
            new_population.insert( child2 );
        }

        population  = new_population;
    }

    println!("{:?}", population.iter().take( 10 ).map( fitness_func ).collect::<Vec<_>>() );

    population.into_iter().min_by( |i, j| fitness_func( i ).cmp( &fitness_func( j ) ) )
}

fn reproduce(lhs: &Individual, rhs: &Individual) -> (Individual, Individual) {
    let mut rng     = rand::thread_rng();
    let     pos     = rng.gen_range( 1..ISIZE-1 );

    let mut lres    = lhs.clone();
    let mut rres    = rhs.clone();

    let (_, lcross) = lres.split_at_mut( pos );
    let (_, rcross) = rres.split_at_mut( pos );

    lcross.iter_mut()
        .zip( rcross )
        .for_each(
            |(lch, rch)| std::mem::swap( lch, rch )
        );

    (make_unique( lres ), make_unique( rres ))
}

fn mutate(ind: &Individual) -> Individual {
    let mut rng     = rand::thread_rng();
    let mut pos_i   = rng.gen_range( 0..ISIZE );
    let mut pos_j   = rng.gen_range( 0..ISIZE );
    let mut res_ind = ind.clone();

    if pos_i == pos_j {
        if pos_i > 0 {
            pos_i -= 1;
        } else {
            pos_j = 1;
        }
    }

    if pos_i > pos_j {
        std::mem::swap( &mut pos_i, &mut pos_j );
    }

    let (lhs, rhs)  = res_ind.split_at_mut( pos_j );
    std::mem::swap( &mut lhs[ pos_i ], &mut rhs[ 0 ] );

    res_ind
}

fn fitness(ind: &Individual) -> u32 {
    let mut res = 0;
    for i in 0..(ISIZE - 1) {
        let curr    = &ind[i];
        let next    = &ind[i + 1];
        res += GRAPH[ curr ][ next ];
    }

    let last    = ind.last().unwrap();
    let first   = ind.first().unwrap();
    res + GRAPH[ last ][ first ]
}

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

const ISIZE: usize = 6;
type Individual = [char; ISIZE];



/// ---------------------------
/// ---- Utility functions ----
/// ---------------------------

fn make_unique(ind: Individual) -> Individual {
    let     range       = ['A', 'B', 'C', 'D', 'E', 'F'];
    let mut arr_used    = [false; ISIZE];
    let mut res_ind     = ind.clone();

    res_ind.iter_mut()
        .for_each(|ch| {
            let i = *ch as usize - 'A' as usize;
            if arr_used[ i ] {
                for i in 0..ISIZE {
                    if !arr_used[ i ] {
                        arr_used[ i ] = true;
                        *ch = range[ i ];
                        break;
                    }
                }
            }
            else {
                arr_used[ i ] = true;
            }
        });

    res_ind
}

#[cfg(test)]
mod tests;
