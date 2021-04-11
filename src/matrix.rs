extern crate bencher;
extern crate criterion;

use rand::Rng;

pub mod benchers;
pub mod criterions;
pub mod normal;
pub mod with_crossbeam;
pub mod with_mpsc;
pub mod with_mpsc2;
pub mod with_mutex;

pub fn generate_matrix(size: usize) -> Vec<Vec<u32>> {
    let mut value = 1;
    let mut matrix = vec![];
    for _ in 0..size {
        let mut row = vec![];
        for _ in 0..size {
            row.push(value);
            value += 1;
        }
        matrix.push(row)
    }
    matrix
}

pub fn generate_matrix_ones(size: usize) -> Vec<Vec<u64>> {
    let mut value = 1;
    let mut matrix = vec![];
    for _ in 0..size {
        let mut row = vec![];
        for _ in 0..size {
            row.push(value);
        }
        matrix.push(row)
    }
    matrix
}

pub fn generate_matrix_rand(size: usize) -> Vec<Vec<u32>> {
    let mut rng = rand::thread_rng();
    let mut matrix = vec![];
    for _ in 0..size {
        let mut row = vec![];
        for _ in 0..size {
            row.push(rng.gen::<u32>());
        }
        matrix.push(row)
    }
    matrix
}

pub fn generate_matrix_u64(size: usize) -> Vec<Vec<u64>> {
    let mut value = 1;
    let mut matrix = vec![];
    for _ in 0..size {
        let mut row = vec![];
        for _ in 0..size {
            row.push(value);
            value += 1;
        }
        matrix.push(row)
    }
    matrix
}

fn is_it_half(a: u64, b: u64) -> bool {
    (a as f64 / b as f64) * 10.0 % 10.0 == 5.0
}

pub fn split_number(target: u64, thread: u64) -> Vec<std::ops::Range<u64>> {
    let mut ranges: Vec<std::ops::Range<u64>> = Vec::new();

    let mut x = 0;
    if is_it_half(target, thread) {
        x = target / thread
    } else {
        x = (target as f64 / thread as f64).round() as u64;
    }

    for i in 0..thread {
        if i == thread - 1 && x * (i + 1) != target {
            ranges.push(x * i..target);
        } else {
            ranges.push(x * i..x * (i + 1));
        }
    }
    ranges
}

#[test]
fn test_generate_matrix() {
    let expected = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(expected, generate_matrix(2));

    let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    assert_eq!(expected, generate_matrix(3));

    let expected = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
        vec![13, 14, 15, 16],
    ];
    assert_eq!(expected, generate_matrix(4));
}
