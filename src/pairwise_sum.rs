pub fn pairwise_sum(a: &[usize; 26], b: &[usize; 26]) -> [usize; 26] {
    let mut sum = [0; 26];

    for i in 0..26 {
        sum[i] = a[i] + b[i];
    }

    sum
}
