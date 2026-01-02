mod perms;
use perms::NyPermsT;
fn main() {

    let mut file_a = NyPermsT {bits: 0644};
    println!("test 1 before: {}", file_a.bits);
    let expect = NyPermsT {bits: 0640};
    println!("test 1 expect: {}", expect.bits);
    file_a.chmod("o-r".to_string()); 
    println!("test 1 actual: {}", file_a.bits);
}
