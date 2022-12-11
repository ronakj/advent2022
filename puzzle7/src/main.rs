use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq)]
enum EntryType {
    DIR,
    FILE,
}

#[derive(Debug)]
struct FileEntry {
    name: String,
    entry_type: EntryType,
    size: i32,
}

fn join(first: &String, second: &String) -> String {
    if first == "/" {
        return format!("{}{}", first, second);
    } else {
        return format!("{}/{}", first, second);
    }
}

fn calculate_size(
    container_map: &HashMap<String, Vec<FileEntry>>,
    dir: &String,
    size_map: &mut HashMap<String, i32>,
) -> i32 {
    let mut size: i32 = 0;
    println!("Calculating size for {}", dir);
    let entries: &Vec<FileEntry> = container_map.get(dir).unwrap();
    for entry in entries {
        if entry.entry_type == EntryType::DIR {
            if size_map.contains_key(&entry.name) {
                size += size_map.get(&entry.name).unwrap();
            } else {
                size += calculate_size(container_map, &join(dir, &entry.name), size_map);
            }
        } else {
            size += entry.size;
        }
    }
    size_map.insert(dir.clone(), size);
    size
}

fn main() {
    let mut container_map: HashMap<String, Vec<FileEntry>> = HashMap::with_capacity(10000);
    let mut current_dir: String = "/".to_string();
    let file_path = "./input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split("\n");

    for s in split {
        if s.starts_with("$") {
            let s_split: Vec<&str> = s.split(" ").collect();
            if s_split[1] == "ls" {
                println!("Listing for {}", current_dir);
                continue;
            }
            let dir = s.split(" ").last().unwrap();
            if dir == "/" {
                current_dir = "/".to_string();
            } else if dir == ".." {
                let splits: Vec<&str> = current_dir.split("/").collect();
                if splits.len() == 2 {
                    current_dir = "/".to_string();
                } else {
                    current_dir = splits[0..splits.len() - 1].join("/");
                }
            } else {
                current_dir = join(&current_dir, &dir.to_string());
            }
        } else {
            if !container_map.contains_key(&current_dir) {
                container_map.insert(current_dir.clone(), Vec::new());
            }
            if s.starts_with("dir") {
                let dir_name = join(&current_dir, &s.split(" ").last().unwrap().to_string());
                (container_map.get_mut(&current_dir).unwrap()).push(FileEntry {
                    name: s.split(" ").last().unwrap().to_string(),
                    entry_type: EntryType::DIR,
                    size: 0,
                });
                if !container_map.contains_key(&dir_name) {
                    container_map.insert(dir_name.clone(), Vec::new());
                }
            } else {
                (container_map.get_mut(&current_dir).unwrap()).push(FileEntry {
                    name: s.split(" ").last().unwrap().to_string(),
                    entry_type: EntryType::FILE,
                    size: s.split(" ").next().unwrap().parse().unwrap(),
                });
            }
        }
    }
    let mut size_map: HashMap<String, i32> = HashMap::with_capacity(10000);

    calculate_size(&container_map, &"/".to_string(), &mut size_map);

    // 40000000 - totalUsed + deletion > 0
    let total_used: i32 = *size_map.get("/").unwrap();
    let mut min_value: i32 = 70000000;

    for (a, value) in size_map.iter() {
        if (40000000 - total_used) + value > 0 && min_value > *value {
            min_value = *value;
        }
    }

    println!("{}", min_value);
}
