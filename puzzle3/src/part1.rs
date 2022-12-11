use std::collections::HashSet;
use std::fs;
// use std::hash::Hash;

fn get_ascii_code(s: &str) -> i32 {
    s.as_bytes()[0] as i32
}

fn get_hashset_string(s: &str) -> HashSet<i32> {
    let mut set: HashSet<i32> = HashSet::with_capacity(100);
    for ch in s.as_bytes() {
        set.insert(*ch as i32);
    }
    set
}

fn main() {
    // --snip--
    let file_path = "./input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split("\n");
    let mut total = 0;
    let a_byte_code = get_ascii_code("a");
    let z_byte_code: i32 = get_ascii_code("z");
    let _a_byte_code = get_ascii_code("A");
    // let Zbyte_code = get_ascii_code("Z");
    for s in split {
        // let mut set: HashSet<i32> = HashSet::with_capacity(100);
        let first_half = get_hashset_string(&s[0..(s.len() / 2)]);
        let second_half = get_hashset_string(&s[(s.len() / 2)..]);
        let mut common = first_half.intersection(&second_half);
        // println!("{:?}", common);
        let ascii = *(common.nth(0).unwrap());
        let res: i32;
        if a_byte_code <= ascii && z_byte_code >= ascii {
            res = ascii - a_byte_code + 1;
        } else {
            res = ascii - _a_byte_code + 27;
        }
        total += res;
    }
    println!("{}", total);
}
