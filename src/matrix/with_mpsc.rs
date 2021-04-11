use std::sync::mpsc;
use std::thread;

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

fn multiply_rows_2(a: Vec<Vec<u32>>, b: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let size = a[0].len();
    let target = size;
    let middle = target / 2;

    let first_half = 0..middle;
    let a_first = a.clone();
    let b_first = b.clone();

    let (tx, rx) = mpsc::channel();
    let tx_copy = tx.clone();
    thread::spawn(move || {
        let mut first_matrix = vec![];
        for r in first_half {
            let mut row = vec![];
            for row_b in b_first.iter() {
                let mut sum = 0;
                for i in 0..size {
                    sum += a_first[r][i] * row_b[i];
                }
                row.push(sum);
            }
            first_matrix.push(row);
        }
        tx_copy.send(first_matrix).unwrap();
    });

    let second_half = middle..target;
    thread::spawn(move || {
        let mut second_matrix = vec![];
        for r in second_half {
            let mut row = vec![];
            for row_b in b.iter() {
                let mut sum = 0;
                for i in 0..size {
                    sum += a[r][i] * row_b[i];
                }
                row.push(sum);
            }
            second_matrix.push(row);
        }
        tx.send(second_matrix).unwrap();
    });

    let mut result = vec![];
    while let Ok(received) = rx.recv() {
        result.extend(received);
    }
    result
}

fn multiply_rows_4(a: Vec<Vec<u32>>, b: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let size = a[0].len();
    let target = size;
    let quarter = target / 4;

    let (tx, rx) = mpsc::channel();

    let first = 0..quarter;
    let a_clone = a.clone();
    let b_clone = b.clone();
    let tx_copy1 = tx.clone();
    thread::spawn(move || {
        let mut matrix = vec![];
        for r in first {
            let mut row = vec![];
            for row_b in b_clone.iter() {
                let mut sum = 0;
                for i in 0..size {
                    sum += a_clone[r][i] * row_b[i];
                }
                row.push(sum);
            }
            matrix.push(row);
        }
        tx_copy1.send(matrix).unwrap();
    });

    let second = quarter..quarter * 2;
    let a_clone = a.clone();
    let b_clone = b.clone();
    let tx_copy2 = tx.clone();
    thread::spawn(move || {
        let mut matrix = vec![];
        for r in second {
            let mut row = vec![];
            for row_b in b_clone.iter() {
                let mut sum = 0;
                for i in 0..size {
                    sum += a_clone[r][i] * row_b[i];
                }
                row.push(sum);
            }
            matrix.push(row);
        }
        tx_copy2.send(matrix).unwrap();
    });

    let third = quarter * 2..quarter * 3;
    let a_clone = a.clone();
    let b_clone = b.clone();
    let tx_copy3 = tx.clone();
    thread::spawn(move || {
        let mut matrix = vec![];
        for r in third {
            let mut row = vec![];
            for row_b in b_clone.iter() {
                let mut sum = 0;
                for i in 0..size {
                    sum += a_clone[r][i] * row_b[i];
                }
                row.push(sum);
            }
            matrix.push(row);
        }
        tx_copy3.send(matrix).unwrap();
    });

    let fourth = quarter * 3..target;
    let a_clone = a.clone();
    let b_clone = b.clone();
    thread::spawn(move || {
        let mut matrix = vec![];
        for r in fourth {
            let mut row = vec![];
            for row_b in b_clone.iter() {
                let mut sum = 0;
                for i in 0..size {
                    sum += a_clone[r][i] * row_b[i];
                }
                row.push(sum);
            }
            matrix.push(row);
        }
        tx.send(matrix).unwrap();
    });

    let mut result = vec![];
    while let Ok(received) = rx.recv() {
        result.extend(received);
    }
    result
}

pub fn split_to_ranges(target: u32, thread: u32) -> Vec<std::ops::Range<u32>> {
    let mut ranges: Vec<std::ops::Range<u32>> = Vec::new();
    let x = target / thread;
    for i in 0..thread {
        ranges.push(x * i..x * (i + 1));
    }
    ranges
}

// fn multiply_rows_threaded(a: Vec<Vec<u32>>, b: Vec<Vec<u32>>) -> Vec<Vec<u32>> {}

fn multiply_matrix(a: Vec<Vec<u32>>, b: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    multiply_rows_2(a, rotate_matrix(b))
}

fn multiply_matrix_thread(thread: usize, a: Vec<Vec<u32>>, b: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = vec![vec![]];
    if thread == 2 {
        result = multiply_rows_2(a, rotate_matrix(b));
    } else if thread == 4 {
        result = multiply_rows_4(a, rotate_matrix(b));
    }
    result
}

pub fn calculate_2(size: usize) -> Vec<Vec<u32>> {
    let a = generate_matrix(size);
    let b = generate_matrix(size);

    multiply_rows_2(a, rotate_matrix(b))
}

pub fn calculate_4(size: usize) -> Vec<Vec<u32>> {
    let a = generate_matrix(size);
    let b = generate_matrix(size);

    multiply_rows_4(a, rotate_matrix(b))
}

pub fn calculate(size: usize) -> Vec<Vec<u32>> {
    let a = generate_matrix(size);
    let b = generate_matrix(size);

    multiply_matrix_thread(4, a, b)
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

// #[test]
// fn test_multipy_rows() {
//     let a = vec![vec![1, 2, 3]];
//     let b = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
//     let expected = vec![vec![30, 36, 42]];
//     assert_eq!(expected, multiply_rows(a, rotate_matrix(b.clone())));

//     let a = vec![vec![4, 5, 6]];
//     let expected = vec![vec![66, 81, 96]];
//     assert_eq!(expected, multiply_rows(a, rotate_matrix(b.clone())));

//     let a = vec![vec![7, 8, 9]];
//     let expected = vec![vec![102, 126, 150]];
//     assert_eq!(expected, multiply_rows(a, rotate_matrix(b.clone())));
// }
#[test]
fn test_multipy_matrix() {
    let a = generate_matrix(2);
    let b = generate_matrix(2);
    let expected = vec![vec![7, 10], vec![15, 22]];
    assert_eq!(expected, multiply_matrix(a, b));

    let a = generate_matrix(3);
    let b = generate_matrix(3);
    let expected = vec![vec![30, 36, 42], vec![66, 81, 96], vec![102, 126, 150]];
    assert_eq!(expected, multiply_matrix(a, b));

    let a = generate_matrix(4);
    let b = generate_matrix(4);
    let expected = vec![
        vec![90, 100, 110, 120],
        vec![202, 228, 254, 280],
        vec![314, 356, 398, 440],
        vec![426, 484, 542, 600],
    ];
    assert_eq!(expected, multiply_matrix(a, b));
}

#[test]
fn test_calculate() {
    let expected = vec![vec![7, 10], vec![15, 22]];
    assert_eq!(expected, calculate(2));

    let expected = vec![vec![30, 36, 42], vec![66, 81, 96], vec![102, 126, 150]];
    assert_eq!(expected, calculate(3));

    let expected = vec![
        vec![90, 100, 110, 120],
        vec![202, 228, 254, 280],
        vec![314, 356, 398, 440],
        vec![426, 484, 542, 600],
    ];
    assert_eq!(expected, calculate(4));
}
