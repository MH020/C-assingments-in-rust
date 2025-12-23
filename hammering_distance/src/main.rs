mod hammering;

fn main() {
    let dna1 = "GAGCCTACTAACGGGAT";
    let dna2 = "CATCGTAATGACGGCCT";

    let exprected_distance = 7;
    let actual_distance = hammering::hamming_distance(dna1, dna2);

    println!("String 1: {}", dna1);
    println!("String 2: {}", dna2);
    println!("Distance: {}", actual_distance);

    println!(
        "Is that correct? {}",
        if exprected_distance == actual_distance {
            "Oh yas!"
        } else {
            "No :("
        }
    ); 
}
