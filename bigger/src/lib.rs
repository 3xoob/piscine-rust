pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max_value = i32::MIN;
    for &value in h.values() {
        if value > max_value {
            max_value = value;
        }
    }
    max_value
}
