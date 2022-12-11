use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn calculate_scenic_scope(
    dir: (i32, i32),
    trees_list: &Vec<Vec<u32>>,
    scenic_score: &mut HashMap<(u32, u32), u32>,
) {
    let mut current_index: (i32, i32) = (0, 0);
    // 1 0, 0 1, -1 0, 0 -1
    if dir.0 == -1 {
        current_index.0 = trees_list.len() as i32 - 1;
    }
    if dir.1 == -1 {
        current_index.1 = trees_list[0].len() as i32 - 1;
    }
    let start_index = current_index.clone();
    loop {
        if current_index.0 < 0 || current_index.0 == trees_list.len() as i32 {
            current_index.0 = start_index.0.clone();
            current_index.1 += 1;
            if current_index.1 >= trees_list[0].len() as i32 {
                break;
            }
        }
        if current_index.1 < 0 || current_index.1 == trees_list[0].len() as i32 {
            current_index.1 = start_index.1.clone();
            current_index.0 += 1;
            if current_index.0 >= trees_list.len() as i32 {
                break;
            }
        }
        let current_index_u32 = (current_index.0 as u32, current_index.1 as u32);
        let mut count = 0;
        let mut visible_index = (current_index.0 - dir.0, current_index.1 - dir.1);
        while visible_index.0 >= 0
            && visible_index.0 < trees_list.len() as i32
            && visible_index.1 >= 0
            && visible_index.1 < trees_list[1].len() as i32
        {
            count += 1;
            if trees_list[current_index.0 as usize][current_index.1 as usize]
                <= trees_list[visible_index.0 as usize][visible_index.1 as usize]
            {
                break;
            }
            visible_index = (visible_index.0 - dir.0, visible_index.1 - dir.1)
        }
        current_index = (current_index.0 + dir.0, current_index.1 + dir.1);
        let score = scenic_score.get(&current_index_u32).unwrap_or(&1);
        scenic_score.insert(current_index_u32, score * count);
    }
}

fn check_and_add_visible(
    dir: (i32, i32),
    trees_list: &Vec<Vec<u32>>,
    visible_set: &mut HashSet<(u32, u32)>,
) {
    let mut current_index: (i32, i32) = (0, 0);
    // 1 0, 0 1, -1 0, 0 -1
    if dir.0 == -1 {
        current_index.0 = trees_list.len() as i32 - 1;
    }
    if dir.1 == -1 {
        current_index.1 = trees_list[0].len() as i32 - 1;
    }
    let start_index = current_index.clone();
    let mut max = 0;
    loop {
        if current_index.0 < 0 || current_index.0 == trees_list.len() as i32 {
            current_index.0 = start_index.0.clone();
            current_index.1 += 1;
            if current_index.1 >= trees_list[0].len() as i32 {
                break;
            }
            max = 0;
        }
        if current_index.1 < 0 || current_index.1 == trees_list[0].len() as i32 {
            current_index.1 = start_index.1.clone();
            current_index.0 += 1;
            if current_index.0 >= trees_list.len() as i32 {
                break;
            }
            max = 0
        }
        if trees_list[current_index.0 as usize][current_index.1 as usize] > max {
            visible_set.insert((current_index.0 as u32, current_index.1 as u32));
            max = trees_list[current_index.0 as usize][current_index.1 as usize];
        }
        if max == 0
            && (current_index.0 == 0
                || (current_index.0 == trees_list.len() as i32 - 1)
                || current_index.1 == 0
                || (current_index.1 == trees_list[0].len() as i32 - 1))
        {
            visible_set.insert((current_index.0 as u32, current_index.1 as u32));
        }
        current_index = (current_index.0 + dir.0, current_index.1 + dir.1);
    }
}

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let trees_list: Vec<Vec<u32>> = contents
        .split("\n")
        .map(|s| s.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();

    let mut visible_set: HashSet<(u32, u32)> = HashSet::new();
    check_and_add_visible((0, 1), &trees_list, &mut visible_set);
    check_and_add_visible((1, 0), &trees_list, &mut visible_set);
    check_and_add_visible((0, -1), &trees_list, &mut visible_set);
    check_and_add_visible((-1, 0), &trees_list, &mut visible_set);

    let mut scenic_score: HashMap<(u32, u32), u32> = HashMap::new();

    calculate_scenic_scope((0, 1), &trees_list, &mut scenic_score);
    calculate_scenic_scope((1, 0), &trees_list, &mut scenic_score);
    calculate_scenic_scope((0, -1), &trees_list, &mut scenic_score);
    calculate_scenic_scope((-1, 0), &trees_list, &mut scenic_score);
    println!("{:?}", scenic_score.values().max().unwrap());
}
