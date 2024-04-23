use std::collections::HashSet;
pub fn find(m: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    let sum = m as i32;
    for a in 1i32..sum {
        for b in a..sum {
            let c = sum - b - a;
            if c > b && (a.pow(2) + b.pow(2) == c.pow(2)) {
                triplets.insert([a as u32, b as u32, c as u32]);
            }
        }
    }
    triplets
}