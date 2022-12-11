use std::fs;

fn main() {
    let file_path = "./input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split: Vec<&str> = contents.split("\n").collect();
    let mut stacks: Vec<Vec<char>> = Vec::with_capacity(100);
    let mut index = 0;
    let mut length = 0;

    for (ind, s) in split.iter().enumerate() {
        if s.chars().nth(1).unwrap() == '1' {
            index = ind;
            length = s.split("   ").count();
            break;
        }
    }

    for _ in 0..length {
        stacks.push(Vec::with_capacity(100));
    }

    for s in split[0..index].iter().rev() {
        let str = s.chars().collect::<Vec<char>>();
        let mut pnt = 1;
        let mut count = 0;
        while count < length {
            if str[pnt].is_alphabetic() {
                stacks[count].push(str[pnt]);
            }
            count += 1;
            pnt += 4;
        }
    }

    for s in split[(index + 2)..].iter() {
        let split: Vec<&str> = s.split(" ").collect();
        let count = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap() - 1;
        let to = split[5].parse::<usize>().unwrap() - 1;

        // 1 2 3 4 5 6 7
        let move_from_index = stacks[from].len() - count;
        let iter: Vec<char> = stacks[from].drain(move_from_index..).collect();

        for ch in iter {
            stacks[to].push(ch);
        }
    }

    let mut final_str: String = String::from("");

    for i in 0..length {
        final_str.push(stacks[i].pop().unwrap());
    }

    println!("{:?} {}", final_str, length);
}
