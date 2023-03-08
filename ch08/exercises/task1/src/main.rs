// Given a list of integers, use a vector and return the median (when sorted, the value in the
// middle position) and mode (the value that occurs most often; a hash map will be helpful here)
// of the list.
use std::collections::HashMap;

fn compute_median(input: &[i32]) -> Result<f32, &str> {
    if input.is_empty() {
        return Err("Cannot calculate median of an empty vector");
    }
    let mut sorted = input.to_vec();
    sorted.sort();

    let middle_mark = input.len() / 2;
    if sorted.len() % 2 == 0 {
        return Ok((sorted[(middle_mark - 1)] + input[(middle_mark)]) as f32 / 2.0);
    } else {
        return Ok(sorted[middle_mark] as f32);
    }
}

fn compute_mode(input: &[i32]) -> Result<Vec<i32>, &str> {
    if input.is_empty() {
        return Err("Cannot calculate the most repeated value of an empty vector");
    }

    let mut values = HashMap::new();
    for &value in input {
        let count = values.entry(value).or_insert(0);
        // de-reference the mutable reference count to get access to the value that it refers to
        *count += 1;
    }

    let max_count = *values.values().max().unwrap();
    let mut mode: Vec<i32> = vec![];
    for (key, count) in values {
        if count == max_count {
            mode.push(key);
        }
    }
    return Ok(mode);
}

fn main() {
    let random_values = vec![1, 2, 3, 34, 2, 1];
    let median = compute_median(&random_values);
    println!("{:?}", random_values);
    println!("{:?}", median.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::compute_mode;

    use super::compute_median;

    #[test]
    fn test_compute_median() {
        assert_eq!(compute_median(&vec![1, 3, 3, 6, 7, 8, 9]), Ok(6.0));
        assert_eq!(compute_median(&vec![9, 8, 7, 1, 3, 3, 6]), Ok(6.0));
        assert_eq!(compute_median(&vec![1, 2, 3, 4, 5, 6, 8, 9]), Ok(4.5));
        assert_eq!(
            compute_median(&vec![]),
            Err("Cannot calculate median of an empty vector")
        );
    }

    #[test]
    fn test_compute_mode() {
        assert_eq!(
            compute_mode(&vec![]),
            Err("Cannot calculate the most repeated value of an empty vector")
        );
        assert_eq!(compute_mode(&vec![1, 2, 3, 3, 4, 4, 4]), Ok(vec![4]));
        assert_eq!(compute_mode(&vec![1, 2, 3, 3, 4, 4]), Ok(vec![3, 4]));
        assert_eq!(compute_mode(&vec![1]), Ok(vec![1]));
    }
}
