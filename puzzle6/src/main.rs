use std::{collections::HashSet, fs};

fn main() {
    let file_path = "./input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut i = 0;

    let num_chars_to_match = 14;
    //abcdefgh
    while i <= (contents.len() - num_chars_to_match) {
        let hash_set: HashSet<char> = contents[i..(i + num_chars_to_match)].chars().collect();
        if hash_set.len() == num_chars_to_match {
            println!("{:?}", hash_set);
            break;
        }
        i += 1;
    }
    println!("{}", i + num_chars_to_match);
}
