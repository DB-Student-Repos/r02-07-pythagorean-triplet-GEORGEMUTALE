use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // Create a HashSet to store the triplets
    let mut triplets = HashSet::new();
    
    // Iterate over possible values of 'a'
    for a in 1..=sum / 3 {
        // Calculate the denominator and numerator based on the parametrization
        let denom = 2 * (sum - a);
        let num = sum * sum - 2 * a * sum;
        
        // Check if the numerator is divisible by the denominator
        if num % denom == 0 {
            // Calculate 'b' from the division
            let b = num / denom;
            
            // Check if 'a' is less than 'b'
            if a < b {
                // Insert the triplet into the HashSet
                triplets.insert([a, b, sum - a - b]);
            }
        }
    }

    // Return the HashSet containing all found Pythagorean triplets
    triplets
}
