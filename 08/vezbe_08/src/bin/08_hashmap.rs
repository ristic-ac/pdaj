use std::collections::HashMap;

fn calculate_median_mode(numbers: &mut [i32]) -> (f32, i32) {
    // Sort for median calculation
    numbers.sort();
    let median = if numbers.len() % 2 == 0 {
        let mid = numbers.len() / 2;
        (numbers[mid - 1] + numbers[mid]) as f32 / 2.0
    } else {
        numbers[numbers.len() / 2] as f32
    };

    // Mode calculation using a hash map
    let mut occurrences = HashMap::new();
    for num in numbers {
        *occurrences.entry(num).or_insert(0) += 1;
    }
    let mode = occurrences.into_iter().max_by_key(|&(_, count)| count).map(|(num, _)| num).unwrap();

    (median, *mode)
}

fn main() {
    let mut numbers = vec![3, 1, 4, 4, 2];
    let (median, mode) = calculate_median_mode(&mut numbers);
    println!("Median: {}, Mode: {}", median, mode);
}
