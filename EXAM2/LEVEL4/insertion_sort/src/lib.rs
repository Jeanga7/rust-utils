pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    (slice.split_at_mut(steps+1).0).sort()
}