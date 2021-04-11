use super::*;
use crossbeam::crossbeam_channel::unbounded;
use std::ops::Range;
use std::{thread, time};

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
    // println!("{:?}", ranges);
    let (snd, rcv) = unbounded();
    crossbeam::scope(|s| {
        let mut id = 0;
        for range in ranges.into_iter() {
            let (snd, rcv) = (snd.clone(), rcv.clone());
            s.spawn(move |_| {
                let new_block = multiply_with_range(range, a, b);
                snd.send((id, new_block)).unwrap();
            });
            id += 1;
        }
    })
    .unwrap();

    let mut received = vec![];
    for _ in 0..thread {
        let recv = rcv.recv().unwrap();
        received.push(recv);
    }

    received.sort_by_key(|tuple| tuple.0);

    let mut result = vec![];
    for tuple in received {
        result.extend(tuple.1);
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
    let b = a.clone();
    let expected = vec![vec![7, 10], vec![15, 22]];
    assert_eq!(expected, multiply_matrix(1, &a, &b));
    assert_eq!(expected, multiply_matrix(2, &a, &b));

    let a = super::generate_matrix_u64(3);
    let b = a.clone();
    let expected = vec![vec![30, 36, 42], vec![66, 81, 96], vec![102, 126, 150]];
    assert_eq!(expected, multiply_matrix(1, &a, &b));
    assert_eq!(expected, multiply_matrix(3, &a, &b));

    let a = super::generate_matrix_u64(4);
    let b = a.clone();
    let expected = vec![
        vec![90, 100, 110, 120],
        vec![202, 228, 254, 280],
        vec![314, 356, 398, 440],
        vec![426, 484, 542, 600],
    ];
    assert_eq!(expected, multiply_matrix(1, &a, &b));
    assert_eq!(expected, multiply_matrix(2, &a, &b));
    assert_eq!(expected, multiply_matrix(3, &a, &b));
    assert_eq!(expected, multiply_matrix(4, &a, &b));

    let a = super::generate_matrix_u64(5);
    let b = a.clone();
    let expected = vec![
        vec![215, 230, 245, 260, 275],
        vec![490, 530, 570, 610, 650],
        vec![765, 830, 895, 960, 1025],
        vec![1040, 1130, 1220, 1310, 1400],
        vec![1315, 1430, 1545, 1660, 1775],
    ];
    assert_eq!(expected, multiply_matrix(1, &a, &b));
    assert_eq!(expected, multiply_matrix(2, &a, &b));
    assert_eq!(expected, multiply_matrix(3, &a, &b));
    assert_eq!(expected, multiply_matrix(4, &a, &b));
    assert_eq!(expected, multiply_matrix(5, &a, &b));
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

    let expected = vec![
        vec![215, 230, 245, 260, 275],
        vec![490, 530, 570, 610, 650],
        vec![765, 830, 895, 960, 1025],
        vec![1040, 1130, 1220, 1310, 1400],
        vec![1315, 1430, 1545, 1660, 1775],
    ];

    assert_eq!(expected, calculate(1, 5));
    assert_eq!(expected, calculate(2, 5));
    assert_eq!(expected, calculate(3, 5));
    assert_eq!(expected, calculate(4, 5));
}
