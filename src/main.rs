use std::collections::HashMap;
use std::fmt::Result;
use std::io::Result as IoResult;

use rand::Rng;

// fn function1() -> Result {
//     // --snip--
// }

// fn function2() -> IoResult<()> {
//     // --snip--
// }

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{}", secret_number);
}
