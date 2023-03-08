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

fn most_repeated_value(input: &[i32]) -> Result<i32, &str> {
    if input.is_empty() {
        return Err("Cannot calculate the most repeated value of an empty vector");
    }

    let mut values = HashMap::new();
    for &value in input {
        let count = values.entry(value).or_insert(0);
        // de-reference the mutable reference count to get access to the value that it refers to
        *count += 1;
    }

    let mut max_count = 0;
    let mut most_repeated_value = 0;
    // iterate over the keys and counts as references as we only need to read their values
    for (&key, &count) in &values {
        if count > max_count {
            most_repeated_value = key;
            max_count = count;
        }

    }
    return Ok(most_repeated_value);
}

fn main() {
    let random_values = vec![1, 2, 3, 34, 2, 1];
    let median = compute_median(&random_values);
    println!("{:?}", random_values);
    println!("{:?}", median.unwrap());
}

#[cfg(test)]
mod tests {
    use crate::most_repeated_value;

    use super::compute_median;

    #[test]
    fn test_compute_median() {
        assert_eq!(compute_median(&vec![1, 3, 3, 6, 7, 8, 9]), Ok(6.0));
        assert_eq!(compute_median(&vec![1, 2, 3, 4, 5, 6, 8, 9]), Ok(4.5));
        assert_eq!(
            compute_median(&vec![]),
            Err("Cannot calculate median of an empty vector")
        );
    }

    #[test]
    fn test_most_repeated_value() {
        assert_eq!(
            most_repeated_value(&vec![]),
            Err("Cannot calculate the most repeated value of an empty vector")
        );
        assert_eq!( most_repeated_value(&vec![1,2,3,3,4,4,4]), Ok(4));
        assert_eq!( most_repeated_value(&vec![1]), Ok(1));
    }
}
