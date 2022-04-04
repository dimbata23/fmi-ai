use rand::{Rng, prelude::SliceRandom};
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

const ISIZE: usize = 6;
type Individual = [char; ISIZE];

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

fn main() {
    let mut rng     = rand::thread_rng();
    let     range   = ['A', 'B', 'C', 'D', 'E', 'F'];
    let mut ind1    = range.clone();
    ind1.shuffle(&mut rng);
    let mut ind2    = range.clone();
    ind2.shuffle(&mut rng);

    println!("{:?} + {:?}", ind1, ind2);

    println!("{:?}", fitness(&ind1));
    println!("{:?}", fitness(&ind2));
    println!("{:?}", reproduce(&ind1, &ind2));

}



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
