pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();

    for (column_index, row) in input.iter().enumerate() {
        let max = row.iter().max();

        for (row_index, column_value) in row.iter().enumerate() {
            if column_value == max.unwrap() && is_value_smallest_in_column(&input, *column_value, row_index) {
                result.push((column_index, row_index));
            }
        }

    }

    result
}

fn is_value_smallest_in_column(input: &[Vec<u64>], value: u64, index: usize) -> bool {
    for row in input.iter() {
        if value > row[index] {
            return false;
        }
    }

    return true;
}