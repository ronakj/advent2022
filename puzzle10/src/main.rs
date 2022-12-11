use std::fs;

fn main() {
    let file_path = "./input";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let split = contents.split("\n");
    let mut cycles = 0;
    let mut register: i32 = 1;
    let mut crt: Vec<char> = vec!['.'; 240];
    for s in split {
        let previous = cycles;
        if s == "noop" {
            cycles += 1;
        } else {
            cycles += 2;
        }
        for cycle in previous..cycles {
            let crt_pos: i32 = cycle;
            let pixel_pos: i32 = cycle % 40;
            if pixel_pos.abs_diff(register) <= 1 {
                crt[crt_pos as usize] = '#';
            }
        }
        if s.contains("addx") {
            let second: Vec<&str> = s.split(" ").collect();
            let number: i32 = second[1].parse::<i32>().unwrap();
            register += number;
        }
    }
    for i in 0..6 {
        let str: String = crt[i * 40..((i + 1) * 40)].iter().collect();
        println!("{}", str);
    }
}
