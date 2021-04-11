#![allow(warnings, unused)]

use std::ops::Range;

fn generate_matrix(size: usize) -> Vec<Vec<u32>> {
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

fn rotate_matrix(matrix: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut matrix = matrix.clone();
    let size = matrix[0].len();
    for row in 0..size {
        for column in row + 1..size {
            let temp = matrix[row][column];
            matrix[row][column] = matrix[column][row];
            matrix[column][row] = temp
        }
    }
    matrix
}

fn multiply(a: Vec<Vec<u32>>, b: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = vec![];
    let size = a[0].len();
    for row_a in a {
        let mut row = vec![];
        for row_b in b.iter() {
            let mut sum = 0;
            for i in 0..size {
                sum += row_a[i] * row_b[i];
            }
            row.push(sum);
        }
        result.push(row);
    }
    result
}

fn multiply_with_range(range: Range<u32>, a: Vec<Vec<u32>>, b: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
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

fn swap_index(size: usize) {
    for row in 0..size {
        for column in row + 1..size {
            print!("{}{} ", row, column);
        }
        println!();
    }
}

use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

static NTHREADS: i32 = 3;

fn is_it_half(a: u32, b: u32) -> bool {
    (a as f32 / b as f32) * 10.0 % 10.0 == 5.0
}

pub fn split_number(target: u32, thread: u32) -> Vec<std::ops::Range<u32>> {
    let mut ranges: Vec<std::ops::Range<u32>> = Vec::new();

    let mut x = 0;
    if is_it_half(target, thread) {
        x = target / thread
    } else {
        x = (target as f32 / thread as f32).round() as u32;
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

fn rearrange_matrix(mut a: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    a.sort_by_key(|k| *k.last().unwrap());
    let mapped = a.into_iter().map(|mut row| {
        row.pop();
        row
    });
    mapped.collect::<Vec<_>>()
}

fn rearrange_tuples(a: &mut Vec<(u32, Vec<Vec<u32>>)>) {
    a.sort_by_key(|k| k.0);
}

use std::sync::{Arc, Mutex};

fn main() {
    let mut a = generate_matrix(4);
    let mut b = a.clone();
    let ranges = split_number(4, 2);

    let tuples = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    let blocks = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    let mut id = Arc::new(Mutex::new(0));
    for range in ranges {
        let blocks = Arc::clone(&blocks);
        let id = Arc::clone(&id);
        let handle = thread::spawn(move || {
            let new_block = multiply_with_range(range, a, b);
            let mut blocks = blocks.lock().unwrap();
            let mut id = id.lock().unwrap();
            blocks.push((*id, new_block));
        });
        handles.push(handle);
        let mut id = id.lock().unwrap();
        *id += 1;
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut blocks = blocks.lock().unwrap();
    // blocks.sort_by(|a, b| a.lock().unwrap().cmp(b.0));

    // rearrange_tuples(&mut to_send);
    // tuples.lock().unwrap().sort_by_key(|tuple| tuple.0);

    // println!("{:?}", result);
}
