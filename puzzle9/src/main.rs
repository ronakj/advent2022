use std::{collections::HashSet, fs};

fn get_new_tail_pos(head_pos: (i32, i32), tail_pos: (i32, i32)) -> (i32, i32) {
    if head_pos.0.abs_diff(tail_pos.0) <= 1 && head_pos.1.abs_diff(tail_pos.1) <= 1 {
        return tail_pos;
    }
    if head_pos.0.abs_diff(tail_pos.0) > 1 && head_pos.1.abs_diff(tail_pos.1) > 1 {
        if head_pos.0 > tail_pos.0 {
            if head_pos.1 > tail_pos.1 {
                return (head_pos.0 - 1, head_pos.1 - 1);
            } else {
                return (head_pos.0 - 1, head_pos.1 + 1);
            }
        } else {
            if head_pos.1 > tail_pos.1 {
                return (head_pos.0 + 1, head_pos.1 - 1);
            } else {
                return (head_pos.0 + 1, head_pos.1 + 1);
            }
        }
    } else if head_pos.0.abs_diff(tail_pos.0) > 1 {
        if head_pos.0 > tail_pos.0 {
            return (head_pos.0 - 1, head_pos.1);
        } else {
            return (head_pos.0 + 1, head_pos.1);
        }
    } else {
        if head_pos.1 > tail_pos.1 {
            return (head_pos.0, head_pos.1 - 1);
        } else {
            return (head_pos.0, head_pos.1 + 1);
        }
    }
}

fn main() {
    let file_path = "./input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split("\n");

    let mut rope_pos: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert((0, 0));

    for s in split {
        let split = s.split(" ").collect::<Vec<&str>>();
        let dir = split[0];
        let steps = split[1];
        // println!("{} {}", dir, steps);
        let steps = steps.parse::<i32>().unwrap();
        for _ in 0..steps {
            match dir {
                "U" => rope_pos[0].0 += 1,
                "D" => rope_pos[0].0 -= 1,
                "L" => rope_pos[0].1 -= 1,
                "R" => rope_pos[0].1 += 1,
                _ => panic!("Invalid direction"),
            }
            for i in 1..10 {
                rope_pos[i] = get_new_tail_pos(rope_pos[i - 1], rope_pos[i]);
            }
            // println!("{:?} {} {}", rope_pos, dir, steps);
            visited.insert(rope_pos[9]);
        }
    }
    println!("{}", visited.len());
}
