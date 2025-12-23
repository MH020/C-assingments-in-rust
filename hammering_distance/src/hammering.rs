pub fn hamming_distance(dna1: &str, dna2: &str ) -> u32{

    if dna1.len() != dna2.len() {
       return 0; 
    }

    let mut distance = 0; 

    for i in 0..dna1.len() {
        if dna1.as_bytes()[i] != dna2.as_bytes()[i] {
            distance += 1;
        }
    }
    distance
}