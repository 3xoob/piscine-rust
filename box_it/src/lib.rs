pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let numbers: Vec<&str> = s.split_whitespace().collect();

    let transformed: Vec<u32> = numbers
        .iter()
        .map(|&num| {
            if num.ends_with('k') {
                num[..num.len() - 1]
                    .parse::<f64>()
                    .unwrap_or(0.0)
                    .mul(1000.0)
                    .round() as u32
            } else {
                num.parse::<f64>().unwrap_or(0.0).round() as u32
            }
        })
        .collect();

    Box::new(transformed)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
