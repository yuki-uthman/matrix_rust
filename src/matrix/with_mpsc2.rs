use std::ops::Range;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

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

fn multiply_with_range(range: Range<u64>, a: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let col_size = a[0].len();
    let row_size = a.len();

    let mut new_block = vec![];
    for thread_index in range {
        // each block for thread
        // do sth here
        let mut new_row = vec![];
        for row_index in 0..row_size {
            let mut element = 0;
            for index in 0..col_size {
                element += a[thread_index as usize][index] * b[index][row_index]
            }
            new_row.push(element);
        }
        new_block.push(new_row);
    }
    new_block
}

fn multiply_matrix(thread: u64, a: &Vec<Vec<u64>>, b: &Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let ranges = split_number(*&a.len() as u64, thread);
    let (tx, rx) = mpsc::channel();
    let mut children = Vec::new();

    let mut id = 0;
    for range in ranges {
        let thread_tx = tx.clone();
        let a = a.clone();
        let b = b.clone();
        let child = thread::spawn(move || {
            let new_block = multiply_with_range(range, &a, &b);
            // println!("sending {:?}", new_block);
            thread_tx.send((id, new_block)).unwrap();
        });

        children.push(child);
        id += 1;
    }

    let mut received = vec![];
    for _ in 0..thread {
        let recv = rx.recv().unwrap();
        received.push(recv);
    }

    received.sort_by_key(|tuple| tuple.0);

    let mut result = vec![];
    for tuple in received {
        result.extend(tuple.1);
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    result
}

pub fn calculate(thread: u64, size: usize) -> Vec<Vec<u64>> {
    let a = super::generate_matrix_u64(size);
    let b = a.clone();

    multiply_matrix(thread, &a, &b)
}

#[test]
fn test_multipy_matrix() {
    let a = super::generate_matrix_u64(2);
    let b = super::generate_matrix_u64(2);
    let expected = vec![vec![7, 10], vec![15, 22]];
    assert_eq!(expected, multiply_matrix(1, &a, &b));

    let a = super::generate_matrix_u64(3);
    let b = super::generate_matrix_u64(3);
    let expected = vec![vec![30, 36, 42], vec![66, 81, 96], vec![102, 126, 150]];
    assert_eq!(expected, multiply_matrix(1, &a, &b));
    assert_eq!(expected, multiply_matrix(3, &a, &b));

    let a = super::generate_matrix_u64(4);
    let b = super::generate_matrix_u64(4);
    let expected = vec![
        vec![90, 100, 110, 120],
        vec![202, 228, 254, 280],
        vec![314, 356, 398, 440],
        vec![426, 484, 542, 600],
    ];
    assert_eq!(expected, multiply_matrix(1, &a, &b));
    assert_eq!(expected, multiply_matrix(2, &a, &b));
    assert_eq!(expected, multiply_matrix(4, &a, &b));
}

#[test]
fn test_calculate() {
    let expected = vec![vec![7, 10], vec![15, 22]];
    assert_eq!(expected, calculate(1, 2));
    assert_eq!(expected, calculate(2, 2));

    let expected = vec![vec![30, 36, 42], vec![66, 81, 96], vec![102, 126, 150]];
    assert_eq!(expected, calculate(1, 3));
    assert_eq!(expected, calculate(3, 3));

    let expected = vec![
        vec![90, 100, 110, 120],
        vec![202, 228, 254, 280],
        vec![314, 356, 398, 440],
        vec![426, 484, 542, 600],
    ];
    assert_eq!(expected, calculate(1, 4));
    assert_eq!(expected, calculate(2, 4));
    assert_eq!(expected, calculate(3, 4));
    assert_eq!(expected, calculate(4, 4));
}
