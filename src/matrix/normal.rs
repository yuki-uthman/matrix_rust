fn generate_matrix(size: usize) -> Vec<Vec<u64>> {
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

fn rotate_matrix(matrix: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
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

fn multiply_rows(a: Vec<Vec<u64>>, b: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
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

fn multiply_matrix(a: Vec<Vec<u64>>, b: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let col_size = a[0].len();
    let row_size = a.len();

    let mut result = vec![];
    for row in a.iter() {
        let mut new_row = vec![];
        for r in 0..row_size {
            let mut sum = 0;
            for c in 0..col_size {
                sum += row[c] * b[c][r];
            }
            new_row.push(sum);
        }
        result.push(new_row);
    }
    result
}

pub fn calculate(size: usize) -> Vec<Vec<u64>> {
    let a = super::generate_matrix_ones(size);
    let b = a.clone();

    multiply_matrix(a, b)
}

pub fn calculate_with_rotation(size: usize) -> Vec<Vec<u64>> {
    let a = generate_matrix(size);
    let b = generate_matrix(size);
    multiply_rows(a, rotate_matrix(b))
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

#[test]
fn test_calculate_with_rotation() {
    let expected = vec![vec![7, 10], vec![15, 22]];
    assert_eq!(expected, calculate_with_rotation(2));

    let expected = vec![vec![30, 36, 42], vec![66, 81, 96], vec![102, 126, 150]];
    assert_eq!(expected, calculate_with_rotation(3));

    let expected = vec![
        vec![90, 100, 110, 120],
        vec![202, 228, 254, 280],
        vec![314, 356, 398, 440],
        vec![426, 484, 542, 600],
    ];
    assert_eq!(expected, calculate_with_rotation(4));
}
