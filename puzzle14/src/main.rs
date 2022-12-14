use std::fs;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Space {
    SAND,
    AIR,
    SOURCE,
    ROCK,
}

fn get_max_sand_unit(input_file: &str) -> usize {
    let contents = fs::read_to_string(input_file).expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut cave_space: [[Space; 1000]; 1000] = [[Space::AIR; 1000]; 1000];
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
                for i in coords_next[1].min(coords_initial[1])
                    ..(coords_next[1].max(coords_initial[1]) + 1)
                {
                    cave_space[coords_initial[0]][i] = Space::ROCK;
                }
            } else if coords_initial[1] == coords_next[1] {
                for i in coords_next[0].min(coords_initial[0])
                    ..(coords_next[0].max(coords_initial[0]) + 1)
                {
                    cave_space[i][coords_initial[1]] = Space::ROCK;
                }
            }
        }
    }
    for cave_y in 0..100 {
        for cave_x in 450..550 {
            if cave_space[cave_x][cave_y] == Space::AIR {
                print!(".");
            }
            if cave_space[cave_x][cave_y] == Space::ROCK {
                print!("#");
            }
            if cave_space[cave_x][cave_y] == Space::SOURCE {
                print!("+");
            }
        }
        println!();
    }
    return 0;
}

fn main() {
    get_max_sand_unit("test_input");
}
