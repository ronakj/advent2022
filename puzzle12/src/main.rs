use std::{collections::HashSet, fs};

fn get_ascii(c: char) -> usize {
    return c as usize;
}

fn get_shortest_distance(
    height_map: &Vec<Vec<usize>>,
    possible_starts: &Vec<(usize, usize)>,
    end: (usize, usize),
) -> usize {
    let mut distance_map = vec![vec![10000000; height_map[0].len()]; height_map.len()];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    distance_map[end.0][end.1] = 0;
    let mut current = end;

    while visited.len() != height_map.len() * height_map[0].len() {
        let mut possible_neighbors: Vec<(usize, usize)> = Vec::with_capacity(4);
        if current.0 != 0 {
            possible_neighbors.push((current.0 - 1, current.1));
        }
        if current.1 != 0 {
            possible_neighbors.push((current.0, current.1 - 1));
        }
        if current.0 != height_map.len() - 1 {
            possible_neighbors.push((current.0 + 1, current.1));
        }
        if current.1 != height_map[0].len() - 1 {
            possible_neighbors.push((current.0, current.1 + 1));
        }
        for neighbor in possible_neighbors {
            if visited.contains(&neighbor) {
                continue;
            }
            let current_height = height_map[current.0][current.1];
            if current_height <= (height_map[neighbor.0][neighbor.1] + 1) {
                let current_distance = distance_map[current.0][current.1];
                let neighbor_distance = distance_map[neighbor.0][neighbor.1];
                if (current_distance + 1) < neighbor_distance {
                    distance_map[neighbor.0][neighbor.1] = current_distance + 1;
                }
            }
        }
        visited.insert((current.0, current.1));
        let mut min_distance = usize::MAX;
        for (i, row) in height_map.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                if !visited.contains(&(i, j)) && distance_map[i][j] < min_distance {
                    min_distance = distance_map[i][j];
                    current = (i, j);
                }
            }
        }
    }
    let mut shortest_distance = usize::MAX;
    for start in possible_starts {
        if distance_map[start.0][start.1] < shortest_distance {
            shortest_distance = distance_map[start.0][start.1];
        }
    }
    return shortest_distance;
}

fn calculate_fewest_steps(input_file: &str) -> (usize, usize) {
    let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let mut height_map: Vec<Vec<usize>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    let mut possible_starts: Vec<(usize, usize)> = Vec::new();
    let mut end: (usize, usize) = (0, 0);
    while let Some(line) = lines.next() {
        let mut row: Vec<usize> = Vec::new();
        for c in line.chars() {
            if c == 'S' || c == 'a' {
                if c == 'S' {
                    start = (height_map.len(), row.len());
                }
                possible_starts.push((height_map.len(), row.len()));
                row.push(get_ascii('a'));
            } else if c == 'E' {
                end = (height_map.len(), row.len());
                row.push(get_ascii('z'));
            } else {
                row.push(get_ascii(c));
            }
        }
        height_map.push(row);
    }
    return (
        get_shortest_distance(&height_map, &Vec::from([start]), end),
        get_shortest_distance(&height_map, &possible_starts, end),
    );
}

fn main() {
    assert_eq!(calculate_fewest_steps("test_input"), (31, 29));
    println!("{:?}", calculate_fewest_steps("input"));
}
