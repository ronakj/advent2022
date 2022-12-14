use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Space {
    SAND,
    AIR,
    SOURCE,
    ROCK,
}

fn is_blocked(cave_space: &[[Space; 1000]; 1000], pos: (usize, usize)) -> bool {
    return cave_space[pos.0][pos.1] == Space::ROCK || cave_space[pos.0][pos.1] == Space::SAND;
}

fn put_sand(
    cave_space: &mut [[Space; 1000]; 1000],
    source: (usize, usize),
    max_down: usize,
) -> bool {
    let mut sand_pos = source.clone();
    loop {
        if sand_pos.1 == (max_down + 1) {
            return false;
        }
        if !is_blocked(cave_space, (sand_pos.0, sand_pos.1 + 1)) {
            sand_pos = (sand_pos.0, sand_pos.1 + 1)
        } else if !is_blocked(cave_space, (sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1)
        } else if !is_blocked(cave_space, (sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1)
        } else {
            cave_space[sand_pos.0][sand_pos.1] = Space::SAND;
            return true;
        }
    }
}

// fn print_cave(cave_space: &[[Space; 1000]; 1000]) {
//     for cave_y in 0..15 {
//         for cave_x in 490..510 {
//             if cave_space[cave_x][cave_y] == Space::AIR {
//                 print!(".");
//             }
//             if cave_space[cave_x][cave_y] == Space::ROCK {
//                 print!("#");
//             }
//             if cave_space[cave_x][cave_y] == Space::SOURCE {
//                 print!("+");
//             }
//             if cave_space[cave_x][cave_y] == Space::SAND {
//                 print!("O");
//             }
//         }
//         println!();
//     }
// }

fn get_max_sand_unit(input_file: &str, has_floor: bool) -> usize {
    let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut cave_space: [[Space; 1000]; 1000] = [[Space::AIR; 1000]; 1000];
    cave_space[500][0] = Space::SOURCE;
    let source: (usize, usize) = (500, 0);
    let mut max_down = 0;
    for line in lines {
        let split: Vec<&str> = line.split(" -> ").collect();
        for i in 0..(split.len() - 1) {
            let coords_initial: Vec<usize> = split[i]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let coords_next: Vec<usize> = split[i + 1]
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            if coords_initial[0] == coords_next[0] {
                max_down = max_down.max(coords_next[1].max(coords_initial[1]));
                for i in coords_next[1].min(coords_initial[1])
                    ..(coords_next[1].max(coords_initial[1]) + 1)
                {
                    cave_space[coords_initial[0]][i] = Space::ROCK;
                }
            } else if coords_initial[1] == coords_next[1] {
                max_down = max_down.max(coords_next[1]);
                for i in coords_next[0].min(coords_initial[0])
                    ..(coords_next[0].max(coords_initial[0]) + 1)
                {
                    cave_space[i][coords_initial[1]] = Space::ROCK;
                }
            }
        }
    }
    if has_floor {
        for i in 0..1000 {
            cave_space[i][max_down + 2] = Space::ROCK;
        }
    }
    let mut count: usize = 0;
    loop {
        let result = put_sand(&mut cave_space, source, 998);
        if has_floor && cave_space[500][0] == Space::SAND {
            count += 1;
            break;
        }
        if !result {
            break;
        }
        count += 1;
    }
    return count;
}

fn main() {
    assert_eq!(24, get_max_sand_unit("test_input", false));
    assert_eq!(93, get_max_sand_unit("test_input", true));
    println!("{}", get_max_sand_unit("input", false));
    println!("{}", get_max_sand_unit("input", true));
}
