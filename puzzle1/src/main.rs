use std::fs;

fn main() {
    // --snip--
    let file_path = "./input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split("\n");
    let mut start = 0;
    let mut a = [0; 5000];
    let mut index = 0;
    for s in split {
        if s == "" {
            a[index] = start;
            index += 1;
            start = 0;
        } else {
            let num = s.parse::<i32>().unwrap();
            start += num;
        }
    }
    a.sort();
    a.reverse();
    println!("{}", a[0] + a[1] + a[2]);
}
