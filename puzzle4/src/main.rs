use std::fs;

fn get_num_tuple(s: &str) -> (i32, i32) {
    let mut pair = s.split("-");
    let first = pair.next().unwrap().parse::<i32>().unwrap();
    let second = pair.next().unwrap().parse::<i32>().unwrap();
    (first, second)
}

// 1 - 4 3 - 6
fn check_overlap(first: (i32, i32), second: (i32, i32)) -> bool {
    if first.0 <= second.0 {
        return first.1 >= second.0;
    }
    if second.0 <= first.0 {
        return second.1 >= first.0;
    }
    return false;
}

fn main() {
    let file_path = "./input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split("\n");
    let mut total = 0;

    for s in split {
        let mut pairs = s.split(",");
        let first_pair = get_num_tuple(pairs.next().unwrap());
        let second_pair = get_num_tuple(pairs.next().unwrap());
        if check_overlap(first_pair, second_pair) {
            total += 1
        }
    }
    println!("{}", total);
}
