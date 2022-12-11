use std::collections::HashMap;
use std::fs;
use std::hash::Hash;

fn main() {
    // --snip--
    let file_path = "./input";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let split = contents.split("\n");
    let mut score = 0;
    #[derive(Debug, PartialEq, Eq, Hash)]
    enum Result {
        WIN,
        DRAW,
        LOSE,
    }
    let outcomes = HashMap::from([
        (
            "A",
            HashMap::from([(Result::DRAW, "X"), (Result::WIN, "Y"), (Result::LOSE, "Z")]),
        ),
        (
            "B",
            HashMap::from([(Result::LOSE, "X"), (Result::DRAW, "Y"), (Result::WIN, "Z")]),
        ),
        (
            "C",
            HashMap::from([(Result::WIN, "X"), (Result::LOSE, "Y"), (Result::DRAW, "Z")]),
        ),
    ]);
    let shape_score = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let outcome_score = HashMap::from([(Result::WIN, 6), (Result::DRAW, 3), (Result::LOSE, 0)]);
    let input_outcome =
        HashMap::from([("X", Result::LOSE), ("Y", Result::DRAW), ("Z", Result::WIN)]);

    for s in split {
        let ch = s.as_bytes();
        let opp = (ch[0] as char).to_string();
        let outcome = input_outcome.get(&*(ch[2] as char).to_string()).unwrap();
        let yours = outcomes.get(&*opp).unwrap().get(&*outcome).unwrap();
        score += shape_score.get(&*yours).unwrap() + outcome_score.get(&*outcome).unwrap();
    }
    print!("{}", score);
}
