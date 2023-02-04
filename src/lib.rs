use std::cmp;

/// Distance returns the Levenstein distance between strings a and b
pub fn distance(a: &str, b: &str) -> usize {
    single_row_distance(a, b)
}

// Compute the Levenshtein distance between strings a and b
#[allow(dead_code)]
fn naive_distance(a: &str, b: &str) -> usize {
    if a.len() == 0 {
        return b.len();
    }

    if b.len() == 0 {
        return a.len();
    }

    if a.chars().nth(0) == b.chars().nth(0) {
        return naive_distance(&a[1..], &b[1..]);
    }

    1 + min3(
        naive_distance(&a[1..], &b),
        naive_distance(&a, &b[1..]),
        naive_distance(&a[1..], &b[1..]),
    )
}

// matrix_distance computes the Levenstein distance of two words without recursion
#[allow(dead_code)]
fn matrix_distance(a: &str, b: &str) -> usize {
    if a.len() == 0 {
        return b.len();
    }

    if b.len() == 0 {
        return a.len();
    }

    // Produces a grid in the form
    // 0 1 2 3 4
    // 1 0 0 0 0
    // 2 0 0 0 0
    // 3 0 0 0 0
    let mut matrix = vec![0; (a.len() + 1) * (b.len() + 1)];
    let index = |x: usize, y: usize| -> usize { (y * (a.len() + 1)) + x };

    for x in 0..=a.len() {
        // Populate top row
        matrix[index(x, 0)] = x;
    }

    for y in 0..=b.len() {
        matrix[index(0, y)] = y;
    }

    for y in 1..=b.len() {
        for x in 1..=a.len() {
            if a.chars().nth(x - 1) == b.chars().nth(y - 1) {
                matrix[index(x, y)] = matrix[index(x - 1, y - 1)]
            } else {
                matrix[index(x, y)] = 1 + min3(
                    matrix[index(x - 1, y)],
                    matrix[index(x, y - 1)],
                    matrix[index(x - 1, y - 1)],
                )
            }
        }
    }

    return matrix[matrix.len() - 1];
}

// Double row distance performs the same calulation as matrix_distance, but swaps between
// only two rows, rather than building and maintaining the entire grid.
#[allow(dead_code)]
fn double_row_distance(a: &str, b: &str) -> usize {
    if a.len() == 0 {
        return b.len();
    }

    if b.len() == 0 {
        return a.len();
    }

    let mut row1 = vec![0; a.len() + 1];
    let mut row2 = vec![0; a.len() + 1];

    // TODO: is there a way to do this with an iterator?
    for i in 0..row2.len() {
        row2[i] = i;
    }

    for y in 0..b.len() {
        (row1, row2) = (row2, row1);
        row2[0] = y + 1;

        for x in 0..a.len() {
            if a.chars().nth(x) == b.chars().nth(y) {
                row2[x + 1] = row1[x];
            } else {
                row2[x + 1] = 1 + min3(row1[x + 1], row1[x], row2[x]);
            }
        }
    }

    row2[row2.len() - 1]
}

// single_row_distance produces the same results as double_row_distance with one array,
// and one temporary variable
fn single_row_distance(a: &str, b: &str) -> usize {
    if a.len() == 0 {
        return b.len();
    }

    if b.len() == 0 {
        return a.len();
    }

    let mut row = vec![0; a.len() + 1];
    for i in 0..row.len() {
        row[i] = i
    }

    let mut last;

    for y in 0..b.len() {
        (last, row[0]) = (row[0], y + 1);
        for x in 0..a.len() {
            // println!("a: {}", a);
            if a.chars().nth(x) == b.chars().nth(y) {
                (last, row[x + 1]) = (row[x + 1], last);
            } else {
                // println!("row len:{}", row.len());
                let tmp = last;
                last = row[x + 1];
                row[x + 1] = 1 + min3(tmp, row[x], row[x + 1]);
            }
        }
    }

    row[a.len()]
}

fn min3<T: std::cmp::Ord>(a: T, b: T, c: T) -> T {
    cmp::min(cmp::min(a, b), c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // matrix_distance("asdasdff", "aaa");
        // return;
        // TODO: Can we make this a module-level constant shared across many tests?
        let test_cases = [
            ("fast", "past", 1),
            ("foo", "bar", 3),
            ("fab", "bar", 2),
            ("aaa", "bbb", 3),
            ("aaaaaa", "bbb", 6),
            ("aaa", "bbbbbb", 6),
            ("aaaaaa", "bbbbbb", 6),
            ("ababab", "bababa", 2), // 1 deltion (left-shifting everything) and 1 insertion
            ("aaaaa", "baaaaa", 1),  // 1 insertion
            ("aabaa", "aaaa", 1),    // 1 deletion
        ];

        for tc in test_cases.iter() {
            let naive_result = naive_distance(tc.0, tc.1);
            assert_eq!(
                naive_result, tc.2,
                "naive_distance({}, {}) - got {}, want {}",
                tc.0, tc.1, naive_result, tc.2
            );

            let matrix_result = matrix_distance(tc.0, tc.1);
            assert_eq!(
                matrix_result, tc.2,
                "matrix_distance({}, {}) - got {}, want {}",
                tc.0, tc.1, matrix_result, tc.2
            );

            let double_row_result = double_row_distance(tc.0, tc.1);
            assert_eq!(
                double_row_result, tc.2,
                "double_row_distance({}, {}) - got {}, want {}",
                tc.0, tc.1, double_row_result, tc.2
            );

            let single_row_result = single_row_distance(tc.0, tc.1);
            assert_eq!(
                single_row_result, tc.2,
                "single_row_distance({}, {}) - got {}, want {}",
                tc.0, tc.1, single_row_result, tc.2
            );
        }
    }
}
