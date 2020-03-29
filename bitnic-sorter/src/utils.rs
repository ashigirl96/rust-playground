use crate::SortOrder;
use crate::SortOrder::*;
use rand::distributions::Standard;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

pub fn old_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64::from_seed([0; 32]);

    let mut v = Vec::with_capacity(n);

    for _ in 0..n {
        v.push(rng.sample(&Standard));
    }
    v
}

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    let mut rng = Pcg64::from_seed([0; 32]);

    rng.sample_iter(&Standard).take(n).collect()
}

pub fn is_sorted<T: Ord>(x: &[T], ord: &SortOrder) -> bool {
    match *ord {
        Ascending => is_sorted_ascending(x),
        Descending => is_sorted_descending(x),
    }
}

pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_descending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_rnd_vec() {
        let mut x = new_u32_vec(8);
        println!("{:#?}", x);
        assert_eq!(x.len(), 8);
    }

    #[test]
    fn windows() {
        let mut x = new_u32_vec(4);
        eprintln!("x = {:#?}", x);
        for window in x.windows(2) {
            eprintln!("window = {:#?}", window);
        }
    }

    #[test]
    fn test_is_sorted() {
        let mut x = vec![0, 1, 3, 4];
        let is_sorted_ = is_sorted(&mut x, &Ascending);
        eprintln!("is_sorted = {:#?}", is_sorted_);

        x.reverse();
        let is_desc_sorted = is_sorted(&mut x, &Descending);
        eprintln!("is_desc_sorted = {:#?}", is_desc_sorted);

        let mut y = vec![0, 1, 5, 4];
        let is_not_sorted = is_sorted(&mut y, &Ascending);
        eprintln!("is_not_sorted = {:#?}", is_not_sorted);
    }
}
