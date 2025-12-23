mod perms; 
use crate::perms::{USER_EXECUTE, USER_READ, USER_WRITE, fmt_perms, NyPermsT};

fn main() {
    let _hellotxt =  NyPermsT  {bits: USER_READ + USER_WRITE + USER_EXECUTE};

    let perms: String = fmt_perms(_hellotxt);

    println!("hello.txt har rettighederne {}", perms)
} 
