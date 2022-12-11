use std::{collections::VecDeque, fs};

#[derive(Debug)]
struct Monkey {
    items: VecDeque<u128>,
    op: String,
    operand_1: String,
    operand_2: String,
    test_divide_by: u128,
    throw_true: usize,
    throw_false: usize,
    items_inspected: u128,
}

fn get_test_input() -> String {
    return fs::read_to_string("./test_input").expect("Should have been able to read the file");
}

fn get_input() -> String {
    return fs::read_to_string("./input").expect("Should have been able to read the file");
}

fn parse_monkey(data: &str) -> Monkey {
    let lines: Vec<&str> = data.split("\n").collect();
    let operand_line: Vec<&str> = lines[2].split(" ").into_iter().collect::<Vec<&str>>();
    Monkey {
        items: lines[1]
            .split(":")
            .last()
            .unwrap()
            .split(", ")
            .into_iter()
            .map(|x| x.trim().parse::<u128>().unwrap())
            .collect(),
        op: operand_line[6].to_string(),
        operand_1: operand_line[5].to_string(),
        operand_2: operand_line[7].to_string(),
        test_divide_by: lines[3]
            .split("by ")
            .into_iter()
            .last()
            .unwrap()
            .parse::<u128>()
            .unwrap(),
        throw_true: lines[4]
            .split("monkey ")
            .into_iter()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        throw_false: lines[5]
            .split("monkey ")
            .into_iter()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap(),
        items_inspected: 0,
    }
}

fn do_one_round(monkeys: &mut Vec<Monkey>, give_relief: bool, remainder_by_product: u128) {
    for monkey_index in 0..monkeys.len() {
        while monkeys[monkey_index].items.len() > 0 {
            let monkey = &mut monkeys[monkey_index];
            let item = monkey.items.pop_front().unwrap();
            let operand_1: u128 = if monkey.operand_1 == "old" {
                item.clone()
            } else {
                monkey.operand_1.parse::<u128>().unwrap()
            };
            let operand_2: u128 = if monkey.operand_2 == "old" {
                item.clone()
            } else {
                monkey.operand_2.parse::<u128>().unwrap()
            };
            let mut new_worry: u128 = if monkey.op == "*" {
                (operand_1 * operand_2) % remainder_by_product
            } else {
                (operand_1 + operand_2) % remainder_by_product
            };
            if give_relief {
                new_worry = new_worry / 3;
            }
            let new_monkey_index: usize;
            if new_worry % monkey.test_divide_by == 0 {
                new_monkey_index = monkey.throw_true;
            } else {
                new_monkey_index = monkey.throw_false;
            }
            monkey.items_inspected += 1;
            drop(monkey);
            monkeys[new_monkey_index].items.push_back(new_worry);
        }
    }
}

fn calculate_monkey_business(input: String, give_relief: bool) -> u128 {
    let mut monkeys: Vec<Monkey> = Vec::with_capacity(10);
    let monkey_str = input.split("\n\n");
    for monkey_data in monkey_str {
        let monkey = parse_monkey(monkey_data);
        monkeys.push(monkey);
    }

    let remainder_product = monkeys.iter().map(|m| m.test_divide_by).product::<u128>();

    if give_relief {
        for _ in 0..20 {
            do_one_round(&mut monkeys, give_relief, remainder_product);
        }
    } else {
        for _ in 0..10000 {
            do_one_round(&mut monkeys, give_relief, remainder_product);
        }
    }

    let mut item_inspected_counts = monkeys
        .iter()
        .map(|monkey| monkey.items_inspected)
        .collect::<Vec<_>>();

    item_inspected_counts.sort();

    let most = item_inspected_counts.pop().unwrap();
    let second_most = item_inspected_counts.pop().unwrap();

    most * second_most
}

fn main() {
    let test_output = calculate_monkey_business(get_test_input(), true);
    assert_eq!(test_output, 10605);

    println!(
        "with relief {}",
        calculate_monkey_business(get_input(), true)
    );

    let test_output = calculate_monkey_business(get_test_input(), false);
    assert_eq!(test_output, 2713310158);

    println!(
        "without relief {}",
        calculate_monkey_business(get_input(), false)
    );
}
