use std::collections::HashSet;

pub fn find(n: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    for a in 1..(n / 3) {
        for b in (a + 1)..(n / 2) {
            let c = n - a - b;
            if a * a + b * b == c * c {
                triplets.insert([a, b, c]);
            }
        }
    }
    triplets
}
