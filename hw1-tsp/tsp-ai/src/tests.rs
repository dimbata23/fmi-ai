use super::*;

#[test]
fn test_fitness() {
    let def_ind = ['A', 'B', 'C', 'D', 'E', 'F'];
    let rev_ind = ['F', 'E', 'D', 'C', 'B', 'A'];
    let ran_ind = ['C', 'F', 'B', 'E', 'A', 'D'];
    assert_eq!( fitness( &def_ind ), 229 );
    assert_eq!( fitness( &rev_ind ), 229 );
    assert_eq!( fitness( &ran_ind ), 302 );
}

#[test]
fn test_reproduce_correctness() {
    let mut rng         = rand::thread_rng();
    let     range       = ['A', 'B', 'C', 'D', 'E', 'F'];
    let     range_vec   = range.iter().collect::<Vec<_>>();

    for _ in 0..100000 {
        let mut ind1        = range.clone();
        let mut ind2        = range.clone();
        ind1.shuffle( &mut rng );
        ind2.shuffle( &mut rng );

        let (res1, res2)    = reproduce( &ind1, &ind2 );
        let mut vec1        = res1.iter().collect::<Vec<_>>();
        let mut vec2        = res2.iter().collect::<Vec<_>>();
        vec1.sort();
        vec2.sort();
        assert_eq!( &vec1, &range_vec );
        assert_eq!( &vec2, &range_vec );
    }
}
