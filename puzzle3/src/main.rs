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
    let file_path = "./input2";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split("\n");
    let mut total = 0;
    let a_byte_code = get_ascii_code("a");
    let z_byte_code: i32 = get_ascii_code("z");
    let _a_byte_code = get_ascii_code("A");
    // let Zbyte_code = get_ascii_code("Z");
    let mut set: HashSet<i32> = HashSet::with_capacity(100);
    for (i, s) in split.enumerate() {
        let s_set = get_hashset_string(s);
        if i % 3 == 0 {
            set = s_set;
        } else if i % 3 == 1 {
            set = set.intersection(&s_set).cloned().collect();
        } else {
            set = set.intersection(&s_set).cloned().collect();
            println!("{:?}", set);
            let ascii = *(set.iter().nth(0).unwrap());
            let res: i32;
            if a_byte_code <= ascii && z_byte_code >= ascii {
                res = ascii - a_byte_code + 1;
            } else {
                res = ascii - _a_byte_code + 27;
            }
            total += res;
        }
    }
    println!("{}", total);
}
