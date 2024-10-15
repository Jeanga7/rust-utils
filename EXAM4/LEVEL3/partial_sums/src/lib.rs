pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    let mut current_sum: u64 = arr.iter().sum();

    result.push(current_sum);
    
    for &value in arr.iter().rev() {
        current_sum -= value;
        result.push(current_sum);
    }
    result
}