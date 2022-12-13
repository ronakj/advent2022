// a -> [1, 2, 3], b-> [5]

use std::fs;

// a -> [1,3,[4,5,[6],7]]
fn get_items(string_to_parse: String) -> Vec<String> {
    let mut items = Vec::new();
    if string_to_parse.starts_with("[") {
        let string_to_parse = string_to_parse[1..string_to_parse.len() - 1].to_string();
        let mut item = String::new();
        let mut depth = 0;
        for c in string_to_parse.chars() {
            if c == '[' {
                depth += 1;
            } else if c == ']' {
                depth -= 1;
            }
            if depth == 0 && c == ',' {
                items.push(item.clone());
                item = String::new();
            } else {
                item.push(c);
            }
        }
        if item.len() > 0 {
            items.push(item.clone());
        }
    } else {
        items.push(string_to_parse);
    }
    return items;
}

#[derive(PartialEq, Debug)]
enum Result {
    Right,
    Wrong,
    Equal,
}

fn compare(left: String, right: String) -> Result {
    // println!("input: {} {}", left, right);
    if !left.starts_with("[") && !right.starts_with("[") {
        let left_int = left.parse::<i32>().unwrap();
        let right_int = right.parse::<i32>().unwrap();
        if left_int == right_int {
            return Result::Equal;
        }
        if left_int < right_int {
            return Result::Right;
        }
        return Result::Wrong;
    }
    let left_items = get_items(left.clone());
    let right_items = get_items(right.clone());
    // println!("{} {} {:?} {:?}", left, right, left_items, right_items);
    for i in 0..left_items.len() {
        if i >= right_items.len() {
            return Result::Wrong;
        }
        let left_item = left_items[i].clone();
        let right_item = right_items[i].clone();
        let result = compare(left_item, right_item);
        if result == Result::Wrong {
            return Result::Wrong;
        }
        if result == Result::Right {
            return Result::Right;
        }
    }
    if left_items.len() < right_items.len() {
        return Result::Right;
    }
    return Result::Equal;
}

fn process_input_p1(input_file: String) -> i32 {
    let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");
    let lines: Vec<&str> = contents.lines().collect();
    let mut count: i32 = 0;
    let mut i = 0;
    while i * 3 < lines.len() - 1 {
        let left = lines[i * 3].to_string();
        let right = lines[i * 3 + 1].to_string();
        let result = compare(left.clone(), right.clone());
        // println!("{} {} {:?}", left, right, result);
        if result == Result::Right {
            count += (i + 1) as i32;
        }
        i += 1;
    }
    return count;
}

fn process_input_p2(input_file: String) -> i32 {
    let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");
    let mut lines: Vec<&str> = contents.lines().filter(|l| !l.trim().is_empty()).collect();
    let decoder1 = "[[2]]";
    let decoder2 = "[[6]]";
    lines.push(decoder1);
    lines.push(decoder2);
    for i in 0..lines.len() {
        for j in 0..lines.len() - 1 - i {
            if compare(lines[j].to_string(), lines[j + 1].to_string()) == Result::Wrong {
                let temp = lines[j];
                lines[j] = lines[j + 1];
                lines[j + 1] = temp;
            }
        }
    }

    let mut product = 1;
    for i in 0..lines.len() {
        if lines[i] == decoder1 {
            product *= (i + 1) as i32;
        }
        if lines[i] == decoder2 {
            product *= (i + 1) as i32;
        }
    }
    return product;
}

fn main() {
    assert_eq!(process_input_p1("test_input".to_string()), 13);
    assert_eq!(process_input_p2("test_input".to_string()), 140);
    println!("P1: {}", process_input_p1("input".to_string()));
    println!("P2: {}", process_input_p2("input".to_string()));
}
