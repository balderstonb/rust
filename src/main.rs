use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let v = vec![1, 1, 2, 3, 3, 4, 4, 4];

    println!("mean = {}", mean(&v));
    println!("median = {}", median(&v));
    println!("mode = {}", mode(&v));
}

fn mean(vals: &[u32]) -> f32 {
    let mut sum = 0;
    let size = vals.len() as u32;
    for i in vals {
        sum = sum + i
    }
    (sum as f32 / size as f32)
}

fn median(vals: &[u32]) -> f32 {
    let sorted = vals.to_vec();
    let size = vals.len() as u32;
    let mid = size / 2 as u32;

    if size == 0 {
        return sorted[0] as f32;
    } else if size % 2 == 0 {
        return (sorted[mid as usize] + sorted[(mid - 1) as usize]) as f32 / 2 as f32;
    } else {
        return sorted[mid as usize] as f32;
    }
}

fn mode<T>(vals: &[T]) -> &T
where
    T: Eq + Hash,
{
    let mut counts = HashMap::new();

    for i in vals {
        *counts.entry(i).or_insert(0) += 1;
    }

    let mut count_vec: Vec<_> = counts.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    *count_vec[0].0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mode() {
        let v = vec![1, 1, 2, 3, 3, 4, 4, 4];
        assert_eq!(4, *mode(&v));
    }

    #[test]
    fn test_mean() {
        let v = vec![1, 5, 12];
        assert_eq!(6.0, mean(&v));
    }

    #[test]
    fn test_median_even() {
        let v = vec![1, 2];
        assert_eq!(1.5, mean(&v));
    }

    #[test]
    fn test_median_odd() {
        let v = vec![3, 1, 2];
        assert_eq!(2.0, mean(&v));
    }
}
